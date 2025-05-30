pub mod cli;
pub mod exit_code;
pub mod match_properties;
pub mod request;

mod core;

use cli::output;
use cli::output::behavior::Behavior;
use cli::request::Request as CliRequest;
use core::Reader;
use core::prospective::MatchProperties as ProspectiveMatchProperties;
use core::results_collection::ResultsCollection;
use core::results_collection::top_bracket::TopBracket;
use core::sliding_accumulator::SlidingAccumulator;
use match_properties::MatchProperties;
use match_properties::location::Location;
use request::Request as CoreRequest;
use request::collection_strategy::CollectionStrategy;
use request::match_options::{LineNumberTracking, MatchOptions, SourceNameTracking};
use request::targets::Targets;
use request::targets::filter::Filter;

use log::debug;
use std::collections::VecDeque;
use std::io::{BufRead as _, Write};
use std::path::Path;
use std::{error, iter};
use walkdir::{DirEntry, WalkDir};

/// This function handles all the application logic.
///
/// The `main` function is merely a `run` call.
///
/// The configuration is passed in `request`.
/// If no input files are specified in `request`, the standard input is used.
///
/// # Errors
///
///   * [`std::fmt::Error`] if encounters any formatting related issues.
///   * [`std::io::Error`] if encounters any I/O related issues.
///   * [`walkdir::Error`] if any errors related to recursive processing occur
///
pub fn run(
    cli_request: &CliRequest,
    output_dest: &mut impl Write,
) -> Result<Vec<MatchProperties>, Box<dyn error::Error>> {
    debug!("Running with the following configuration: {cli_request:?}");

    let results = collect_matches(&cli_request.core)?;

    match cli_request.output_behavior {
        Behavior::Normal(formatting) => {
            write!(
                output_dest,
                "{}",
                output::format_results(&results, &formatting)
            )?;
        }
        Behavior::Quiet => {}
    }

    Ok(results)
}

/// Find fuzzy matches of `query` in `targets` using the configuration supplied `options`.
///
/// # Errors
///
///   * [`io::Error`] if encounters any I/O related issues.
///   * [`walkdir::Error`] if any errors related to recursive processing occur
///
pub fn collect_matches(
    request: &CoreRequest,
) -> Result<Vec<MatchProperties>, Box<dyn error::Error>> {
    match request.collection_strategy {
        CollectionStrategy::CollectAll => {
            let mut vec = Vec::new();
            collect_matches_into(
                &request.query,
                &request.targets,
                &request.match_options,
                &mut vec,
            )?;
            Ok(vec.into_sorted_vec())
        }
        CollectionStrategy::CollectTop(n) => {
            let mut top_bracket = TopBracket::new(n);
            collect_matches_into(
                &request.query,
                &request.targets,
                &request.match_options,
                &mut top_bracket,
            )?;
            Ok(top_bracket.into_sorted_vec())
        }
    }
}

// TODO: make this take core::Request?
fn collect_matches_into(
    query: &str,
    targets: &Targets,
    options: &MatchOptions,
    dest: &mut impl ResultsCollection,
) -> Result<(), Box<dyn error::Error>> {
    for reader in make_readers(targets) {
        let reader = reader?;
        debug!("Processing {}.", reader.display_name());
        process_reader(reader, query, options, dest)?;
    }
    Ok(())
}

