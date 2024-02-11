pub mod cli;
mod core;
mod matching_results;

pub use crate::{
    core::{
        exit_code::ExitCode,
        request::{ContextSize, Lines, MatchOptions, OutputBehavior, Request, Targets},
    },
    matching_results::result::MatchingResult,
};

use crate::{
    cli::output,
    core::{reader::Reader, request::MatchCollectionStrategy},
    matching_results::{
        context_accumulators::SlidingAccumulator,
        result::{MatchingResultState, PartialMatchingResult},
        result_collection::ResultCollection,
        top_bracket::TopBracket,
    },
};
use log::debug;
use std::{
    collections::VecDeque,
    error,
    io::{self, BufRead, Write},
    iter, mem,
    path::Path,
};
use walkdir::WalkDir;

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
    request: &Request,
    output_dest: &mut impl Write,
) -> Result<Vec<MatchingResult>, Box<dyn error::Error>> {
    debug!("Running with the following configuration: {:?}", request);

    let results = match request.strategy {
        MatchCollectionStrategy::CollectAll => {
            collect_all_matches(&request.query, &request.targets, &request.match_options)
        }
        MatchCollectionStrategy::CollectTop(n) => {
            collect_top_matches(&request.query, &request.targets, &request.match_options, n)
        }
    }?;

    match request.output_behavior {
        OutputBehavior::Normal(formatting) => {
            write!(
                output_dest,
                "{}",
                output::format_results(&results, &formatting)
            )?;
        }
        OutputBehavior::Quiet => {}
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
pub fn collect_all_matches(
    query: &str,
    targets: &Targets,
    options: &MatchOptions,
) -> Result<Vec<MatchingResult>, Box<dyn error::Error>> {
    let mut result = Vec::new();
    collect_matches_common(query, targets, options, &mut result)?;
    result.sort_by(|a, b| b.cmp(a));
    Ok(result)
}

/// Same as [`collect_all_matches`] but collects only a given number of matches with the highest score.
///
/// # Errors
///
///   * [`io::Error`] if encounters any I/O related issues.
///   * [`walkdir::Error`] if any errors related to recursive processing occur
///
pub fn collect_top_matches(
    query: &str,
    targets: &Targets,
    options: &MatchOptions,
    top: usize,
) -> Result<Vec<MatchingResult>, Box<dyn error::Error>> {
    let mut result = TopBracket::new(top);
    collect_matches_common(query, targets, options, &mut result)?;
    Ok(result.into_vec())
}

fn collect_matches_common<T: ResultCollection>(
    query: &str,
    targets: &Targets,
    options: &MatchOptions,
    dest: &mut T,
) -> Result<(), Box<dyn error::Error>>{
    for reader in make_readers(targets) {
        let reader = reader?;
        debug!("Processing {}.", reader.display_name());
        merge_target_matches(query, reader, options, dest)?;
    }
    Ok(())
}

fn merge_target_matches<T: ResultCollection>(
    query: &str,
    target: Reader,
    options: &MatchOptions,
    dest: &mut T,
) -> Result<(), io::Error> {
    let display_name = target.display_name().clone();
    let ContextSize {
        before: Lines(lines_before),
        after: Lines(lines_after),
    } = options.context_size;
    let mut context_before = SlidingAccumulator::new(lines_before);
    let mut pending_results: VecDeque<PartialMatchingResult> = VecDeque::new();
    for (index, line) in target.into_source().lines().enumerate() {
        let line = line?;

        // Feed the current line to the results that are waiting for their post-contexts to fill up (if there are any).
        for partial_result in mem::take(&mut pending_results) {
            match partial_result.feed(line.clone()) {
                MatchingResultState::Complete(matching_result) => dest.push(matching_result),
                MatchingResultState::Incomplete(partial_matching_result) => {
                    pending_results.push_back(partial_matching_result)
                }
            }
        }

        if let Some(m) = vscode_fuzzy_score_rs::fuzzy_match(query, &line) {
            let line_number = index + 1;
            debug!(
                "Found a match in {display_name}, line {line_number}, positions {:?}",
                m.positions()
            );

            match MatchingResultState::new(
                line.clone(),
                m,
                options.track_file_names.then_some(display_name.clone()),
                options.track_line_numbers.then_some(line_number),
                context_before.snapshot(),
                lines_after,
            ) {
                MatchingResultState::Complete(matching_result) => dest.push(matching_result),
                MatchingResultState::Incomplete(partial_matching_result) => {
                    pending_results.push_back(partial_matching_result)
                }
            }
        }

        context_before.feed(line);
    }

    // It is possible that the end of the file was reached when some matches were still waiting
    // for their post-context to fill up. In such case we just add what we have to `result`.
    for partial_result in pending_results {
        dest.push(partial_result.complete());
    }

    Ok(())
}

fn make_readers(
    targets: &Targets,
) -> Box<dyn Iterator<Item = Result<Reader, Box<dyn error::Error>>> + '_> {
    match targets {
        Targets::Files(files) => {
            debug!(
                "*Non*-recursive mode; using the following input files: {:?}",
                files
            );
            Box::new(
                files
                    .iter()
                    .map(|p| Reader::file_reader(p).map_err(|e| e.into())),
            )
        }
        Targets::RecursiveEntries(entries) => {
            debug!(
                "Recursive mode; using the following input targets: {:?}",
                entries
            );
            make_recursive_reader_iterator(entries.iter())
        }
        Targets::Stdin => {
            debug!("*Non*-recursive mode; using STDIN.");
            Box::new(iter::once(Ok(Reader::stdin_reader())))
        }
    }
}

fn make_recursive_reader_iterator<'item>(
    targets: impl Iterator<Item = impl AsRef<Path> + 'item> + 'item,
) -> Box<dyn Iterator<Item = Result<Reader, Box<dyn error::Error>>> + 'item> {
    Box::new(targets.flat_map(WalkDir::new).filter_map(|item| {
        item.map_or_else(
            |e| Some(Err(e.into())),
            |d| {
                d.metadata().map_or_else(
                    |e| Some(Err(e.into())),
                    |m| {
                        m.is_file()
                            .then_some(Reader::file_reader(d.path()).map_err(|e| e.into()))
                    },
                )
            },
        )
    }))
}
