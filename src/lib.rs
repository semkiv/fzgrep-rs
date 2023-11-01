use colored::Colorize;
use details::{MatchInFile, MatchInLine};
use log::debug;
use std::{
    error::Error,
    fs,
    io::{self, BufReader},
};

/// Represents a run configuration.
///
/// Holds the query, the list of files and the output formatting options.
///
#[derive(Debug)]
pub struct Config {
    query: String,
    targets: Vec<String>,
    formatting_options: FormattingOptions,
}

impl Config {
    /// A constructor that parses a [`String`] iterator into a run configuration.
    ///
    /// `args` can technically be anything that satisfies the requirements,
    /// but in practice it is used just with [`std::env::args`].
    ///
    /// # Errors:
    ///
    ///   * [`Err<String>`] if parsing `args` fails. This can happen in theory fail,
    ///     but it practice it can be caused only by a violation of the constraints imposed by the parser,
    /// in which case it should exit using other mechanism (see below).
    ///   * If `args` do not satisfy internal invariant (e.g. there are too few arguments),
    ///     the parser will cause the program to exit fast using [`std::process::exit`].
    ///
    /// For more info see the [`clap`] crate documentation.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = ["fzgrep", "query", "file1", "file2", "file3"];
    /// let config = fzgrep::Config::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(config.query(), "query");
    /// assert_eq!(config.targets(), &vec![String::from("file1"), String::from("file2"), String::from("file3")]);
    /// assert!(!config.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--line-number", "query", "file"];
    /// let config = fzgrep::Config::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(config.query(), "query");
    /// assert_eq!(config.targets(), &vec![String::from("file")]);
    /// assert!(config.formatting_options().line_number());
    /// ```
    ///
    pub fn new(args: impl Iterator<Item = String>) -> Result<Config, String> {
        let matches = details::parse_args(args);
        let query = matches
            .get_one::<String>("pattern")
            .ok_or(String::from("Missing QUERY argument (required)"))?;

        let targets = matches
            .get_many::<String>("file")
            .map_or(Vec::new(), |files| files.map(String::clone).collect());

        let formatting_options_builder =
            FormattingOptionsBuilder::new().line_number(matches.get_flag("line_number"));

        Ok(Config {
            query: query.clone(),
            targets,
            formatting_options: formatting_options_builder.build(),
        })
    }

    /// A simple getter that just returns the query.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = ["fzgrep", "query"];
    /// let config = fzgrep::Config::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(config.query(), "query");
    /// ```
    ///
    pub fn query(&self) -> &str {
        &self.query
    }

    /// A simple getter that just returns the list of files.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = ["fzgrep", "query", "file1", "file2", "file3"];
    /// let config = fzgrep::Config::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(config.targets(), &vec![String::from("file1"), String::from("file2"), String::from("file3")]);
    /// ```
    ///
    pub fn targets(&self) -> &Vec<String> {
        &self.targets
    }

    /// A simple getter that just returns formatting options.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = ["fzgrep", "query", "file"];
    /// let config = fzgrep::Config::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(!config.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "-n", "query", "file"];
    /// let config = fzgrep::Config::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(config.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--line-number", "query", "file"];
    /// let config = fzgrep::Config::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(config.formatting_options().line_number());
    /// ```
    ///
    pub fn formatting_options(&self) -> FormattingOptions {
        self.formatting_options
    }
}

/// Holds various formatting options.
///
/// Specifically:
///   * whether line numbers should be printed.
///
/// Use [`FormattingOptionsBuilder`] to configure [`FormattingOptions`] if needed (using the builder pattern).
///
/// # Examples
///
/// ```
/// let options = fzgrep::FormattingOptionsBuilder::new()
///     .line_number(true)
///     .build();
/// assert!(options.line_number());
/// ```
///
#[derive(Clone, Copy, Debug, Default)]
pub struct FormattingOptions {
    line_number: bool,
}

/// A simple getter that just returns the value of `--line-number` flag.
///
/// # Examples
///
/// ```
/// let options = fzgrep::FormattingOptions::default();
/// assert!(!options.line_number());
/// ```
///
/// ```
/// let options = fzgrep::FormattingOptionsBuilder::new()
///     .line_number(true)
///     .build();
/// assert!(options.line_number());
/// ```
///
impl FormattingOptions {
    pub fn line_number(&self) -> bool {
        self.line_number
    }
}