#[expect(
    clippy::panic_in_result_fn,
    reason = "There are a couple of cases of logic (i.e. programmer's) errors"
)]
fn process_reader(
    reader: Reader,
    query: &str,
    options: &MatchOptions,
    dest: &mut impl ResultsCollection,
) -> Result<(), Box<dyn error::Error>> {
    let display_name = reader.display_name().clone();

    let context_size = options.context_size;
    let lines_after = context_size.lines_after;
    let mut before_context_accumulator = SlidingAccumulator::new(context_size.lines_before);

    // `Option` is purely for technical purposes: `ProspectiveMatchProperties::update` method consumes the item
    // so there must be something place in its stead. Having `Option` just makes things easiser
    // (an alternative could be storing items and using `std::mem::take` on the item being updated,
    // but this requires the items to be dafault-constructible as it leaves a default-constructed item instead,
    // which, in addition and generally speaking, might not be cheap)
    let mut pending_results =
        VecDeque::<Option<ProspectiveMatchProperties>>::with_capacity(lines_after);

    for (index, line) in reader.into_source().lines().enumerate() {
        let line = line?;
        let line_number = index.wrapping_add(1);

        for prospective_props in &mut pending_results {
            #[expect(
                clippy::expect_used,
                reason = "It is a logic error if an actual \"hole\" is found among the pending results.\
                          They appear only temporary while the actual value is taken for updating."
            )]
            let current = prospective_props
                .take()
                .expect("Found a \"hole\" when processing current pending match results");
            *prospective_props = Some(current.update(line.clone()));
        }

        // Since the input is processed line-by-line, at most one pending match can be completed.
        // Since the pending matches are added in the order they appear in the input
        // and are removed upon completion (see below) it's only the first pending match
        // that can become complete at a given step.
        let first = pending_results.pop_front();
        // If there's anything pending at all...
        if let Some(props) = first {
            match props {
                // This is purely technical, see the note about using `Option` above.
                Some(props) => match props {
                    // The pending match is now complete, move it the results collection.
                    ProspectiveMatchProperties::Ready(props) => dest.add(props),
                    // The pending match still lacks some context, put it back into the queue.
                    ProspectiveMatchProperties::Pending { .. } => {
                        pending_results.push_front(Some(props));
                    }
                },
                #[expect(
                    clippy::panic,
                    reason = "It is a logic error if an actual \"hole\" is found among the pending results.\
                              They appear only temporary while the actual value is taken for updating."
                )]
                None => {
                    panic!("Found a \"hole\" when processing pending results")
                }
            }
        }

        if let Some(fuzzy_match) = vscode_fuzzy_score_rs::fuzzy_match(query, &line) {
            let positions = fuzzy_match.positions().clone();

            let match_location = Location {
                source_name: (options.source_name_tracking == SourceNameTracking::On)
                    .then(|| display_name.clone()),
                line_number: (options.line_number_tracking == LineNumberTracking::On)
                    .then_some(line_number),
            };
            let prospective_props = ProspectiveMatchProperties::new(
                line.clone(),
                fuzzy_match,
                match_location,
                before_context_accumulator.snapshot(),
                lines_after,
            );

            match prospective_props {
                ProspectiveMatchProperties::Ready(props) => dest.add(props),
                ProspectiveMatchProperties::Pending { .. } => {
                    pending_results.push_back(Some(prospective_props));
                }
            }

            debug!("Found a match in {display_name}, line {line_number}, positions {positions:?}");
        }

        before_context_accumulator.feed(line);
    }

    for props in pending_results {
        match props {
            Some(props) => dest.add(props.complete()),
            #[expect(
                clippy::panic,
                reason = "It is a logic error if an actual \"hole\" is found among the pending results.\
                          They appear only temporary while the actual value is taken for updating."
            )]
            None => panic!("Found a \"hole\" when completing remaining pending results"),
        }
    }

    Ok(())
}

fn make_readers(
    targets: &Targets,
) -> Box<dyn Iterator<Item = Result<Reader, Box<dyn error::Error>>> + '_> {
    match targets {
        Targets::Files(files) => {
            debug!("*Non*-recursive mode; using the following input files: {files:?}");
            Box::new(
                #[expect(
                    clippy::redundant_closure_for_method_calls,
                    reason = "`|e| e.into()` is arguably more concise than `std::convert::Into::into`"
                )]
                files
                    .iter()
                    .map(|path| Reader::file_reader(path).map_err(|err| err.into())),
            )
        }
        Targets::RecursiveEntries { paths, filter } => {
            debug!("Recursive mode; using the following input targets: {paths:?}");
            debug!(
                "File filter{}",
                filter.as_ref().map_or_else(
                    || String::from(" not set"),
                    |filter| format!(": {filter:?}")
                )
            );
            make_recursive_reader_iterator(paths.iter(), filter.as_ref())
        }
        Targets::Stdin => {
            debug!("*Non*-recursive mode; using STDIN.");
            Box::new(iter::once(Ok(Reader::stdin_reader())))
        }
    }
}

fn make_recursive_reader_iterator<'item>(
    targets: impl Iterator<Item = impl AsRef<Path> + 'item> + 'item,
    filter: Option<&'item Filter>,
) -> Box<dyn Iterator<Item = Result<Reader, Box<dyn error::Error>>> + 'item> {
    let is_not_disallowed = move |dir_entry: &DirEntry| {
        filter.is_none_or(|filter| !filter.is_disallowed_by_exclude(dir_entry.path()))
    };
    let is_allowed = move |dir_entry: &DirEntry| {
        filter.is_none_or(|filter| filter.is_allowed_by_include(dir_entry.path()))
    };

    Box::new(
        targets
            .flat_map(move |target| WalkDir::new(target).sort_by_file_name().into_iter().filter_entry(is_not_disallowed))
            .filter_map(move |item| {
                item.map_or_else(
                    |err| Some(Err(err.into())),
                    |entry| {
                        if !is_allowed(&entry) {
                            return None;
                        }

                        entry.metadata().map_or_else(
                            |err| Some(Err(err.into())),
                            |metadata| {
                                #[expect(
                    clippy::redundant_closure_for_method_calls,
                    reason = "`|e| e.into()` is arguably more concise than `std::convert::Into::into`"
                )]
                                metadata.is_file()
                                    .then_some(Reader::file_reader(entry.path()).map_err(|err| err.into()))
                            },
                        )
                    },
                )
            }),
    )
}
