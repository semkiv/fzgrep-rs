mod cli;
mod core;
mod matching_results;

pub use cli::{FormattingOptions, FormattingOptionsBuilder, Request};
pub use vscode_fuzzy_score_rs::FuzzyMatch;

use colored::Colorize;
use core::reader::Reader;
use log::debug;
use matching_results::matching_line::{Location, MatchingLine};
use std::io::{self, BufRead};

/// This function handles all the application logic.
///
/// The `main` function is merely a `run` call.
///
/// The run configuration is based on `args`, which are expected to be a sequence of command line arguments.
/// The first positional argument is considered the query
/// and the rest of positional arguments are considered the files to grep.
/// If no files are supplied `stdin` will used.
///
/// # Errors
///
///   * [`std::io::Error`] if encounters any I/O related issues.
///   * If `args` do not satisfy internal invariant (e.g. there are too few arguments),
///     the parser will cause the program to exit fast using [`std::process::exit`].
///
/// For more info see the [`clap`] crate documentation.
///
pub fn run(request: Request) -> Result<(), io::Error> {
    debug!("Running with the following configuration: {:?}", request);

    let matches = find_matches(&request.query(), &request.targets())?;
    println!("{}", format_results(matches, request.formatting_options()));

    Ok(())
}

/// Find fuzzy matches using the configuration supplied `request`.
///
/// # Errors
///
///   * io::Error if encounters any I/O related issues.
///
pub fn find_matches(query: &str, targets: &Vec<String>) -> Result<Vec<MatchingLine>, io::Error> {
    let readers = if targets.is_empty() {
        vec![Reader::stdin_reader()]
    } else {
        targets
            .iter()
            .map(|f| Reader::file_reader(f))
            .collect::<Result<Vec<_>, _>>()?
    };

    let mut matches = Vec::new();
    for reader in readers {
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
pub fn format_results(matches: Vec<MatchingLine>, options: FormattingOptions) -> String {
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

        ret.push_str(&format!("{colored_target}"));

        if let Some(_) = match_itr.peek() {
            ret.push('\n');
        }
    }

    ret
}

fn process_one_target(query: &str, target: Reader) -> Result<Vec<MatchingLine>, io::Error> {
    let displayed_name = target.displayed_name().clone();
    let mut ret = Vec::new();
    for (index, line) in target.source().lines().enumerate() {
        let line = line?;
        if let Some(m) = vscode_fuzzy_score_rs::fuzzy_match(query, &line) {
            ret.push(MatchingLine {
                location: Location {
                    file_name: displayed_name.clone(),
                    line_number: index + 1,
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
            format_results(results, FormattingOptions::default()),
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
                results,
                FormattingOptionsBuilder::new().line_number(true).build()
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
                results,
                FormattingOptionsBuilder::new().file_name(true).build()
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
                results,
                FormattingOptionsBuilder::new()
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
