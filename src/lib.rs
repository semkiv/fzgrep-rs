mod cli;
mod core;
mod matching_results;

pub use core::request::Request;

use crate::{
    cli::output,
    core::{
        reader::Reader,
        request::{ContextSize, Lines, MatchOptions, OutputBehavior, Targets},
    },
    matching_results::{
        context_accumulators::SlidingAccumulator,
        result::{MatchingResult, MatchingResultState, PartialMatchingResult},
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
    let matches = find_matches(&request.query, &request.targets, &request.match_options)?;
    match request.output_behavior {
        OutputBehavior::Normal(formatting) => {
            write!(
                output_dest,
                "{}",
                output::format_results(&matches, &formatting)
            )?;
        }
        OutputBehavior::Quiet => {}
    }
    Ok(matches)
}

/// Find fuzzy matches using the configuration supplied `request`.
/// If there are no input files in the request, the standard input will be used.
///
/// # Errors
///
///   * [`io::Error`] if encounters any I/O related issues.
///   * [`walkdir::Error`] if any errors related to recursive processing occur
///
pub(crate) fn find_matches(
    query: &str,
    targets: &Targets,
    options: &MatchOptions,
) -> Result<Vec<MatchingResult>, Box<dyn error::Error>> {
    let mut matches = Vec::new();
    for reader in make_readers(targets) {
        let reader = reader?;
        debug!("Processing {}.", reader.display_name());
        matches.append(&mut process_one_target(query, reader, options)?);
    }

    // sort in descending order
    matches.sort_by(|a, b| b.cmp(a));

    Ok(matches)
}

const fn make_readers(
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

const fn make_recursive_reader_iterator<'item>(
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

fn process_one_target(
    query: &str,
    target: Reader,
    options: &MatchOptions,
) -> Result<Vec<MatchingResult>, io::Error> {
    let display_name = target.display_name().clone();
    let mut result = Vec::new();

    let ContextSize {
        before: Lines(lines_before),
        after: Lines(lines_after),
    } = options.context_size;
    let mut context_before = SlidingAccumulator::new(lines_before);
    let mut pending_results: VecDeque<PartialMatchingResult> = VecDeque::new();
    for (index, line) in target.source().lines().enumerate() {
        let line = line?;
        context_before.feed(line);

        // Feed the current line to the results that are waiting for their post-contexts to fill up (if there are any).
        for partial_result in mem::take(&mut pending_results) {
            match partial_result.feed(line) {
                MatchingResultState::Complete(matching_result) => result.push(matching_result),
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
                line,
                m,
                options.track_file_names.then_some(display_name),
                options.track_line_numbers.then_some(line_number),
                context_before.snapshot(),
                lines_after,
            ) {
                MatchingResultState::Complete(matching_result) => result.push(matching_result),
                MatchingResultState::Incomplete(partial_matching_result) => {
                    pending_results.push_back(partial_matching_result)
                }
            }
        }
    }

    // It is possible that the end of the file was reached when some matches were still waiting
    // for their post-context to fill up. In such case we just add what we have to `result`.
    for partial_result in pending_results {
        result.push(partial_result.complete());
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    #[test]
    fn results_sorting() {
        let mut results = vec![
            MatchingLine {
                location: Location {
                    file_name: String::from("Third"),
                    line_number: 13,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
            },
            MatchingLine {
                location: Location {
                    file_name: String::from("First"),
                    line_number: 100500,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
            },
            MatchingLine {
                location: Location {
                    file_name: String::from("Second"),
                    line_number: 42,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
            },
        ];

        let expected = vec![
            MatchingLine {
                location: Location {
                    file_name: String::from("First"),
                    line_number: 100500,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
            },
            MatchingLine {
                location: Location {
                    file_name: String::from("Second"),
                    line_number: 42,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
            },
            MatchingLine {
                location: Location {
                    file_name: String::from("Third"),
                    line_number: 13,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
            },
        ];

        results.sort();
        assert_eq!(results, expected);
    }
}
