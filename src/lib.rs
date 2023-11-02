mod cli;

pub use cli::{FormattingOptions, FormattingOptionsBuilder, Request};
pub use vscode_fuzzy_score_rs::FuzzyMatch;

use colored::Colorize;
use log::debug;
use std::{
    cmp::Ordering,
    error::Error,
    fs,
    io::{self, BufRead, BufReader},
};

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
///   * [`Box<Err<String>>`] if fails to parse `args`.
///   * [`Box<std::io::Error>`] if encounters any I/O related issues.
///   * If `args` do not satisfy internal invariant (e.g. there are too few arguments),
///     the parser will cause the program to exit fast using [`std::process::exit`].
///
/// For more info see the [`clap`] crate documentation.
///
pub fn run(args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let request = Request::new(args)?;
    debug!("Running with the following configuration: {:?}", request);

    let matches = find_matches(&request)?;
    println!("{}", format_results(matches, request.formatting_options()));

    Ok(())
}

/// Find fuzzy matches using the configuration supplied `request`.
///
/// # Errors
///
///   * io::Error if encounters any I/O related issues.
///
pub fn find_matches(request: &Request) -> Result<Vec<MatchInFile>, io::Error> {
    let mut matches = Vec::new();
    if request.targets().is_empty() {
        // no files specified => default to stdin
        let stdin_reader = Box::new(BufReader::new(io::stdin()));
        for matching_line in process_one_target(request.query(), stdin_reader)? {
            matches.push(MatchInFile {
                filename: None,
                matching_line,
            });
        }
    } else {
        for filename in request.targets() {
            let file_reader = Box::new(BufReader::new(fs::File::open(filename.clone())?));
            for matching_line in process_one_target(request.query(), file_reader)? {
                matches.push(MatchInFile {
                    filename: Some(filename.clone()),
                    matching_line,
                });
            }
        }
    }

    // sort in descending order
    matches.sort_by(|a, b| b.cmp(a));

    Ok(matches)
}

/// Formats supplied `matches` into a rich text string.
///
/// When grepping files the format is as follows:
/// ```text
/// <filename>:<line-number>: <colored-matching-line> (score: <score>)
/// ```
/// where `colored-matching-line` is a matching line with matching characters painted blue.
///
/// In case of using `stdin` the format is slightly different:
/// ```text
/// <line-number>: <colored-matching-line> (score: <score>)
/// ```
///
pub fn format_results(matches: Vec<MatchInFile>, options: FormattingOptions) -> String {
    let mut ret = String::new();
    let mut match_itr = matches.iter().peekable();
    while let Some(m) = match_itr.next() {
        let MatchInFile {
            filename,
            matching_line:
                MatchInLine {
                    line_number,
                    line_content: line,
                    fuzzy_match,
                },
        } = m;
        let mut colored_target = String::new();
        let mut matches_it = fuzzy_match.positions().iter().peekable();
        for (index, ch) in line.chars().enumerate() {
            if matches_it.peek().is_some_and(|pos| **pos == index) {
                colored_target.push_str(&ch.to_string().blue().to_string());
                matches_it.next();
            } else {
                colored_target.push(ch);
            }
        }

        if let Some(filename) = filename {
            ret.push_str(&format!("{filename}:"));
            if !options.line_number() {
                ret.push(' ');
            }
        }
        if options.line_number() {
            ret.push_str(&format!("{line_number}: "));
        }

        ret.push_str(&format!("{colored_target} (score {})", fuzzy_match.score()));

        if let Some(_) = match_itr.peek() {
            ret.push('\n');
        }
    }

    ret
}

/// Represents a match in a line.
#[derive(Debug)]
pub struct MatchInLine {
    line_number: usize,
    line_content: String,
    fuzzy_match: FuzzyMatch,
}

impl MatchInLine {
    /// Just a simple getter that returns a number of the matching line.
    pub fn line_number(&self) -> usize {
        self.line_number
    }

