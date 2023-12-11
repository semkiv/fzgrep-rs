mod cli;
mod core;
mod matching_results;

pub use cli::{OutputOptions, Request};
pub use core::exit_code::ExitCode;

use core::reader::Reader;
use log::debug;
use matching_results::matching_line::{Location, MatchingLine};
use std::{
    env, error,
    io::{self, BufRead},
    iter,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;
use yansi::Paint;

/// This function handles all the application logic.
///
/// The `main` function is merely a `run` call.
///
/// The configuration is passed in `request`.
/// If no input files are specified in `request`, the standard input is used.
///
/// # Errors
///
///   * [`std::io::Error`] if encounters any I/O related issues.
///   * [`walkdir::error::Error`] if any errors related to recursive processing occur
///
pub fn run(request: &Request) -> Result<Vec<MatchingLine>, Box<dyn error::Error>> {
    debug!("Running with the following configuration: {:?}", request);
    let matches = find_matches(request.query(), request.targets(), request.recursive())?;
    if !request.quiet() && !matches.is_empty() {
        println!(
            "{}",
            format_results(&matches, &request.output_options())
        );
    }
    Ok(matches)
}

/// Find fuzzy matches using the configuration supplied `request`.
/// If there are no input files in the request, the standard input will be used.
///
/// # Errors
///
///   * [`io::Error`] if encounters any I/O related issues.
///   * [`walkdir::error::Error`] if any errors related to recursive processing occur
///
pub fn find_matches(
    query: &str,
    targets: &Option<Vec<PathBuf>>,
    recursive: bool,
) -> Result<Vec<MatchingLine>, Box<dyn error::Error>> {
    let mut matches = Vec::new();
    for reader in make_readers(targets, recursive) {
        let reader = reader?;
        debug!("Processing {}.", reader.display_name());
        matches.append(&mut process_one_target(query, reader)?);
    }

    // sort in descending order
    matches.sort_by(|a, b| b.cmp(a));

    Ok(matches)
}

/// Formats supplied `matches` into a rich text string.
///
/// When grepping files the format is as follows:
/// ```text
/// <filename>:<line-number>:<colored-matching-line>
/// ```
/// where `colored-matching-line` is a matching line with matching characters painted blue.
/// Whether `<filename>` and `<line-number>` are printed depends on `options`.
///
pub fn format_results(matches: &[MatchingLine], options: &OutputOptions) -> String {
    let mut ret = String::new();
    let mut match_itr = matches.iter().peekable();
    while let Some(m) = match_itr.next() {
        let MatchingLine {
            location:
                Location {
                    file_name,
                    line_number,
                },
            content,
            fuzzy_match,
        } = m;

        let mut colored_target = String::new();
        let mut matches_it = fuzzy_match.positions().iter().peekable();
        for (index, ch) in content.chars().enumerate() {
            if matches_it.peek().is_some_and(|pos| **pos == index) {
                colored_target.push_str(&Paint::blue(ch).to_string());
                matches_it.next();
            } else {
                colored_target.push(ch);
            }
        }

        if options.file_name {
            ret.push_str(&format!("{file_name}:"));
        }
        if options.line_number {
            ret.push_str(&format!("{line_number}:"));
        }

        ret.push_str(&colored_target);

        if match_itr.peek().is_some() {
            ret.push('\n');
        }
    }

    ret
}

fn make_readers(
    targets: &Option<Vec<PathBuf>>,
    recursive: bool,
) -> Box<dyn Iterator<Item = Result<Reader, Box<dyn error::Error>>> + '_> {
    if !recursive {
        // In non-recursive mode we simply create a `Reader` for each of the specified targets
        // (which are expected to be files in this case).
        // That is if we have any, otherwise we use the standard input
        if let Some(targets) = targets {
            debug!(
                "*Non*-recursive mode; using the following input files: {:?}",
                targets
            );
            Box::new(
                targets
                    .iter()
                    .map(|p| Reader::file_reader(p).map_err(|e| e.into())),
            )
        } else {
            debug!("*Non*-recursive mode; no input files specified => using STDIN.");
            Box::new(iter::once(Ok(Reader::stdin_reader())))
        }
    } else {
        // In recursive mode on the other hand we have to account for the fact
        // that targets can potentially be directories, so we have to process each of them recursively.
        // Again, that is if we have any, otherwise we use the current working directory.
        if let Some(targets) = targets {
            debug!(
                "Recursive mode; using the following input targets: {:?}",
                targets
            );
            make_recursive_reader_iterator(targets.iter())
        } else {
            debug!("Recursive mode; no input files specified => using CWD.");
            make_recursive_cwd_reader_iterator()
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

fn make_recursive_cwd_reader_iterator(
) -> Box<dyn Iterator<Item = Result<Reader, Box<dyn error::Error>>>> {
    env::current_dir()
        .map_or_else::<Box<dyn Iterator<Item = Result<Reader, Box<dyn error::Error>>>>, _, _>(
            |e| Box::new(iter::once(Err(e.into()))),
            |cwd| make_recursive_reader_iterator(iter::once(cwd)),
        )
}

fn process_one_target(query: &str, target: Reader) -> Result<Vec<MatchingLine>, io::Error> {
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
            ret.push(MatchingLine {
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
    use super::*;

    #[test]
    fn results_output_options_default() {
        let results = vec![
            MatchingLine {
                location: Location {
                    file_name: String::from("First"),
                    line_number: 42,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
            },
            MatchingLine {
                location: Location {
                    file_name: String::from("Second"),
                    line_number: 100500,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
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
        assert_eq!(
            format_results(&results, &OutputOptions::default()),
            format!(
                "{}{}st\ntes{}\n{}{}s{}",
                Paint::blue('t'),
                Paint::blue('e'),
                Paint::blue('t'),
                Paint::blue('t'),
                Paint::blue('e'),
                Paint::blue('t')
            )
        )
    }

    #[test]
    fn results_output_options_line_number() {
        let results = vec![
            MatchingLine {
                location: Location {
                    file_name: String::from("First"),
                    line_number: 42,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
            },
            MatchingLine {
                location: Location {
                    file_name: String::from("Second"),
                    line_number: 100500,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
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
        assert_eq!(
            format_results(
                &results,
                &OutputOptions {
                    line_number: true,
                    ..Default::default()
                }
            ),
            format!(
                "42:{}{}st\n100500:tes{}\n13:{}{}s{}",
                Paint::blue('t'),
                Paint::blue('e'),
                Paint::blue('t'),
                Paint::blue('t'),
                Paint::blue('e'),
                Paint::blue('t')
            )
        )
    }

    #[test]
    fn results_output_options_file_name() {
        let results = vec![
            MatchingLine {
                location: Location {
                    file_name: String::from("First"),
                    line_number: 42,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
            },
            MatchingLine {
                location: Location {
                    file_name: String::from("Second"),
                    line_number: 100500,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
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
        assert_eq!(
            format_results(
                &results,
                &OutputOptions {
                    file_name: true,
                    ..Default::default()
                }
            ),
            format!(
                "First:{}{}st\nSecond:tes{}\nThird:{}{}s{}",
                Paint::blue('t'),
                Paint::blue('e'),
                Paint::blue('t'),
                Paint::blue('t'),
                Paint::blue('e'),
                Paint::blue('t')
            )
        )
    }

    #[test]
    fn results_output_options_all_options() {
        let results = vec![
            MatchingLine {
                location: Location {
                    file_name: String::from("First"),
                    line_number: 42,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
            },
            MatchingLine {
                location: Location {
                    file_name: String::from("Second"),
                    line_number: 100500,
                },
                content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
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
        assert_eq!(
            format_results(
                &results,
                &OutputOptions {
                    line_number: true,
                    file_name: true
                }
            ),
            format!(
                "First:42:{}{}st\nSecond:100500:tes{}\nThird:13:{}{}s{}",
                Paint::blue('t'),
                Paint::blue('e'),
                Paint::blue('t'),
                Paint::blue('t'),
                Paint::blue('e'),
                Paint::blue('t')
            )
        )
    }

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