/// A builder that can be used to build [`FormattingOptions`] objects.
///
/// # Examples
/// ```
/// let options = fzgrep::FormattingOptionsBuilder::new()
///     .line_number(true)
///     .build();
/// assert!(options.line_number());
/// ```
///
#[derive(Default)]
pub struct FormattingOptionsBuilder {
    line_number: bool,
}

impl FormattingOptionsBuilder {
    /// Creates a new builder with default options.
    ///
    /// # Examples
    /// ```
    /// let builder = fzgrep::FormattingOptionsBuilder::new();
    /// let options = builder.build();
    /// assert!(!options.line_number());
    /// ```
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `line_number` option to the provided value.
    ///
    /// # Examples
    /// ```
    /// let options = fzgrep::FormattingOptionsBuilder::new()
    ///     .line_number(true)
    ///     .build();
    /// assert!(options.line_number());
    /// ```
    ///
    pub fn line_number(mut self, line_number: bool) -> Self {
        self.line_number = line_number;
        self
    }

    /// Consumes the builder object in exchange for a configured [`FormattingOptions`] object.
    ///
    /// # Examples
    /// ```
    /// let builder = fzgrep::FormattingOptionsBuilder::new().line_number(true);
    /// let options = builder.build();
    /// assert!(options.line_number());
    /// ```
    ///
    pub fn build(self) -> FormattingOptions {
        FormattingOptions {
            line_number: self.line_number,
        }
    }
}

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
    let config = Config::new(args)?;
    debug!("Running with the following configuration: {:?}", config);

    let matches = find_matches(&config)?;
    println!("{}", format_results(matches, config.formatting_options()));

    Ok(())
}

/// Find fuzzy matches using the configuration supplied `config`.
///
/// # Errors
///
///   * io::Error if encounters any I/O related issues.
///
pub fn find_matches(config: &Config) -> Result<Vec<MatchInFile>, io::Error> {
    let mut matches = Vec::new();
    if config.targets.is_empty() {
        // no files specified => default to stdin
        let stdin_reader = Box::new(BufReader::new(io::stdin()));
        for matching_line in details::process_one_target(&config.query, stdin_reader)? {
            matches.push(MatchInFile {
                filename: None,
                matching_line,
            });
        }
    } else {
        for filename in &config.targets {
            let file_reader = Box::new(BufReader::new(fs::File::open(filename.clone())?));
            for matching_line in details::process_one_target(&config.query, file_reader)? {
                matches.push(MatchInFile {
                    filename: Some(filename.clone()),
                    matching_line,
                });
            }
        }
    }

    matches.sort_by(|a, b| {
        b.matching_line
            .fuzzy_match
            .score()
            .cmp(&a.matching_line.fuzzy_match.score())
    });

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
            ret.push_str(&format!("{}:", filename));
            if !options.line_number {
                ret.push(' ');
            }
        }
        if options.line_number {
            ret.push_str(&format!("{}: ", line_number));
        }

        ret.push_str(&format!(
            "{} (score {})",
            colored_target,
            fuzzy_match.score()
        ));

        if let Some(_) = match_itr.peek() {
            ret.push('\n');
        }
    }

    ret
}

mod details {
    use clap::{Arg, ArgAction, ArgMatches, Command};
    use std::{
        cmp::Ordering,
        io::{self, BufRead},
    };

    #[derive(Debug)]
    pub struct MatchInLine {
        pub line_number: usize,
        pub line_content: String,
        pub fuzzy_match: vscode_fuzzy_score_rs::FuzzyMatch,
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

    #[derive(Debug)]
    pub struct MatchInFile {
        pub filename: Option<String>,
        pub matching_line: MatchInLine,
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