    /// Just a simple getter that returns the whole matching line.
    pub fn line_content(&self) -> &String {
        &self.line_content
    }

    /// Just a simple getter that returns a [`FuzzyMatch`] object for this match.
    pub fn fuzzy_match(&self) -> &FuzzyMatch {
        &self.fuzzy_match
    }
}

impl PartialEq for MatchInLine {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_match.eq(&other.fuzzy_match)
    }
}

impl PartialOrd for MatchInLine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.fuzzy_match.partial_cmp(&other.fuzzy_match)
    }
}

impl Eq for MatchInLine {}

impl Ord for MatchInLine {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fuzzy_match.cmp(&other.fuzzy_match)
    }
}

/// Represents a match in a file.
#[derive(Debug)]
pub struct MatchInFile {
    filename: Option<String>,
    matching_line: MatchInLine,
}

impl MatchInFile {
    /// Just a simple getter that returns the name of the file.
    /// In case of using the standard input returns [`None`].
    pub fn filename(&self) -> &Option<String> {
        &self.filename
    }

    /// Just a simple getter that return [`MatchInLine`] object for this match.
    pub fn matching_line(&self) -> &MatchInLine {
        &self.matching_line
    }
}

impl PartialEq for MatchInFile {
    fn eq(&self, other: &Self) -> bool {
        self.matching_line.eq(&other.matching_line)
    }
}

impl PartialOrd for MatchInFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.matching_line.partial_cmp(&other.matching_line)
    }
}

impl Eq for MatchInFile {}

impl Ord for MatchInFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.matching_line.cmp(&other.matching_line)
    }
}

