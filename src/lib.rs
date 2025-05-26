pub mod cli;
pub mod exit_code;
pub mod match_properties;
pub mod request;

mod core;

use cli::output;
use cli::output::behavior::Behavior;
use cli::request::Request as CliRequest;
use core::context_accumulators::SlidingAccumulator;
use core::prospective_match_properties::ProspectiveMatchProperties;
use core::reader::Reader;
use core::results_collection::ResultsCollection;
use core::top_bracket::TopBracket;
use log::debug;
use match_properties::MatchProperties;
use match_properties::location::Location;
use request::Request as CoreRequest;
use request::collection_strategy::CollectionStrategy;
use request::match_options::{LineNumberTracking, MatchOptions, SourceNameTracking};
use request::targets::Targets;
use request::targets::filter::Filter;
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
pub fn collect_matches(request: &CoreRequest) -> Result<Vec<MatchProperties>, Box<dyn error::Error>> {
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
    let mut pending_results =
        VecDeque::<Option<ProspectiveMatchProperties>>::with_capacity(lines_after);

    for (index, line) in reader.into_source().lines().enumerate() {
        let line = line?;
        let line_number = index.wrapping_add(1);

        for prospective_props in &mut pending_results {
            *prospective_props = Some(prospective_props.take().unwrap().update(line.clone()));
        }

        // TODO: clarify
        let first = pending_results.pop_front();
        if let Some(props) = first {
            match props {
                Some(props) => match props {
                    ProspectiveMatchProperties::Ready(props) => dest.add(props),
                    ProspectiveMatchProperties::Pending { .. } => {
                        pending_results.push_front(Some(props));
                    }
                },
                None => unreachable!("There should not be any \"holes\" at this point"),
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
            None => unreachable!("There should be no \"holes\" at this point"),
        }
    }

    Ok(())
}

// // TODO: refactor common pieces
// // TODO: make temporary result less ad-hoc

// fn process_target_no_context(
//     target: Reader,
//     query: &str,
//     line_number_tracking: LineNumberTracking,
//     source_name_tracking: SourceNameTracking,
//     dest: &mut impl ResultsCollection,
// ) -> Result<(), io::Error> {
//     let display_name = target.display_name().clone();
//     let reported_display_name =
//         (source_name_tracking == SourceNameTracking::On).then(|| display_name.clone());

//     for (index, line) in target.into_source().lines().enumerate() {
//         let line = line?;
//         let line_number = index.wrapping_add(1);
//         let reported_line_number =
//             (line_number_tracking == LineNumberTracking::On).then_some(line_number);

//         if let Some(fuzzy_match) = vscode_fuzzy_score_rs::fuzzy_match(query, &line) {
//             let positions = fuzzy_match.positions().clone();
//             dest.add(MatchProperties {
//                 matching_line: line,
//                 fuzzy_match,
//                 location: Location {
//                     source_name: reported_display_name.clone(),
//                     line_number: reported_line_number,
//                 },
//                 context: Context {
//                     before: None,
//                     after: None,
//                 },
//             });
//             debug!("Found a match in {display_name}, line {line_number}, positions {positions:?}");
//         }
//     }

//     Ok(())
// }

// fn process_target_before_context(
//     target: Reader,
//     query: &str,
//     line_number_tracking: LineNumberTracking,
//     source_name_tracking: SourceNameTracking,
//     lines_before: usize,
//     dest: &mut impl ResultsCollection,
// ) -> Result<(), io::Error> {
//     let display_name = target.display_name().clone();
//     let reported_display_name =
//         (source_name_tracking == SourceNameTracking::On).then(|| display_name.clone());

//     let mut before_context = VecDeque::with_capacity(lines_before);

//     for (index, line) in target.into_source().lines().enumerate() {
//         let line = line?;
//         let line_number = index.wrapping_add(1);
//         let reported_line_number =
//             (line_number_tracking == LineNumberTracking::On).then_some(line_number);

//         if let Some(fuzzy_match) = vscode_fuzzy_score_rs::fuzzy_match(query, &line) {
//             let positions = fuzzy_match.positions().clone();
//             dest.add(MatchProperties {
//                 matching_line: line.clone(),
//                 fuzzy_match,
//                 location: Location {
//                     source_name: reported_display_name.clone(),
//                     line_number: reported_line_number,
//                 },
//                 context: Context {
//                     before: Some(Vec::from(before_context.clone())),
//                     after: None,
//                 },
//             });
//             debug!("Found a match in {display_name}, line {line_number}, positions {positions:?}");
//         }

//         if before_context.len() == lines_before {
//             before_context.pop_front();
//         }
//         before_context.push_back(line);
//     }

//     Ok(())
// }

// fn process_target_after_context(
//     target: Reader,
//     query: &str,
//     line_number_tracking: LineNumberTracking,
//     source_name_tracking: SourceNameTracking,
//     lines_after: usize,
//     dest: &mut impl ResultsCollection,
// ) -> Result<(), io::Error> {
//     let display_name = target.display_name().clone();
//     let reported_display_name =
//         (source_name_tracking == SourceNameTracking::On).then(|| display_name.clone());

//     let mut pending_results = VecDeque::<(MatchProperties, usize)>::with_capacity(lines_after);

//     for (index, line) in target.into_source().lines().enumerate() {
//         let line = line?;
//         let line_number = index.wrapping_add(1);
//         let reported_line_number =
//             (line_number_tracking == LineNumberTracking::On).then_some(line_number);

//         for (props, missing) in &mut pending_results {
//             props
//                 .context
//                 .after
//                 .as_mut()
//                 .expect("Encountered a partial result without a context. This is a bug.")
//                 .push(line.clone());
//             *missing -= 1;
//         }

//         while let Some((_, 0)) = pending_results.front() {
//             let (props, _) = pending_results.pop_front().unwrap();
//             dest.add(props);
//         }

//         if let Some(fuzzy_match) = vscode_fuzzy_score_rs::fuzzy_match(query, &line) {
//             let positions = fuzzy_match.positions().clone();

//             pending_results.push_back((
//                 MatchProperties {
//                     matching_line: line.clone(),
//                     fuzzy_match,
//                     location: Location {
//                         source_name: reported_display_name.clone(),
//                         line_number: reported_line_number,
//                     },
//                     context: Context {
//                         before: None,
//                         after: Some(Vec::with_capacity(lines_after)),
//                     },
//                 },
//                 lines_after,
//             ));

//             debug!("Found a match in {display_name}, line {line_number}, positions {positions:?}");
//         }
//     }

//     for (props, _) in pending_results {
//         dest.add(props);
//     }

//     Ok(())
// }

// fn process_target_full_context(
//     target: Reader,
//     query: &str,
//     line_number_tracking: LineNumberTracking,
//     source_name_tracking: SourceNameTracking,
//     lines_before: usize,
//     lines_after: usize,
//     dest: &mut impl ResultsCollection,
// ) -> Result<(), io::Error> {
//     let display_name = target.display_name().clone();
//     let reported_display_name =
//         (source_name_tracking == SourceNameTracking::On).then(|| display_name.clone());

//     let mut before_context = VecDeque::with_capacity(lines_before);
//     let mut pending_results = VecDeque::<(MatchProperties, usize)>::with_capacity(lines_after);

//     for (index, line) in target.into_source().lines().enumerate() {
//         let line = line?;
//         let line_number = index.wrapping_add(1);
//         let reported_line_number =
//             (line_number_tracking == LineNumberTracking::On).then_some(line_number);

//         for (props, missing) in &mut pending_results {
//             props
//                 .context
//                 .after
//                 .as_mut()
//                 .expect("Encountered a partial result without a context. This is a bug.")
//                 .push(line.clone());
//             *missing -= 1;
//         }

//         while let Some((_, 0)) = pending_results.front() {
//             let (props, _) = pending_results.pop_front().unwrap();
//             dest.add(props);
//         }

//         if let Some(fuzzy_match) = vscode_fuzzy_score_rs::fuzzy_match(query, &line) {
//             let positions = fuzzy_match.positions().clone();

//             pending_results.push_back((
//                 MatchProperties {
//                     matching_line: line.clone(),
//                     fuzzy_match,
//                     location: Location {
//                         source_name: reported_display_name.clone(),
//                         line_number: reported_line_number,
//                     },
//                     context: Context {
//                         before: Some(Vec::from(before_context.clone())),
//                         after: Some(Vec::with_capacity(lines_after)),
//                     },
//                 },
//                 lines_after,
//             ));

//             debug!("Found a match in {display_name}, line {line_number}, positions {positions:?}");
//         }

//         if before_context.len() == lines_before {
//             before_context.pop_front();
//         }
//         before_context.push_back(line);
//     }

//     for (props, _) in pending_results {
//         dest.add(props);
//     }

//     Ok(())
// }

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
    let exclude_pred = move |dir_entry: &DirEntry| {
        filter.is_some_and(|filter| filter.matches_exclude(dir_entry.path()))
    };
    let include_pred = move |dir_entry: &DirEntry| {
        filter.is_some_and(|filter| filter.matches_include(dir_entry.path()))
    };

    Box::new(
        targets
            .flat_map(move |target| WalkDir::new(target).sort_by_file_name().into_iter().filter_entry(exclude_pred))
            .filter_map(move |item| {
                item.map_or_else(
                    |err| Some(Err(err.into())),
                    |entry| {
                        if include_pred(&entry) {
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