    pub fn process_one_target(
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

    pub fn parse_args(args: impl Iterator<Item = String>) -> ArgMatches {
        Command::new(option_env!("CARGO_NAME").unwrap_or("fzgrep"))
            .version(option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
            .author(option_env!("CARGO_EMAIL").unwrap_or("Andrii Semkiv <semkiv@gmail.com>"))
            .arg(
                Arg::new("pattern")
                    .value_name("PATTERN")
                    .required(true)
                    .help("Pattern to match"),
            )
            .arg(
                Arg::new("file")
                    .value_name("FILE")
                    .num_args(0..)
                    .help("Files to search in; if none provided uses standard input"),
            )
            .arg(
                Arg::new("line_number")
                    .short('n')
                    .long("line-number")
                    .action(ArgAction::SetTrue)
                    .help("Print line number with matching lines"),
            )
            .get_matches_from(args)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_query() {
        let config = Config {
            query: String::from("test"),
            targets: Vec::new(),
            formatting_options: FormattingOptions { line_number: false },
        };
        assert_eq!(config.query(), "test");
    }

    #[test]
    fn config_targets() {
        let config = Config {
            query: String::from("test"),
            targets: vec![
                String::from("File1"),
                String::from("File2"),
                String::from("File3"),
            ],
            formatting_options: FormattingOptions { line_number: false },
        };
        assert_eq!(
            config.targets(),
            &vec![
                String::from("File1"),
                String::from("File2"),
                String::from("File3")
            ]
        );
    }

    #[test]
    fn config_formatting_options_default() {
        let config = Config {
            query: String::from("test"),
            targets: Vec::new(),
            formatting_options: FormattingOptions {
                line_number: true,
                ..FormattingOptions::default()
            },
        };
        assert!(config.formatting_options().line_number);

        let config = Config {
            query: String::from("test"),
            targets: Vec::new(),
            formatting_options: FormattingOptions {
                line_number: false,
                ..FormattingOptions::default()
            },
        };
        assert!(!config.formatting_options().line_number);
    }

    #[test]
    fn config_formatting_options_line_number() {
        let config = Config {
            query: String::from("test"),
            targets: Vec::new(),
            formatting_options: FormattingOptions {
                line_number: true,
                ..FormattingOptions::default()
            },
        };
        assert!(config.formatting_options().line_number);

        let config = Config {
            query: String::from("test"),
            targets: Vec::new(),
            formatting_options: FormattingOptions {
                line_number: false,
                ..FormattingOptions::default()
            },
        };
        assert!(!config.formatting_options().line_number);
    }

    #[test]
    fn args_parsing_stdin() -> Result<(), String> {
        let args = ["fzgrep", "Query"];
        let config = Config::new(args.into_iter().map(String::from))?;
        assert_eq!(config.query(), "Query");
        assert_eq!(config.targets(), &Vec::<String>::new(),);
        Ok(())
    }

    #[test]
    fn args_parsing_files() -> Result<(), String> {
        let args = ["fzgrep", "Query", "File1", "File2", "File3"];
        let config = Config::new(args.into_iter().map(String::from))?;
        assert_eq!(config.query(), "Query");
        assert_eq!(
            config.targets(),
            &vec![
                String::from("File1"),
                String::from("File2"),
                String::from("File3")
            ]
        );
        Ok(())
    }

    #[test]
    fn args_parsing_non_ascii_emoji() -> Result<(), String> {
        let args = ["fzgrep", "🐣🦀", "File1", "👨‍🔬.txt", "File3"];
        let config = Config::new(args.into_iter().map(String::from))?;
        assert_eq!(config.query(), "🐣🦀");
        assert_eq!(
            config.targets(),
            &vec![
                String::from("File1"),
                String::from("👨‍🔬.txt"),
                String::from("File3")
            ]
        );
        Ok(())
    }

    #[test]
    fn args_parsing_non_ascii_cyrillic() -> Result<(), String> {
        let args = ["fzgrep", "тест", "File1", "тест.txt", "File3"];
        let config = Config::new(args.into_iter().map(String::from))?;
        assert_eq!(config.query(), "тест");
        assert_eq!(
            config.targets(),
            &vec![
                String::from("File1"),
                String::from("тест.txt"),
                String::from("File3")
            ]
        );
        Ok(())
    }

    #[test]
    fn args_parsing_non_ascii_chinese() -> Result<(), String> {
        let args = ["fzgrep", "打电", "File1", "测试.txt", "File3"];
        let config = Config::new(args.into_iter().map(String::from))?;
        assert_eq!(config.query(), "打电");
        assert_eq!(
            config.targets(),
            &vec![
                String::from("File1"),
                String::from("测试.txt"),
                String::from("File3")
            ]
        );
        Ok(())
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
            format_results(results, FormattingOptions { line_number: true, ..FormattingOptions::default() }),
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
                FormattingOptions {
                    line_number: true,
                    ..FormattingOptions::default()
                }
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