fn process_one_target(
    query: &str,
    target: Box<dyn BufRead>,
) -> Result<Vec<MatchInLine>, io::Error> {
    let mut ret = Vec::new();
    for (index, line) in target.lines().enumerate() {
        let line = line?;
        if let Some(m) = vscode_fuzzy_score_rs::fuzzy_match(query, &line) {
            ret.push(MatchInLine {
                line_number: index + 1,
                line_content: line,
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
    fn match_in_line_line_number() {
        let m = MatchInLine {
            line_number: 42,
            line_content: String::new(),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
        };
        assert_eq!(m.line_number(), 42);
    }

    #[test]
    fn match_in_line_line_content() {
        let m = MatchInLine {
            line_number: 42,
            line_content: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
        };
        assert_eq!(m.line_content(), "test");
    }

    #[test]
    fn match_in_line_fuzzy_match() {
        let m = MatchInLine {
            line_number: 42,
            line_content: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
        };
        assert_eq!(
            m.fuzzy_match(),
            &vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap()
        );
    }

    #[test]
    fn match_in_line_comparisons_ne() {
        let m1 = MatchInLine {
            line_number: 42,
            line_content: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
        };
        let m2 = MatchInLine {
            line_number: 42,
            line_content: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tes", "test").unwrap(),
        };
        assert_ne!(m1, m2);
    }

    #[test]
    fn match_in_line_comparisons_eq() {
        let m1 = MatchInLine {
            line_number: 42,
            line_content: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
        };
        let m2 = MatchInLine {
            line_number: 43,
            line_content: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
        };
        assert_eq!(m1, m2);
    }

    #[test]
    fn match_in_line_comparisons_lt() {
        let m1 = MatchInLine {
            line_number: 42,
            line_content: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test1").unwrap(),
        };
        let m2 = MatchInLine {
            line_number: 41,
            line_content: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
        };
        assert!(m1 < m2);
    }

    #[test]
    fn match_in_line_comparisons_gt() {
        let m1 = MatchInLine {
            line_number: 42,
            line_content: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
        };
        let m2 = MatchInLine {
            line_number: 41,
            line_content: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test2").unwrap(),
        };
        assert!(m1 > m2);
    }

    #[test]
    fn match_in_line_comparisons_le() {
        let m1 = MatchInLine {
            line_number: 42,
            line_content: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test1").unwrap(),
        };
        let m2 = MatchInLine {
            line_number: 42,
            line_content: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
        };
        assert!(m1 <= m2);
    }

    #[test]
    fn match_in_line_comparisons_ge() {
        let m1 = MatchInLine {
            line_number: 41,
            line_content: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
        };
        let m2 = MatchInLine {
            line_number: 41,
            line_content: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test2").unwrap(),
        };
        assert!(m1 >= m2);
    }

    #[test]
    fn match_in_file_filename() {
        let m = MatchInFile {
            filename: None,
            matching_line: MatchInLine {
                line_number: 42,
                line_content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
            },
        };
        assert_eq!(m.filename(), &None);

        let m = MatchInFile {
            filename: Some(String::from("test")),
            matching_line: MatchInLine {
                line_number: 42,
                line_content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
            },
        };
        assert_eq!(m.filename().clone().unwrap(), "test");
    }

    #[test]
    fn match_in_file_matching_line() {
        let m = MatchInFile {
            filename: None,
            matching_line: MatchInLine {
                line_number: 42,
                line_content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
            },
        };
        assert_eq!(
            m.matching_line(),
            &MatchInLine {
                line_number: 42,
                line_content: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
            }
        );
    }

    #[test]
    fn match_in_file_comparisons_ne() {
        let m1 = MatchInFile {
            filename: None,
            matching_line: MatchInLine {
                line_number: 42,
                line_content: String::from("test1"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test1").unwrap(),
            },
        };
        let m2 = MatchInFile {
            filename: None,
            matching_line: MatchInLine {
                line_number: 42,
                line_content: String::from("test2"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
            },
        };

        assert_ne!(m1, m2);
    }

    #[test]
    fn match_in_file_comparisons_eq() {
        let m1 = MatchInFile {
            filename: None,
            matching_line: MatchInLine {
                line_number: 42,
                line_content: String::from("test1"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
            },
        };
        let m2 = MatchInFile {
            filename: Some(String::from("test")),
            matching_line: MatchInLine {
                line_number: 43,
                line_content: String::from("test2"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
            },
        };

        assert_eq!(m1, m2);
    }

    #[test]
    fn match_in_file_comparisons_lt() {
        let m1 = MatchInFile {
            filename: None,
            matching_line: MatchInLine {
                line_number: 42,
                line_content: String::from("test1"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test1").unwrap(),
            },
        };
        let m2 = MatchInFile {
            filename: None,
            matching_line: MatchInLine {
                line_number: 42,
                line_content: String::from("test2"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
            },
        };

        assert!(m1 < m2);
    }

    #[test]
    fn match_in_file_comparisons_gt() {
        let m1 = MatchInFile {
            filename: None,
            matching_line: MatchInLine {
                line_number: 42,
                line_content: String::from("test1"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
            },
        };
        let m2 = MatchInFile {
            filename: Some(String::from("test")),
            matching_line: MatchInLine {
                line_number: 43,
                line_content: String::from("test2"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test2").unwrap(),
            },
        };

        assert!(m1 > m2);
    }

    #[test]
    fn match_in_file_comparisons_le() {
        let m1 = MatchInFile {
            filename: Some(String::from("test")),
            matching_line: MatchInLine {
                line_number: 42,
                line_content: String::from("test1"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test1").unwrap(),
            },
        };
        let m2 = MatchInFile {
            filename: None,
            matching_line: MatchInLine {
                line_number: 42,
                line_content: String::from("test2"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
            },
        };

        assert!(m1 < m2);
    }

    #[test]
    fn match_in_file_comparisons_ge() {
        let m1 = MatchInFile {
            filename: None,
            matching_line: MatchInLine {
                line_number: 43,
                line_content: String::from("test1"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
            },
        };
        let m2 = MatchInFile {
            filename: None,
            matching_line: MatchInLine {
                line_number: 42,
                line_content: String::from("test2"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test2").unwrap(),
            },
        };

        assert!(m1 > m2);
    }

    #[test]
    fn results_formatting_default() {
        let results = vec![
            MatchInFile {
                filename: Some(String::from("First")),
                matching_line: MatchInLine {
                    line_number: 42,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                },
            },
            MatchInFile {
                filename: Some(String::from("Second")),
                matching_line: MatchInLine {
                    line_number: 100500,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                },
            },
            MatchInFile {
                filename: Some(String::from("Third")),
                matching_line: MatchInLine {
                    line_number: 13,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                },
            },
        ];
        assert_eq!(
            format_results(results, FormattingOptions::default()),
            format!(
                "First: {}{}st (score 17)\nSecond: tes{} (score 2)\nThird: {}{}s{} (score 19)",
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
    fn results_formatting_default_no_filename() {
        let results = vec![
            MatchInFile {
                filename: None,
                matching_line: MatchInLine {
                    line_number: 42,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                },
            },
            MatchInFile {
                filename: None,
                matching_line: MatchInLine {
                    line_number: 100500,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                },
            },
            MatchInFile {
                filename: None,
                matching_line: MatchInLine {
                    line_number: 13,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                },
            },
        ];
        assert_eq!(
            format_results(results, FormattingOptions::default()),
            format!(
                "{}{}st (score 17)\ntes{} (score 2)\n{}{}s{} (score 19)",
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
            MatchInFile {
                filename: Some(String::from("First")),
                matching_line: MatchInLine {
                    line_number: 42,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                },
            },
            MatchInFile {
                filename: Some(String::from("Second")),
                matching_line: MatchInLine {
                    line_number: 100500,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                },
            },
            MatchInFile {
                filename: Some(String::from("Third")),
                matching_line: MatchInLine {
                    line_number: 13,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                },
            },
        ];
        assert_eq!(
            format_results(results, FormattingOptionsBuilder::new().line_number(true).build()),
            format!(
                "First:42: {}{}st (score 17)\nSecond:100500: tes{} (score 2)\nThird:13: {}{}s{} (score 19)",
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
    fn results_formatting_no_filename_line_number() {
        let results = vec![
            MatchInFile {
                filename: None,
                matching_line: MatchInLine {
                    line_number: 42,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                },
            },
            MatchInFile {
                filename: None,
                matching_line: MatchInLine {
                    line_number: 100500,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                },
            },
            MatchInFile {
                filename: None,
                matching_line: MatchInLine {
                    line_number: 13,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                },
            },
        ];
        assert_eq!(
            format_results(
                results,
                FormattingOptionsBuilder::new().line_number(true).build()
            ),
            format!(
                "42: {}{}st (score 17)\n100500: tes{} (score 2)\n13: {}{}s{} (score 19)",
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
            MatchInFile {
                filename: Some(String::from("Third")),
                matching_line: MatchInLine {
                    line_number: 13,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                },
            },
            MatchInFile {
                filename: Some(String::from("First")),
                matching_line: MatchInLine {
                    line_number: 100500,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                },
            },
            MatchInFile {
                filename: Some(String::from("Second")),
                matching_line: MatchInLine {
                    line_number: 42,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                },
            },
        ];

        let expected = vec![
            MatchInFile {
                filename: Some(String::from("First")),
                matching_line: MatchInLine {
                    line_number: 100500,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                },
            },
            MatchInFile {
                filename: Some(String::from("Second")),
                matching_line: MatchInLine {
                    line_number: 42,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                },
            },
            MatchInFile {
                filename: Some(String::from("Third")),
                matching_line: MatchInLine {
                    line_number: 13,
                    line_content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                },
            },
        ];

        results.sort();
        assert_eq!(results, expected);
    }
}
