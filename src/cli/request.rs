use crate::cli::{
    error::{ColorOverrideParsingError, ColorSequenceParsingError, StyleSequenceParsingError},
    output_options::{FormattingOptions, OutputOptions},
};
use atty::Stream;
use clap::{parser::ValuesRef, Arg, ArgAction, ArgMatches, Command};
use log::{warn, LevelFilter};
use std::path::PathBuf;
use yansi::{Color, Style};

/// Represents a run configuration.
///
/// Holds the query, the list of files and the output output options.
///
#[derive(Debug, PartialEq)]
pub struct Request {
    query: String,
    targets: Option<Vec<PathBuf>>,
    recursive: bool,
    output_options: OutputOptions,
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
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &Some(vec![PathBuf::from("file")]));
    /// assert!(!request.recursive());
    /// assert!(!request.output_options().line_number);
    /// assert!(!request.output_options().file_name);
    /// assert!(!request.quiet());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Error);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // no input files - use the standard input
    /// let args = ["fzgrep", "query"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.targets(), &None);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // no input files and `--recursive` flag - use current directory
    /// let args = ["fzgrep", "--recursive", "query"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.targets(), &None);
    /// assert!(request.recursive());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// use std::path::PathBuf;
    /// // multiple input files
    /// let args = ["fzgrep", "query", "file1", "file2", "file3"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.targets(), &Some(vec![PathBuf::from("file1"), PathBuf::from("file2"), PathBuf::from("file3")]));
    /// // `--with-filename` is assumed in case of multiple input files
    /// assert!(request.output_options().file_name);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// use std::path::PathBuf;
    /// // recursive mode
    /// let args = ["fzgrep", "--recursive", "query", "."];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.targets(), &Some(vec![PathBuf::from(".")]));
    /// assert!(request.recursive());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // request line numbers to be printed
    /// let args = ["fzgrep", "--line-number", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert!(request.output_options().line_number);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // request file names to be printed
    /// let args = ["fzgrep", "--with-filename", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert!(request.output_options().file_name);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // with more than one input file `--with-filename` is assumed
    /// // it is possible to override this by specifically opting out like so
    /// let args = ["fzgrep", "--no-filename", "query", "file1", "file2"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert!(!request.output_options().file_name);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // silence the output
    /// let args = ["fzgrep", "--quiet", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.quiet(), true);
    /// assert_eq!(request.verbosity(), log::LevelFilter::Off);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // activate warn log messages (in addition to error messages enabled by default)
    /// let args = ["fzgrep", "--verbose", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.verbosity(), log::LevelFilter::Warn);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // activate warn and info log messages (in addition to error messages enabled by default)
    /// let args = ["fzgrep", "-vv", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.verbosity(), log::LevelFilter::Info);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // activate warn, info and debug log messages (in addition to error messages enabled by default)
    /// let args = ["fzgrep", "-vvv", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.verbosity(), log::LevelFilter::Debug);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// // activate warn, info, debug and trace log messages (in addition to error messages enabled by default)
    /// let args = ["fzgrep", "-vvvv", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.verbosity(), log::LevelFilter::Trace);
    /// ```
    ///
    pub fn new(args: impl Iterator<Item = String>) -> Self {
        let matches = match_command_line_args(args);

        Request {
            query: Request::query_from(&matches),
            targets: Request::targets_from(&matches),
            recursive: matches.get_flag("recursive"),
            output_options: OutputOptions {
                line_number: matches.get_flag("line_number"),
                file_name: Request::file_name_from(&matches),
                formatting: Request::formatting_from(&matches),
            },
            quiet: matches.get_flag("quiet"),
            verbosity: Request::verbosity_from(&matches),
        }
    }

    /// A simple getter that just returns the query.
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "query"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.query(), "query");
    /// ```
    ///
    pub fn query(&self) -> &str {
        &self.query
    }

    /// A simple getter that just returns the list of input targets,
    /// files or, potentially (with `--recursive` option), directories.
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::Request;
    /// use std::path::PathBuf;
    /// let args = ["fzgrep", "query", "file1", "file2", "file3"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.targets(), &Some(vec![PathBuf::from("file1"), PathBuf::from("file2"), PathBuf::from("file3")]));
    /// ```
    ///
    pub fn targets(&self) -> &Option<Vec<PathBuf>> {
        &self.targets
    }

    /// A simple getter that just returns whether the recursive mode is requested.
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::Request;
    /// use std::path::PathBuf;
    /// let args = ["fzgrep", "--recursive", "query"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert!(request.recursive());
    /// ```
    ///
    pub fn recursive(&self) -> bool {
        self.recursive
    }

    /// A simple getter that just returns output options.
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert!(!request.output_options().line_number);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "-n", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert!(request.output_options().line_number);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--line-number", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert!(request.output_options().line_number);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--with-filename", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert!(request.output_options().file_name);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--no-filename", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert!(!request.output_options().file_name);
    /// ```
    ///
    pub fn output_options(&self) -> OutputOptions {
        self.output_options
    }

    /// A simple getter that returns whether it has been requested to silence the output
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert!(!request.quiet());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--quiet", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert!(request.quiet());
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--silent", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
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
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert!(request.quiet());
    /// assert_eq!(request.verbosity(), log::LevelFilter::Off);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "--verbose", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.verbosity(), log::LevelFilter::Warn);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "-vv", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.verbosity(), log::LevelFilter::Info);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "-vvv", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.verbosity(), log::LevelFilter::Debug);
    /// ```
    ///
    /// ```
    /// use fzgrep::Request;
    /// let args = ["fzgrep", "-vvvv", "query", "file"];
    /// let request = Request::new(args.into_iter().map(String::from));
    /// assert_eq!(request.verbosity(), log::LevelFilter::Trace);
    /// ```
    ///
    pub fn verbosity(&self) -> LevelFilter {
        self.verbosity
    }

    fn query_from(matches: &ArgMatches) -> String {
        let query = matches
            .get_one::<String>("pattern")
            .expect("QUERY argument is required, it cannot be empty");
        query.clone()
    }

    fn targets_from(matches: &ArgMatches) -> Option<Vec<PathBuf>> {
        matches
            .get_many::<String>("target")
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
            .get_many("target")
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

    fn formatting_from(matches: &ArgMatches) -> Option<FormattingOptions> {
        if let Some(behavior) = matches.get_one::<String>("color") {
            let behavior = behavior.as_str();
            match behavior {
                "auto" | "always" => {
                    let formatting_options = matches
                        .get_one::<FormattingOptions>("color_overrides")
                        .cloned()
                        .unwrap_or_default();
                    match behavior {
                        "auto" => {
                            if atty::is(Stream::Stdout) {
                                Some(formatting_options)
                            } else {
                                None
                            }
                        }
                        "always" => Some(formatting_options),
                        _ => unreachable!(),
                    }
                }
                "never" => None,
                _ => unreachable!(),
            }
        } else {
            Some(FormattingOptions::default())
        }
    }

    fn color_overrides_parser(
        grep_sequence: &str,
    ) -> Result<FormattingOptions, ColorOverrideParsingError> {
        let mut options = FormattingOptions::default();

        for token in grep_sequence.split(':') {
            if let Some((cap, sgr)) = token.split_once('=') {
                match cap {
                    "ms" => {
                        options.selected_match = Request::style_from(sgr)
                            .map_err(ColorOverrideParsingError::BadStyleSequence)?
                    }
                    "mc" => {
                        options.context_match = Request::style_from(sgr)
                            .map_err(ColorOverrideParsingError::BadStyleSequence)?
                    }
                    "ln" => {
                        options.line_number = Request::style_from(sgr)
                            .map_err(ColorOverrideParsingError::BadStyleSequence)?
                    }
                    "fn" => {
                        options.file_name = Request::style_from(sgr)
                            .map_err(ColorOverrideParsingError::BadStyleSequence)?
                    }
                    "se" => {
                        options.separator = Request::style_from(sgr)
                            .map_err(ColorOverrideParsingError::BadStyleSequence)?
                    }
                    "sl" => {
                        options.selected_line = Request::style_from(sgr)
                            .map_err(ColorOverrideParsingError::BadStyleSequence)?
                    }
                    "cx" => {
                        options.context = Request::style_from(sgr)
                            .map_err(ColorOverrideParsingError::BadStyleSequence)?
                    }
                    "bn" | "mt" => {
                        return Err(ColorOverrideParsingError::UnsupportedCapability(
                            cap.to_string(),
                        ));
                    }
                    _ => {
                        return Err(ColorOverrideParsingError::BadCapability(cap.to_string()));
                    }
                }
            } else {
                return Err(ColorOverrideParsingError::NotAnOverride(token.to_string()));
            }
        }

        Ok(options)
    }

    fn style_from(sgr_sequence: &str) -> Result<Style, StyleSequenceParsingError> {
        let mut style = Style::default();
        let mut itr = sgr_sequence.split(';');
        while let Some(token) = itr.next() {
            if token.is_empty() {
                continue;
            }

            let code = token
                .parse::<u8>()
                .map_err(|e| StyleSequenceParsingError::NotACode(token.to_string(), e))?;
            match code {
                0 => {}
                1 => style = style.bold(),
                2 => style = style.dimmed(),
                3 => style = style.italic(),
                4 => style = style.underline(),
                5 | 6 => {
                    warn!("Slow and rapid blinks are treated the same way");
                    style = style.blink();
                }
                7 => style = style.invert(),
                8 => style = style.hidden(),
                9 => style = style.strikethrough(),
                30..=39 => {
                    style = style.fg(Request::color_from(code, &mut itr)
                        .map_err(StyleSequenceParsingError::BadColorSequence)?)
                }
                40..=49 => {
                    style = style.bg(Request::color_from(code, &mut itr)
                        .map_err(StyleSequenceParsingError::BadColorSequence)?)
                }
                10..=29 | 50..=107 => return Err(StyleSequenceParsingError::UnsupportedCode(code)),
                _ => return Err(StyleSequenceParsingError::BadCode(code)),
            }
        }

        Ok(style)
    }

    fn color_from<'a>(
        code: u8,
        itr: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Color, ColorSequenceParsingError> {
        let code_suffix = code % 10;
        match code_suffix {
            0 => Ok(Color::Black),
            1 => Ok(Color::Red),
            2 => Ok(Color::Green),
            3 => Ok(Color::Yellow),
            4 => Ok(Color::Blue),
            5 => Ok(Color::Magenta),
            6 => Ok(Color::Cyan),
            7 => Ok(Color::White),
            8 => {
                if let Some(differentiator) = itr.next() {
                    let differentiator = differentiator.parse::<u8>().map_err(|e| {
                        ColorSequenceParsingError::NotACode(differentiator.to_string(), e)
                    })?;
                    match differentiator {
                        2 => match (itr.next(), itr.next(), itr.next()) {
                            (Some(r), Some(g), Some(b)) => {
                                let r = r.parse::<u8>().map_err(|e| {
                                    ColorSequenceParsingError::NotACode(r.to_string(), e)
                                })?;
                                let g = g.parse::<u8>().map_err(|e| {
                                    ColorSequenceParsingError::NotACode(g.to_string(), e)
                                })?;
                                let b = b.parse::<u8>().map_err(|e| {
                                    ColorSequenceParsingError::NotACode(b.to_string(), e)
                                })?;
                                Ok(Color::RGB(r, g, b))
                            }
                            _ => Err(ColorSequenceParsingError::BadTrueColor),
                        },
                        5 => {
                            if let Some(n) = itr.next() {
                                let n = n.parse::<u8>().map_err(|e| {
                                    ColorSequenceParsingError::NotACode(n.to_string(), e)
                                })?;
                                Ok(Color::Fixed(n))
                            } else {
                                Err(ColorSequenceParsingError::BadFixedColor)
                            }
                        }
                        _ => Err(ColorSequenceParsingError::BadColorType(differentiator)),
                    }
                } else {
                    Err(ColorSequenceParsingError::IncompleteSequence)
                }
            }
            9 => Ok(Color::Default),
            _ => unreachable!(),
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
            Arg::new("target")
                .value_name("TARGET")
                .num_args(0..)
                .help(
                    "Targets (file or directories) to search in;\n\
                    if none provided uses current working directory with `--recursive`,\n\
                    and the standard input otherwise"
                ),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .action(ArgAction::SetTrue)
                .help("Recurse directories")
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
        .arg(
            Arg::new("color")
                .long("color")
                .visible_alias("colour")
                .value_name("WHEN")
                .value_parser(["always", "auto", "never"])
                .default_value("auto")
                .help(
                    "Display matched strings, lines, context, file names, line numbers and separators in color.\n\
                    With 'auto' the output is colored only when the standard input is connected to a terminal."
                )
        )
        .arg(
            Arg::new("color_overrides")
                .long("color-overrides")
                .visible_alias("colour-overrides")
                .value_name("CAPS")
                .value_parser(Request::color_overrides_parser)
                .help(
                    "Controls how the '--color' option highlights output.\n\
                    The format follows 'grep' and the value is expected to be a colon-separated list of capabilities\n\
                    Supported capabilities are as follows:\n\
                    \t'ms=' color for matching text in a selected line\n\
                    \t'mc=' color for matching text in a context line\n\
                    \t'ln=' color for line numbers\n\
                    \t'fn=' color for file names\n\
                    \t'se=' color for separators\n\
                    \t'sl=' color for the whole selected line (including the non-matching part)\n\
                    \t'cx=' color for the whole context (including the non-matching part)\n\
                    Note that some of `grep` capabilities (e.g. 'rv', 'ne', 'mt=', 'bn=') are not available\n\
                    The default behavior is equivalent to '--color-overrides ms=01;31:mc=01;31:sl=:cx=:fn=35:ln=32:se=36'.\n\
                    For more information see 'grep' documentation: https://man7.org/linux/man-pages/man1/grep.1.html#ENVIRONMENT\n\
                    and/or ASCII escape codes: https://en.wikipedia.org/wiki/ANSI_escape_code."
                )
        )
        .next_line_help(true)
        .get_matches_from(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_no_targets() {
        let args = ["fzgrep", "query"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: None,
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_single_target() {
        let args = ["fzgrep", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_multiple_targets() {
        let args = ["fzgrep", "query", "file1", "file2", "file3"];
        let request = Request::new(args.into_iter().map(String::from));

        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![
                    PathBuf::from("file1"),
                    PathBuf::from("file2"),
                    PathBuf::from("file3")
                ]),
                recursive: false,
                // with multiple files we implicitly enable file names
                output_options: OutputOptions {
                    file_name: true,
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_non_ascii_emoji() {
        let args = ["fzgrep", "🐣🦀", "file1", "👨‍🔬.txt", "file3"];
        let request = Request::new(args.into_iter().map(String::from));

        assert_eq!(
            request,
            Request {
                query: String::from("🐣🦀"),
                targets: Some(vec![
                    PathBuf::from("file1"),
                    PathBuf::from("👨‍🔬.txt"),
                    PathBuf::from("file3")
                ]),
                recursive: false,
                // with multiple files we implicitly enable file names
                output_options: OutputOptions {
                    file_name: true,
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_non_ascii_cyrillic() {
        let args = ["fzgrep", "тест", "file1", "тест.txt", "file3"];
        let request = Request::new(args.into_iter().map(String::from));

        assert_eq!(
            request,
            Request {
                query: String::from("тест"),
                targets: Some(vec![
                    PathBuf::from("file1"),
                    PathBuf::from("тест.txt"),
                    PathBuf::from("file3")
                ]),
                recursive: false,
                // with multiple files we implicitly enable file names
                output_options: OutputOptions {
                    file_name: true,
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_non_ascii_chinese() {
        let args = ["fzgrep", "打电", "file1", "测试.txt", "file3"];
        let request = Request::new(args.into_iter().map(String::from));

        assert_eq!(
            request,
            Request {
                query: String::from("打电"),
                targets: Some(vec![
                    PathBuf::from("file1"),
                    PathBuf::from("测试.txt"),
                    PathBuf::from("file3")
                ]),
                recursive: false,
                // with multiple files we implicitly enable file names
                output_options: OutputOptions {
                    file_name: true,
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_recursive_short() {
        let args = ["fzgrep", "-r", "query", "dir"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("dir")]),
                recursive: true,
                output_options: OutputOptions::default(),
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_recursive_long() {
        let args = ["fzgrep", "--recursive", "query", "dir"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("dir")]),
                recursive: true,
                output_options: OutputOptions::default(),
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_with_file_name_short() {
        let args = ["fzgrep", "-f", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    file_name: true,
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_with_file_name_long() {
        let args = ["fzgrep", "--with-filename", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    file_name: true,
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_no_file_name_short() {
        let args = ["fzgrep", "-F", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    file_name: false,
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_no_file_name_long() {
        let args = ["fzgrep", "--no-filename", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    file_name: false,
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_quiet_short() {
        let args = ["fzgrep", "-q", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: true,
                verbosity: LevelFilter::Off
            }
        );
    }

    #[test]
    fn constructor_quiet_long() {
        let args = ["fzgrep", "--quiet", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: true,
                verbosity: LevelFilter::Off
            }
        );
    }

    #[test]
    fn constructor_silent_long() {
        let args = ["fzgrep", "--silent", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: true,
                verbosity: LevelFilter::Off
            }
        );
    }

    #[test]
    fn constructor_verbose_short() {
        let args = ["fzgrep", "-v", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: false,
                verbosity: LevelFilter::Warn
            }
        );
    }

    #[test]
    fn constructor_verbose_long() {
        let args = ["fzgrep", "--verbose", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: false,
                verbosity: LevelFilter::Warn
            }
        );
    }

    #[test]
    fn constructor_verbose_info_short() {
        let args = ["fzgrep", "-vv", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: false,
                verbosity: LevelFilter::Info
            }
        );
    }

    #[test]
    fn constructor_verbose_info_long() {
        let args = ["fzgrep", "--verbose", "--verbose", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: false,
                verbosity: LevelFilter::Info
            }
        );
    }

    #[test]
    fn constructor_verbose_debug_short() {
        let args = ["fzgrep", "-vvv", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
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
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: false,
                verbosity: LevelFilter::Debug
            }
        );
    }

    #[test]
    fn constructor_verbose_trace_short() {
        let args = ["fzgrep", "-vvvv", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
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
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: false,
                verbosity: LevelFilter::Trace
            }
        );
    }

    #[test]
    fn constructor_verbose_extra_short() {
        let args = ["fzgrep", "-vvvvv", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
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
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: false,
                verbosity: LevelFilter::Trace
            }
        );
    }

    #[test]
    fn constructor_color_auto() {
        let args = ["fzgrep", "--color", "auto", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions::default(),
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_always() {
        let args = ["fzgrep", "--color", "always", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions::default()),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_never() {
        let args = ["fzgrep", "--color", "never", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: None,
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_never_with_color_overrides() {
        let args = [
            "fzgrep",
            "--color",
            "never",
            "--color-overrides",
            "ms=1;33",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: None,
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_reset() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=0",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_bold() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=1",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().bold(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_dim() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=2",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().dimmed(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_italic() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=3",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().italic(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_underline() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=4",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().underline(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_slow_blink() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=5",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().blink(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_rapid_blink() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=6",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().blink(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_invert() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=7",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().invert(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_hide() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=8",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().hidden(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_strike() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=9",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().strikethrough(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_fg_color_black() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=30",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Black),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_fg_color_red() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=31",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Red),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_fg_color_green() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=32",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Green),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_fg_color_yellow() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=33",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Yellow),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }
    #[test]
    fn constructor_color_overrides_selected_match_fg_color_blue() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=34",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Blue),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_fg_color_magenta() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=35",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Magenta),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_fg_color_cyan() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=36",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Cyan),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_fg_color_white() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=37",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::White),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_fg_color_8bit() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=38;5;120",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Fixed(120)),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_fg_color_24bit() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=38;2;192;255;238",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::RGB(192, 255, 238)),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_fg_color_default() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=39",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Default),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_bg_color_black() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=40",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().bg(Color::Black),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_bg_color_red() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=41",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().bg(Color::Red),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_bg_color_green() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=42",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().bg(Color::Green),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_bg_color_yellow() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=43",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().bg(Color::Yellow),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }
    #[test]
    fn constructor_color_overrides_selected_match_bg_color_blue() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=44",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().bg(Color::Blue),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_bg_color_magenta() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=45",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().bg(Color::Magenta),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_bg_color_cyan() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=46",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().bg(Color::Cyan),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_bg_color_white() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=47",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().bg(Color::White),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_bg_color_8bit() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=48;5;120",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().bg(Color::Fixed(120)),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_bg_color_24bit() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=48;2;192;255;238",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().bg(Color::RGB(192, 255, 238)),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_bg_color_default() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=49",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::default().bg(Color::Default),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_match_multiple_styles() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=33;3;4;48;2;192;255;238",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Yellow)
                            .italic()
                            .underline()
                            .bg(Color::RGB(192, 255, 238)),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_context_match() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "mc=1;32;43",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        context_match: Style::new(Color::Green).bold().bg(Color::Yellow),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_line_number() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ln=1;32;43",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        line_number: Style::new(Color::Green).bold().bg(Color::Yellow),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_file_name() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "fn=1;32;43",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        file_name: Style::new(Color::Green).bold().bg(Color::Yellow),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_separator() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "se=1;32;43",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        separator: Style::new(Color::Green).bold().bg(Color::Yellow),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_selected_line() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "sl=1;32;43",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_line: Style::new(Color::Green).bold().bg(Color::Yellow),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_context() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "cx=1;32;43",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        context: Style::new(Color::Green).bold().bg(Color::Yellow),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_multiple_capabilities() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=1;32;43:ln=2;33;44:fn=3;34;45",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Green).bold().bg(Color::Yellow),
                        line_number: Style::new(Color::Yellow).dimmed().bg(Color::Blue),
                        file_name: Style::new(Color::Blue).italic().bg(Color::Magenta),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_color_overrides_all() {
        let args = [
            "fzgrep", "--color", "always",
            "--color-overrides",
            "ms=01;34;43:mc=01;34;48;5;177:sl=02;37:cx=02;37:fn=04;38;5;51:ln=03;04;38;2;127;127;127:se=35;48;2;0;192;0",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: false,
                output_options: OutputOptions {
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Blue).bg(Color::Yellow).bold(),
                        context_match: Style::new(Color::Blue).bg(Color::Fixed(177)).bold(),
                        selected_line: Style::new(Color::White).dimmed(),
                        context: Style::new(Color::White).dimmed(),
                        file_name: Style::new(Color::Fixed(51)).underline(),
                        line_number: Style::new(Color::RGB(127, 127, 127)).italic().underline(),
                        separator: Style::new(Color::Magenta).bg(Color::RGB(0, 192, 0))
                    }),
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Error
            }
        );
    }

    #[test]
    fn constructor_all_options_short() {
        let args = ["fzgrep", "-rnfv", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: true,
                output_options: OutputOptions {
                    line_number: true,
                    file_name: true,
                    ..Default::default()
                },
                quiet: false,
                verbosity: LevelFilter::Warn
            }
        );
    }

    #[test]
    fn constructor_all_options_long() {
        let args = [
            "fzgrep",
            "--recursive",
            "--line-number",
            "--with-filename",
            "--verbose",
            "--color",
            "always",
            "--color-overrides",
            "ms=05;34",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Some(vec![PathBuf::from("file")]),
                recursive: true,
                output_options: OutputOptions {
                    line_number: true,
                    file_name: true,
                    formatting: Some(FormattingOptions {
                        selected_match: Style::new(Color::Blue).blink(),
                        ..Default::default()
                    }),
                },
                quiet: false,
                verbosity: LevelFilter::Warn
            }
        );
    }

    #[test]
    fn query() {
        let request = Request {
            query: String::from("test"),
            targets: None,
            recursive: false,
            output_options: OutputOptions::default(),
            quiet: false,
            verbosity: LevelFilter::Error,
        };
        assert_eq!(request.query(), "test");
    }

    #[test]
    fn targets() {
        let request = Request {
            query: String::from("test"),
            targets: Some(vec![
                PathBuf::from("File1"),
                PathBuf::from("File2"),
                PathBuf::from("File3"),
            ]),
            recursive: false,
            output_options: OutputOptions::default(),
            quiet: false,
            verbosity: LevelFilter::Error,
        };
        assert_eq!(
            request.targets(),
            &Some(vec![
                PathBuf::from("File1"),
                PathBuf::from("File2"),
                PathBuf::from("File3")
            ])
        );
    }

    #[test]
    fn recursive() {
        let request = Request {
            query: String::from("test"),
            targets: None,
            recursive: true,
            output_options: OutputOptions::default(),
            quiet: false,
            verbosity: LevelFilter::Error,
        };
        assert!(request.recursive());
    }

    #[test]
    fn output_options() {
        let request = Request {
            query: String::from("test"),
            targets: None,
            recursive: false,
            output_options: OutputOptions {
                line_number: true,
                file_name: true,
                ..Default::default()
            },
            quiet: false,
            verbosity: LevelFilter::Error,
        };
        assert!(request.output_options().line_number);
        assert!(request.output_options().file_name);
    }

    #[test]
    fn verbosity() {
        let request = Request {
            query: String::from("test"),
            targets: None,
            recursive: false,
            output_options: OutputOptions {
                line_number: true,
                file_name: true,
                ..Default::default()
            },
            quiet: false,
            verbosity: LevelFilter::Debug,
        };
        assert_eq!(request.verbosity(), LevelFilter::Debug);
    }
}
