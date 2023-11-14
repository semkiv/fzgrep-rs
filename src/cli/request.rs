use crate::cli::formatting_options::{FormattingOptions, FormattingOptionsBuilder};
use clap::{parser::ValuesRef, Arg, ArgAction, ArgMatches, Command};
use log::LevelFilter;

/// Represents a run configuration.
///
/// Holds the query, the list of files and the output formatting options.
///
#[derive(Debug, PartialEq)]
pub struct Request {
    query: String,
    targets: Option<Vec<String>>,
    formatting_options: FormattingOptions,
    verbosity: LevelFilter,
}

impl Request {
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
    /// let args = ["fzgrep", "query"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &None);
    /// assert!(!request.formatting_options().line_number());
    /// assert!(!request.formatting_options().file_name());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Error)
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &Some(vec![String::from("file")]));
    /// assert!(!request.formatting_options().line_number());
    /// assert!(!request.formatting_options().file_name());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Error)
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "query", "file1", "file2", "file3"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &Some(vec![String::from("file1"), String::from("file2"), String::from("file3")]));
    /// assert!(!request.formatting_options().line_number());
    /// // `--with-filename` is assumed in case of multiple input files
    /// assert!(request.formatting_options().file_name());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Error)
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--line-number", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &Some(vec![String::from("file")]));
    /// assert!(request.formatting_options().line_number());
    /// assert!(!request.formatting_options().file_name());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Error)
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--with-filename", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &Some(vec![String::from("file")]));
    /// assert!(!request.formatting_options().line_number());
    /// assert!(request.formatting_options().file_name());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Error)
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--no-filename", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &Some(vec![String::from("file")]));
    /// assert!(!request.formatting_options().line_number());
    /// assert!(!request.formatting_options().file_name());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Error)
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--quiet", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &Some(vec![String::from("file")]));
    /// assert!(!request.formatting_options().line_number());
    /// assert!(!request.formatting_options().file_name());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Off)
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--verbose", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &Some(vec![String::from("file")]));
    /// assert!(!request.formatting_options().line_number());
    /// assert!(!request.formatting_options().file_name());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Warn)
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "-vv", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &Some(vec![String::from("file")]));
    /// assert!(!request.formatting_options().line_number());
    /// assert!(!request.formatting_options().file_name());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Info)
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "-vvv", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &Some(vec![String::from("file")]));
    /// assert!(!request.formatting_options().line_number());
    /// assert!(!request.formatting_options().file_name());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Debug)
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "-vvvv", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &Some(vec![String::from("file")]));
    /// assert!(!request.formatting_options().line_number());
    /// assert!(!request.formatting_options().file_name());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Trace)
    /// ```
    ///
    pub fn new(args: impl Iterator<Item = String>) -> Result<Request, String> {
        let matches = parse_args(args);

        let formatting_options_builder = FormattingOptionsBuilder::new()
            .line_number(matches.get_flag("line_number"))
            .file_name(Request::file_name_from(&matches));

        Ok(Request {
            query: Request::query_from(&matches)?,
            targets: Request::targets_from(&matches),
            formatting_options: formatting_options_builder.build(),
            verbosity: Request::verbosity_from(&matches),
        })
    }

    /// A simple getter that just returns the query.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = ["fzgrep", "query"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
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
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.targets(), &Some(vec![String::from("file1"), String::from("file2"), String::from("file3")]));
    /// ```
    ///
    pub fn targets(&self) -> &Option<Vec<String>> {
        &self.targets
    }

    /// A simple getter that just returns formatting options.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = ["fzgrep", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(!request.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "-n", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--line-number", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--with-filename", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.formatting_options().file_name());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--no-filename", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(!request.formatting_options().file_name());
    /// ```
    ///
    pub fn formatting_options(&self) -> FormattingOptions {
        self.formatting_options
    }

    /// A simple getter that just returns the verbosity level.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = ["fzgrep", "--quiet", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.verbosity(), log::LevelFilter::Off);
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--verbose", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.verbosity(), log::LevelFilter::Warn);
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "-vv", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.verbosity(), log::LevelFilter::Info);
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "-vvv", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.verbosity(), log::LevelFilter::Debug);
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "-vvvv", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.verbosity(), log::LevelFilter::Trace);
    /// ```
    ///
    pub fn verbosity(&self) -> LevelFilter {
        self.verbosity
    }

    fn query_from(matches: &ArgMatches) -> Result<String, String> {
        let query = matches
            .get_one::<String>("pattern")
            .ok_or(String::from("Missing QUERY argument (required)"))?;
        Ok(query.clone())
    }

    fn targets_from(matches: &ArgMatches) -> Option<Vec<String>> {
        matches
            .get_many::<String>("file")
            .map(|files| files.map(String::clone).collect())
    }

    fn file_name_from(matches: &ArgMatches) -> bool {
        // `--with-filename` flag has been specified -> file names *should* be printed
        if matches.get_flag("with_filename") {
            return true;
        }

        // `--no-filename` flag has been specified -> file names *should not* be printed
        if matches.get_flag("no_filename") {
            return false;
        }

        // no flags specified, but there are multiple input files -> file names *should* be printed
        if matches
            .get_many("file")
            .is_some_and(|fs: ValuesRef<'_, String>| fs.len() > 1)
        {
            return true;
        }

        // default case -> file names *will not* be printed
        false
    }

    fn verbosity_from(matches: &ArgMatches) -> LevelFilter {
        if matches.get_flag("quiet") {
            return LevelFilter::Off;
        }

        match matches.get_count("verbose") {
            0 => return LevelFilter::Error,
            1 => return LevelFilter::Warn,
            2 => return LevelFilter::Info,
            3 => return LevelFilter::Debug,
            4.. => return LevelFilter::Trace,
        }
    }
}

