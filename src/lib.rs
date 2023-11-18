mod cli;
mod core;
mod matching_results;

pub use cli::{FormattingOptions, FormattingOptionsBuilder, Request};
pub use core::exit_code::ExitCode;

use colored::Colorize;
use core::reader::Reader;
use log::debug;
use matching_results::matching_line::{Location, MatchingLine};
use std::{
    io::{self, BufRead},
    path::PathBuf,
};

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
///
pub fn run(request: &Request) -> Result<Vec<MatchingLine>, io::Error> {
    debug!("Running with the following configuration: {:?}", request);
    let matches = find_matches(request.query(), request.input_files())?;
    if !request.quiet() && !matches.is_empty() {
        println!(
            "{}",
            format_results(&matches, &request.formatting_options())
        );
    }
    Ok(matches)
}

/// Find fuzzy matches using the configuration supplied `request`.
/// If there are no input files in the request, the standard input will be used.
///
/// # Errors
///
///   * io::Error if encounters any I/O related issues.
///
pub fn find_matches(
    query: &str,
    targets: &Option<Vec<PathBuf>>,
) -> Result<Vec<MatchingLine>, io::Error> {
    let readers: Box<dyn Iterator<Item = Reader>> = if let Some(targets) = targets {
        debug!("Using the following input files: {:?}", targets);
        Box::new(targets.iter().map_while(|f| Reader::file_reader(f).ok()))
    } else {
        debug!("No input files specified, using the standard input.");
        Box::new(std::iter::once(Reader::stdin_reader()))
    };

    let mut matches = Vec::new();
    for reader in readers {
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
pub fn format_results(matches: &[MatchingLine], options: &FormattingOptions) -> String {
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
                colored_target.push_str(&ch.to_string().blue().to_string());
                matches_it.next();
            } else {
                colored_target.push(ch);
            }
        }

        if options.file_name() {
            ret.push_str(&format!("{file_name}:"));
        }
        if options.line_number() {
            ret.push_str(&format!("{line_number}:"));
        }

        ret.push_str(&colored_target);

        if match_itr.peek().is_some() {
            ret.push('\n');
        }
    }

    ret
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
    fn results_formatting_default() {
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
            format_results(&results, &FormattingOptions::default()),
            format!(
                "{}{}st\ntes{}\n{}{}s{}",
                "t".blue(),
                "e".blue(),
                "t".blue(),
                "t".blue(),
                "e".blue(),
                "t".blue()
            )
        )
    }

    #[test]
    fn results_formatting_line_number() {
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
                &FormattingOptionsBuilder::new().line_number(true).build()
            ),
            format!(
                "42:{}{}st\n100500:tes{}\n13:{}{}s{}",
                "t".blue(),
                "e".blue(),
                "t".blue(),
                "t".blue(),
                "e".blue(),
                "t".blue()
            )
        )
    }

    #[test]
    fn results_formatting_file_name() {
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
                &FormattingOptionsBuilder::new().file_name(true).build()
            ),
            format!(
                "First:{}{}st\nSecond:tes{}\nThird:{}{}s{}",
                "t".blue(),
                "e".blue(),
                "t".blue(),
                "t".blue(),
                "e".blue(),
                "t".blue()
            )
        )
    }

    #[test]
    fn results_formatting_all_options() {
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
                &FormattingOptionsBuilder::new()
                    .line_number(true)
                    .file_name(true)
                    .build()
            ),
            format!(
                "First:42:{}{}st\nSecond:100500:tes{}\nThird:13:{}{}s{}",
                "t".blue(),
                "e".blue(),
                "t".blue(),
                "t".blue(),
                "e".blue(),
                "t".blue()
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
