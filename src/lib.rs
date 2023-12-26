mod cli;
mod core;
mod matching_results;

pub use core::request::Request;

use cli::output;
use core::{
    reader::Reader,
    request::{MatchOptions, OutputBehavior, Targets},
};
use log::debug;
use matching_results::result::MatchingResult;
use std::{
    error,
    io::{self, BufRead, Write},
    iter,
    path::Path,
};
use walkdir::WalkDir;

// use std::{
//     env
//     io::BufRead},
//     path::PathBuf,
// };

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
    let mut ret = Vec::new();
    for (index, line) in target.source().lines().enumerate() {
        let line = line?;
        if let Some(m) = vscode_fuzzy_score_rs::fuzzy_match(query, &line) {
            let line_number = index + 1;
            debug!(
                "Found a match in {display_name}, line {line_number}, positions {:?}",
                m.positions()
            );
            ret.push(MatchingResult {
                location: Location {
                    file_name: display_name.clone(),
                    line_number,
                },
                content: line,
                fuzzy_match: m,
            });
        }
    }

    Ok(ret)
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
