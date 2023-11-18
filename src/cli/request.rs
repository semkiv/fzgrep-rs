use crate::cli::formatting_options::{FormattingOptions, FormattingOptionsBuilder};
use clap::{parser::ValuesRef, Arg, ArgAction, ArgMatches, Command};
use log::LevelFilter;
use std::path::PathBuf;

/// Represents a run configuration.
///
/// Holds the query, the list of files and the output formatting options.
///
#[derive(Debug, PartialEq)]
pub struct Request {
    query: String,
    input_files: Option<Vec<PathBuf>>,
    formatting_options: FormattingOptions,
    quiet: bool,
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
    /// use fzgrep::Request;
    /// use std::path::PathBuf;
    /// // basic usage
    /// let args = ["fzgrep", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.input_files(), &Some(vec![PathBuf::from("file")]));
    /// assert!(!request.formatting_options().line_number());
    /// assert!(!request.formatting_options().file_name());
    /// assert!(!request.quiet());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Error);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // no input files - use the standard input
    /// let args = ["fzgrep", "query"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.input_files(), &None);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// use std::path::PathBuf;
    /// // multiple input files
    /// let args = ["fzgrep", "query", "file1", "file2", "file3"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.input_files(), &Some(vec![PathBuf::from("file1"), PathBuf::from("file2"), PathBuf::from("file3")]));
    /// // `--with-filename` is assumed in case of multiple input files
    /// assert!(request.formatting_options().file_name());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // request line numbers to be printed
    /// let args = ["fzgrep", "--line-number", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // request file names to be printed
    /// let args = ["fzgrep", "--with-filename", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.formatting_options().file_name());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // with more than one input file `--with-filename` is assumed
    /// // it is possible to override this by specifically opting out like so
    /// let args = ["fzgrep", "--no-filename", "query", "file1", "file2"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(!request.formatting_options().file_name());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // silence the output
    /// let args = ["fzgrep", "--quiet", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.quiet(), true);
    /// assert_eq!(request.verbosity(), log::LevelFilter::Off);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // activate warn log messages (in addition to error messages enabled by default)
    /// let args = ["fzgrep", "--verbose", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.verbosity(), log::LevelFilter::Warn);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // activate warn and info log messages (in addition to error messages enabled by default)
    /// let args = ["fzgrep", "-vv", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.verbosity(), log::LevelFilter::Info);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // activate warn, info and debug log messages (in addition to error messages enabled by default)
    /// let args = ["fzgrep", "-vvv", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.verbosity(), log::LevelFilter::Debug);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // activate warn, info, debug and trace log messages (in addition to error messages enabled by default)
    /// let args = ["fzgrep", "-vvvv", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.verbosity(), log::LevelFilter::Trace);
    /// ```
    ///
    pub fn new(args: impl Iterator<Item = String>) -> Result<Request, String> {
        let matches = match_command_line_args(args);

        let formatting_options_builder = FormattingOptionsBuilder::new()
            .line_number(matches.get_flag("line_number"))
            .file_name(Request::file_name_from(&matches));

        Ok(Request {
            query: Request::query_from(&matches)?,
            input_files: Request::targets_from(&matches),
            formatting_options: formatting_options_builder.build(),
            quiet: matches.get_flag("quiet"),
            verbosity: Request::verbosity_from(&matches),
        })
    }

    /// A simple getter that just returns the query.
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "query"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
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
    /// use fzgrep::Request;
    /// use std::path::PathBuf;
    /// let args = ["fzgrep", "query", "file1", "file2", "file3"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.input_files(), &Some(vec![PathBuf::from("file1"), PathBuf::from("file2"), PathBuf::from("file3")]));
    /// ```
    ///
    pub fn input_files(&self) -> &Option<Vec<PathBuf>> {
        &self.input_files
    }

    /// A simple getter that just returns formatting options.
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(!request.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "-n", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--line-number", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--with-filename", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.formatting_options().file_name());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--no-filename", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(!request.formatting_options().file_name());
    /// ```
    ///
    pub fn formatting_options(&self) -> FormattingOptions {
        self.formatting_options
    }

    /// A simple getter that returns whether it has been requested to silence the output
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(!request.quiet());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--quiet", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.quiet());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--silent", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.quiet());
    /// ```
    ///
    pub fn quiet(&self) -> bool {
        self.quiet
    }

    /// A simple getter that just returns the verbosity level.
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--quiet", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.quiet());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Off);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--verbose", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.verbosity(), log::LevelFilter::Warn);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "-vv", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.verbosity(), log::LevelFilter::Info);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "-vvv", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.verbosity(), log::LevelFilter::Debug);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "-vvvv", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from)).unwrap();
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

    fn targets_from(matches: &ArgMatches) -> Option<Vec<PathBuf>> {
        matches
            .get_many::<String>("file")
            .map(|files| files.map(PathBuf::from).collect())
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
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            4.. => LevelFilter::Trace,
        }
    }
}