fn parse_args(args: impl Iterator<Item = String>) -> ArgMatches {
    Command::new(option_env!("CARGO_NAME").unwrap_or("fzgrep"))
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
        .author(option_env!("CARGO_EMAIL").unwrap_or("Andrii Semkiv <semkiv@gmail.com>"))
        .after_help("With more than one FILEs assume -f.")
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
        .arg(
            Arg::new("with_filename")
                .short('f')
                .long("with-filename")
                .action(ArgAction::SetTrue)
                .conflicts_with("no_filename")
                .help("Print file name with output lines"),
        )
        .arg(
            Arg::new("no_filename")
                .short('F')
                .long("no-filename")
                .action(ArgAction::SetTrue)
                .conflicts_with("with_filename")
                .help("Suppress the file name prefix on output"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .visible_alias("silent")
                .action(ArgAction::SetTrue)
                .conflicts_with("verbose")
                .help("Suppress all output")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count)
                .conflicts_with("quiet")
                .help("Verbose output. Specify multiple times to increase verbosity.\nWithout the switch only errors are reported (unless '-q' is specified);\n\t'-v' additionally enables warning messages;\n\t'-vv' additionally enables info messages;\n\t'-vvv' additionally enables debug messages;\n\tand '-vvvv' additionally enables trace messages.")
        )
        .get_matches_from(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_stdin() {
        let args = ["fzgrep", "query"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: None,
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_single_file() {
        let args = ["fzgrep", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_multiple_files() {
        let args = ["fzgrep", "query", "file1", "file2", "file3"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();

        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![
                    String::from("file1"),
                    String::from("file2"),
                    String::from("file3")
                ]),
                // with multiple files we implicitly enable file names
                formatting_options: FormattingOptionsBuilder::new().file_name(true).build(),
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_non_ascii_emoji() {
        let args = ["fzgrep", "🐣🦀", "file1", "👨‍🔬.txt", "file3"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();

        assert_eq!(
            request,
            Request {
                query: String::from("🐣🦀"),
                targets: Some(vec![
                    String::from("file1"),
                    String::from("👨‍🔬.txt"),
                    String::from("file3")
                ]),
                // with multiple files we implicitly enable file names
                formatting_options: FormattingOptionsBuilder::new().file_name(true).build(),
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_non_ascii_cyrillic() {
        let args = ["fzgrep", "тест", "file1", "тест.txt", "file3"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();

        assert_eq!(
            request,
            Request {
                query: String::from("тест"),
                targets: Some(vec![
                    String::from("file1"),
                    String::from("тест.txt"),
                    String::from("file3")
                ]),
                // with multiple files we implicitly enable file names
                formatting_options: FormattingOptionsBuilder::new().file_name(true).build(),
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_non_ascii_chinese() {
        let args = ["fzgrep", "打电", "file1", "测试.txt", "file3"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();

        assert_eq!(
            request,
            Request {
                query: String::from("打电"),
                targets: Some(vec![
                    String::from("file1"),
                    String::from("测试.txt"),
                    String::from("file3")
                ]),
                // with multiple files we implicitly enable file names
                formatting_options: FormattingOptionsBuilder::new().file_name(true).build(),
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_line_number_short() {
        let args = ["fzgrep", "-n", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptionsBuilder::new().line_number(true).build(),
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_line_number_long() {
        let args = ["fzgrep", "--line-number", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptionsBuilder::new().line_number(true).build(),
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_with_file_name_short() {
        let args = ["fzgrep", "-f", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptionsBuilder::new().file_name(true).build(),
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_with_file_name_long() {
        let args = ["fzgrep", "--with-filename", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptionsBuilder::new().file_name(true).build(),
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_no_file_name_short() {
        let args = ["fzgrep", "-F", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptionsBuilder::new().file_name(false).build(),
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_no_file_name_long() {
        let args = ["fzgrep", "--no-filename", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptionsBuilder::new().file_name(false).build(),
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_quiet_short() {
        let args = ["fzgrep", "-q", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Off
            }
        );
    }

    #[test]
    fn constructor_quiet_long() {
        let args = ["fzgrep", "--quiet", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Off
            }
        );
    }

    #[test]
    fn constructor_silent_long() {
        let args = ["fzgrep", "--silent", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Off
            }
        );
    }

    #[test]
    fn constructor_verbose_short() {
        let args = ["fzgrep", "-v", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Warn
            }
        );
    }

    #[test]
    fn constructor_verbose_long() {
        let args = ["fzgrep", "--verbose", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Warn
            }
        );
    }

    #[test]
    fn constructor_verbose_info_short() {
        let args = ["fzgrep", "-vv", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Info
            }
        );
    }

    #[test]
    fn constructor_verbose_info_long() {
        let args = ["fzgrep", "--verbose", "--verbose", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Info
            }
        );
    }

    #[test]
    fn constructor_verbose_debug_short() {
        let args = ["fzgrep", "-vvv", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Debug
            }
        );
    }

    #[test]
    fn constructor_verbose_debug_long() {
        let args = [
            "fzgrep",
            "--verbose",
            "--verbose",
            "--verbose",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Debug
            }
        );
    }

    #[test]
    fn constructor_verbose_trace_short() {
        let args = ["fzgrep", "-vvvv", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Trace
            }
        );
    }

    #[test]
    fn constructor_verbose_trace_long() {
        let args = [
            "fzgrep",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Trace
            }
        );
    }

    #[test]
    fn constructor_verbose_extra_short() {
        let args = ["fzgrep", "-vvvvv", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Trace
            }
        );
    }

    #[test]
    fn constructor_verbose_extra_long() {
        let args = [
            "fzgrep",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptions::default(),
                verbosity: LevelFilter::Trace
            }
        );
    }

    #[test]
    fn constructor_all_options_short() {
        let args = ["fzgrep", "-nfv", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptionsBuilder::new()
                    .line_number(true)
                    .file_name(true)
                    .build(),
                verbosity: LevelFilter::Warn
            }
        );
    }

    #[test]
    fn constructor_all_options_long() {
        let args = [
            "fzgrep",
            "--line-number",
            "--with-filename",
            "--verbose",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from)).unwrap();
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![String::from("file")]),
                formatting_options: FormattingOptionsBuilder::new()
                    .line_number(true)
                    .file_name(true)
                    .build(),
                verbosity: LevelFilter::Warn
            }
        );
    }

    #[test]
    fn query() {
        let request = Request {
            query: String::from("test"),
            targets: None,
            formatting_options: FormattingOptions::default(),
            verbosity: LevelFilter::Error,
        };
        assert_eq!(request.query(), "test");
    }

    #[test]
    fn targets() {
        let request = Request {
            query: String::from("test"),
            targets: Some(vec![
                String::from("File1"),
                String::from("File2"),
                String::from("File3"),
            ]),
            formatting_options: FormattingOptions::default(),
            verbosity: LevelFilter::Error,
        };
        assert_eq!(
            request.targets(),
            &Some(vec![
                String::from("File1"),
                String::from("File2"),
                String::from("File3")
            ])
        );
    }

    #[test]
    fn formatting_options() {
        let request = Request {
            query: String::from("test"),
            targets: None,
            formatting_options: FormattingOptionsBuilder::new()
                .line_number(true)
                .file_name(true)
                .build(),
            verbosity: LevelFilter::Error,
        };
        assert!(request.formatting_options().line_number());
        assert!(request.formatting_options().file_name());
    }

    #[test]
    fn verbosity() {
        let request = Request {
            query: String::from("test"),
            targets: None,
            formatting_options: FormattingOptionsBuilder::new()
                .line_number(true)
                .file_name(true)
                .build(),
            verbosity: LevelFilter::Debug,
        };
        assert_eq!(request.verbosity(), LevelFilter::Debug);
    }
}