fn match_command_line_args(args: impl Iterator<Item = String>) -> ArgMatches {
    Command::new(option_env!("CARGO_NAME").unwrap_or("fzgrep"))
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
        .author(option_env!("CARGO_EMAIL").unwrap_or("Andrii Semkiv <semkiv@gmail.com>"))
        .after_help(
            "With more than one FILEs assume -f.\n\
            Exit status is 0 if any match is found, 1 otherwise; if any error(s) occur, the exit status is 2."
        )
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
                .help(
                    "Verbose output. Specify multiple times to increase verbosity.\n\
                    Without the switch only errors are reported (unless '-q' is specified);\n\
                    \t'-v' additionally enables warning messages;\n\
                    \t'-vv' additionally enables info messages;\n\
                    \t'-vvv' additionally enables debug messages;\n\
                    \tand '-vvvv' additionally enables trace messages."
                )
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
                input_files: None,
                formatting_options: FormattingOptions::default(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: false,
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
                input_files: Some(vec![
                    PathBuf::from("file1"),
                    PathBuf::from("file2"),
                    PathBuf::from("file3")
                ]),
                // with multiple files we implicitly enable file names
                formatting_options: FormattingOptionsBuilder::new().file_name(true).build(),
                quiet: false,
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
                input_files: Some(vec![
                    PathBuf::from("file1"),
                    PathBuf::from("👨‍🔬.txt"),
                    PathBuf::from("file3")
                ]),
                // with multiple files we implicitly enable file names
                formatting_options: FormattingOptionsBuilder::new().file_name(true).build(),
                quiet: false,
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
                input_files: Some(vec![
                    PathBuf::from("file1"),
                    PathBuf::from("тест.txt"),
                    PathBuf::from("file3")
                ]),
                // with multiple files we implicitly enable file names
                formatting_options: FormattingOptionsBuilder::new().file_name(true).build(),
                quiet: false,
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
                input_files: Some(vec![
                    PathBuf::from("file1"),
                    PathBuf::from("测试.txt"),
                    PathBuf::from("file3")
                ]),
                // with multiple files we implicitly enable file names
                formatting_options: FormattingOptionsBuilder::new().file_name(true).build(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptionsBuilder::new().line_number(true).build(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptionsBuilder::new().line_number(true).build(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptionsBuilder::new().file_name(true).build(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptionsBuilder::new().file_name(true).build(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptionsBuilder::new().file_name(false).build(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptionsBuilder::new().file_name(false).build(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: true,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: true,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: true,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptions::default(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptionsBuilder::new()
                    .line_number(true)
                    .file_name(true)
                    .build(),
                quiet: false,
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
                input_files: Some(vec![PathBuf::from("file")]),
                formatting_options: FormattingOptionsBuilder::new()
                    .line_number(true)
                    .file_name(true)
                    .build(),
                quiet: false,
                verbosity: LevelFilter::Warn
            }
        );
    }

    #[test]
    fn query() {
        let request = Request {
            query: String::from("test"),
            input_files: None,
            formatting_options: FormattingOptions::default(),
            quiet: false,
            verbosity: LevelFilter::Error,
        };
        assert_eq!(request.query(), "test");
    }

    #[test]
    fn targets() {
        let request = Request {
            query: String::from("test"),
            input_files: Some(vec![
                PathBuf::from("File1"),
                PathBuf::from("File2"),
                PathBuf::from("File3"),
            ]),
            formatting_options: FormattingOptions::default(),
            quiet: false,
            verbosity: LevelFilter::Error,
        };
        assert_eq!(
            request.input_files(),
            &Some(vec![
                PathBuf::from("File1"),
                PathBuf::from("File2"),
                PathBuf::from("File3")
            ])
        );
    }

    #[test]
    fn formatting_options() {
        let request = Request {
            query: String::from("test"),
            input_files: None,
            formatting_options: FormattingOptionsBuilder::new()
                .line_number(true)
                .file_name(true)
                .build(),
            quiet: false,
            verbosity: LevelFilter::Error,
        };
        assert!(request.formatting_options().line_number());
        assert!(request.formatting_options().file_name());
    }

    #[test]
    fn verbosity() {
        let request = Request {
            query: String::from("test"),
            input_files: None,
            formatting_options: FormattingOptionsBuilder::new()
                .line_number(true)
                .file_name(true)
                .build(),
            quiet: false,
            verbosity: LevelFilter::Debug,
        };
        assert_eq!(request.verbosity(), LevelFilter::Debug);
    }
}
