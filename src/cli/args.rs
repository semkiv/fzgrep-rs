use crate::{
    cli::{
        error::ColorOverrideParsingError,
        formatting::{Formatting, FormattingOptions},
        sgr_sequence,
    },
    core::request::{ContextSize, Lines, MatchOptions, OutputBehavior, Request, Targets},
};
use atty::Stream;
use clap::{parser::ValuesRef, value_parser, Arg, ArgAction, ArgMatches, Command};
use log::LevelFilter;
use std::{env, path::PathBuf};

/// Sets up a [`Request`] struct based on the program command line arguments
///
/// `args` can technically be any [`String`] iterator but in practice it is expected to be used only with [`std::env::args`].
///
/// # Errors:
///
/// If `args` do not satisfy internal invariant (e.g. there are too few arguments),
/// the parser will cause the program to exit fast using [`std::process::exit`].
/// For more info see the [`clap`] crate documentation.
///
/// # Examples:
///
/// ```
/// // basic usage
/// use atty::{self, Stream};
/// use fzgrep::cli::{args, formatting::{Formatting, FormattingOptions}};
/// use fzgrep::{ContextSize, Lines, MatchOptions, OutputBehavior, Request, Targets};
/// use log::LevelFilter;
/// use std::path::PathBuf;
///
/// let args = ["fzgrep", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request,
///     Request{
///         query: String::from("query"),
///         targets: Targets::Files(vec![PathBuf::from("file")]),
///         match_options: MatchOptions {
///             track_line_numbers: false,
///             track_file_names: false,
///             context_size: ContextSize {
///                 before: Lines(0),
///                 after: Lines(0),
///             },
///         },
///         output_behavior: OutputBehavior::Normal(
///             if atty::is(Stream::Stdout) {
///                 Formatting::On(FormattingOptions::default())
///             } else {
///                 Formatting::Off
///             }
///         ),
///         log_verbosity: LevelFilter::Error,
///     }
/// );
/// ```
///
/// ```
/// // no input files - use the standard input
/// use fzgrep::cli::args;
/// use fzgrep::Targets;
///
/// let args = ["fzgrep", "query"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.targets, Targets::Stdin);
/// ```
///
/// ```
/// // no input files and `--recursive` flag - use current directory///
/// use fzgrep::cli::args;
/// use fzgrep::Targets;
/// use std::{env, path::PathBuf};
///
/// let args = ["fzgrep", "--recursive", "query"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.targets, Targets::RecursiveEntries(vec![env::current_dir().unwrap()]));
/// ```
///
/// ```
/// // multiple input files
/// use fzgrep::cli::args;
/// use fzgrep::Targets;
/// use std::path::PathBuf;
///
/// let args = ["fzgrep", "query", "file1", "file2", "file3"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.targets, Targets::Files(vec![PathBuf::from("file1"), PathBuf::from("file2"), PathBuf::from("file3")]));
/// // with more than one input file `--with-filename` is assumed
/// assert!(request.match_options.track_file_names);
/// ```
///
/// ```
/// // recursive mode
/// use fzgrep::cli::args;
/// use fzgrep::Targets;
/// use std::path::PathBuf;
///
/// let args = ["fzgrep", "--recursive", "query", "."];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.targets, Targets::RecursiveEntries(vec![PathBuf::from(".")]));
/// ```
///
/// ```
/// // request line numbers to be printed
/// use fzgrep::cli::args;
///
/// let args = ["fzgrep", "--line-number", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert!(request.match_options.track_line_numbers);
/// ```
///
/// ```
/// // request file names to be printed
/// use fzgrep::cli::args;
///
/// let args = ["fzgrep", "--with-filename", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert!(request.match_options.track_file_names);
/// ```
///
/// ```
/// // with more than one input file `--with-filename` is assumed
/// // it is possible to override this by specifically opting out like so
/// use fzgrep::cli::args;
/// use fzgrep::Targets;
/// use std::path::PathBuf;
///
/// let args = ["fzgrep", "--no-filename", "query", "file1", "file2"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.targets, Targets::Files(vec![PathBuf::from("file1"), PathBuf::from("file2")]));
/// assert!(!request.match_options.track_file_names);
/// ```
///
/// ```
/// // symmetric context
/// use fzgrep::cli::args;
/// use fzgrep::{ContextSize, Lines};
///
/// let args = ["fzgrep", "--context", "2", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request.match_options.context_size,
///     ContextSize {
///         before: Lines(2),
///         after: Lines(2),
///     }
/// );
/// ```
///
/// ```
/// // asymmetric context
/// use fzgrep::cli::args;
/// use fzgrep::{ContextSize, Lines};
///
/// let args = ["fzgrep", "--before-context", "1", "--after-context", "2", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request.match_options.context_size,
///     ContextSize {
///         before: Lines(1),
///         after: Lines(2),
///     }
/// );
/// ```
///
/// ```
/// // silence the output
/// use fzgrep::cli::args;
/// use fzgrep::OutputBehavior;
/// use log::LevelFilter;
///
/// let args = ["fzgrep", "--quiet", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.output_behavior, OutputBehavior::Quiet);
/// assert_eq!(request.log_verbosity, LevelFilter::Off);
/// ```
///
/// ```
/// // activate warn log messages (in addition to error messages enabled by default)
/// use fzgrep::cli::args;
/// use log::LevelFilter;
///
/// let args = ["fzgrep", "--verbose", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.log_verbosity, LevelFilter::Warn);
/// ```
///
/// ```
/// // activate warn and info log messages (in addition to error messages enabled by default)
/// use fzgrep::cli::args;
/// use log::LevelFilter;
///
/// let args = ["fzgrep", "-vv", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.log_verbosity, LevelFilter::Info);
/// ```
///
/// ```
/// // activate warn, info and debug log messages (in addition to error messages enabled by default)
/// use fzgrep::cli::args;
/// use log::LevelFilter;
///
/// let args = ["fzgrep", "-vvv", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.log_verbosity, LevelFilter::Debug);
/// ```
///
/// ```
/// // activate warn, info, debug and trace log messages (in addition to error messages enabled by default)
/// use fzgrep::cli::args;
/// use log::LevelFilter;
///
/// let args = ["fzgrep", "-vvvv", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.log_verbosity, LevelFilter::Trace);
/// ```
///
pub fn make_request(args: impl Iterator<Item = String>) -> Request {
    let matches = match_command_line(args);

    Request {
        query: query_from(&matches),
        targets: targets_from(&matches),
        match_options: match_options_from(&matches),
        output_behavior: output_behavior_from(&matches),
        log_verbosity: log_verbosity_from(&matches),
    }
}

fn match_command_line(args: impl Iterator<Item = String>) -> ArgMatches {
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
            Arg::new("context")
                .short('C')
                .long("context")
                .value_name("NUM")
                .value_parser(value_parser!(usize))
                .conflicts_with_all(["before_context", "after_context"])
                .help("Print NUM lines of surrounding context")
        )
        .arg(
            Arg::new("before_context")
                .short('B')
                .long("before-context")
                .value_name("NUM")
                .value_parser(value_parser!(usize))
                .conflicts_with("context")
                .help("Print NUM lines of leading context")
        )
        .arg(
            Arg::new("after_context")
                .short('A')
                .long("after-context")
                .value_name("NUM")
                .value_parser(value_parser!(usize))
                .conflicts_with("context")
                .help("Print NUM lines of trailing context")
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
                .value_parser(color_overrides_parser)
                .help(
                    "Controls how the '--color' option highlights output.\n\
                    The format follows 'grep' and the value is expected to be a colon-separated list of capabilities\n\
                    Supported capabilities are as follows:\n\
                    \t'ms=' color for matching text in a selected line\n\
                    \t'ln=' color for line numbers\n\
                    \t'fn=' color for file names\n\
                    \t'se=' color for separators\n\
                    \t'sl=' color for the whole selected line (the non-matching part)\n\
                    \t'cx=' color for the surrounding context\n\
                    Note that some of `grep` capabilities (e.g. 'rv', 'ne', 'mt=', 'bn=') are not available\n\
                    The default behavior is equivalent to '--color-overrides ms=01;31:mc=01;31:sl=:cx=:fn=35:ln=32:se=36'.\n\
                    For more information see 'grep' documentation: https://man7.org/linux/man-pages/man1/grep.1.html#ENVIRONMENT\n\
                    and/or ASCII escape codes: https://en.wikipedia.org/wiki/ANSI_escape_code."
                )
        )
        .next_line_help(true)
        .get_matches_from(args)
}

fn color_overrides_parser(
    grep_sequence: &str,
) -> Result<FormattingOptions, ColorOverrideParsingError> {
    let mut options = FormattingOptions::default();

    for token in grep_sequence.split(':') {
        if let Some((cap, sgr)) = token.split_once('=') {
            match cap {
                "ms" => {
                    options.selected_match = sgr_sequence::style_from(sgr)
                        .map_err(ColorOverrideParsingError::BadStyleSequence)?
                }
                "ln" => {
                    options.line_number = sgr_sequence::style_from(sgr)
                        .map_err(ColorOverrideParsingError::BadStyleSequence)?
                }
                "fn" => {
                    options.file_name = sgr_sequence::style_from(sgr)
                        .map_err(ColorOverrideParsingError::BadStyleSequence)?
                }
                "se" => {
                    options.separator = sgr_sequence::style_from(sgr)
                        .map_err(ColorOverrideParsingError::BadStyleSequence)?
                }
                "sl" => {
                    options.selected_line = sgr_sequence::style_from(sgr)
                        .map_err(ColorOverrideParsingError::BadStyleSequence)?
                }
                "cx" => {
                    options.context = sgr_sequence::style_from(sgr)
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

fn query_from(matches: &ArgMatches) -> String {
    let query = matches
        .get_one::<String>("pattern")
        .expect("QUERY argument is required, it cannot be empty");
    query.clone()
}

fn targets_from(matches: &ArgMatches) -> Targets {
    match matches.get_many::<String>("target") {
        Some(targets) => {
            let targets = targets.map(PathBuf::from).collect::<Vec<_>>();
            if matches.get_flag("recursive") {
                Targets::RecursiveEntries(targets)
            } else {
                Targets::Files(targets)
            }
        }
        None => {
            if matches.get_flag("recursive") {
                Targets::RecursiveEntries(vec![env::current_dir().unwrap_or(PathBuf::from("."))])
            } else {
                Targets::Stdin
            }
        }
    }
}

fn match_options_from(matches: &ArgMatches) -> MatchOptions {
    MatchOptions {
        track_line_numbers: matches.get_flag("line_number"),
        track_file_names: track_file_name_from(matches),
        context_size: context_size_from(matches),
    }
}

fn track_file_name_from(matches: &ArgMatches) -> bool {
    // `--with-filename` flag has been specified -> file names *should* be tracked
    if matches.get_flag("with_filename") {
        return true;
    }
    // `--no-filename` flag has been specified -> file names *should not* be tracked
    if matches.get_flag("no_filename") {
        return false;
    }
    // no flags specified, but there are multiple input files -> file names *should* be tracked
    if matches
        .get_many("target")
        .is_some_and(|fs: ValuesRef<'_, String>| fs.len() > 1)
    {
        return true;
    }
    // default case -> file names *should not* be tracked
    false
}

fn context_size_from(matches: &ArgMatches) -> ContextSize {
    if let Some(num) = matches.get_one::<usize>("context").copied() {
        ContextSize {
            before: Lines(num),
            after: Lines(num),
        }
    } else {
        ContextSize {
            before: Lines(
                matches
                    .get_one::<usize>("before_context")
                    .copied()
                    .unwrap_or(0),
            ),
            after: Lines(
                matches
                    .get_one::<usize>("after_context")
                    .copied()
                    .unwrap_or(0),
            ),
        }
    }
}

fn formatting_from(matches: &ArgMatches) -> Formatting {
    if let Some(behavior) = matches.get_one::<String>("color") {
        let behavior = behavior.as_str();
        if behavior == "always" || (behavior == "auto" && atty::is(Stream::Stdout)) {
            let formatting_options = matches
                .get_one::<FormattingOptions>("color_overrides")
                .cloned()
                .unwrap_or_default();
            Formatting::On(formatting_options)
        } else if behavior == "never" || (behavior == "auto" && atty::isnt(Stream::Stdout)) {
            Formatting::Off
        } else {
            unreachable!();
        }
    } else {
        Formatting::On(FormattingOptions::default())
    }
}

fn output_behavior_from(matches: &ArgMatches) -> OutputBehavior {
    if matches.get_flag("quiet") {
        return OutputBehavior::Quiet;
    }

    OutputBehavior::Normal(formatting_from(matches))
}

fn log_verbosity_from(matches: &ArgMatches) -> LevelFilter {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::request::Lines;
    use yansi::{Color, Style};

    #[test]
    fn make_request_no_targets() {
        let args = ["fzgrep", "query"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Targets::Stdin,
                match_options: MatchOptions {
                    track_line_numbers: false,
                    track_file_names: false,
                    context_size: ContextSize {
                        before: Lines(0),
                        after: Lines(0),
                    },
                },
                output_behavior: OutputBehavior::Normal(Formatting::On(
                    FormattingOptions::default()
                )),
                log_verbosity: LevelFilter::Error,
            }
        );
    }

    #[test]
    fn make_request_no_targets_recursive() {
        let args = ["fzgrep", "--recursive", "query"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Targets::RecursiveEntries(vec![env::current_dir().unwrap()]),
                match_options: MatchOptions {
                    track_line_numbers: false,
                    track_file_names: false,
                    context_size: ContextSize {
                        before: Lines(0),
                        after: Lines(0),
                    },
                },
                output_behavior: OutputBehavior::Normal(Formatting::On(
                    FormattingOptions::default()
                )),
                log_verbosity: LevelFilter::Error,
            }
        );
    }

    #[test]
    fn make_request_single_target() {
        let args = ["fzgrep", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Targets::Files(vec![PathBuf::from("file")]),
                match_options: MatchOptions {
                    track_line_numbers: false,
                    track_file_names: false,
                    context_size: ContextSize {
                        before: Lines(0),
                        after: Lines(0),
                    },
                },
                output_behavior: OutputBehavior::Normal(Formatting::On(
                    FormattingOptions::default()
                )),
                log_verbosity: LevelFilter::Error,
            }
        );
    }

    #[test]
    fn make_request_multiple_targets() {
        let args = ["fzgrep", "query", "file1", "file2", "file3"];
        let request = make_request(args.into_iter().map(String::from));

        assert_eq!(
            request.targets,
            Targets::Files(vec![
                PathBuf::from("file1"),
                PathBuf::from("file2"),
                PathBuf::from("file3")
            ])
        );
        assert!(request.match_options.track_file_names);
    }

    #[test]
    fn make_request_multiple_targets_no_filename() {
        let args = [
            "fzgrep",
            "--no-filename",
            "query",
            "file1",
            "file2",
            "file3",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert!(!request.match_options.track_file_names);
    }

    #[test]
    fn make_request_non_ascii_emoji() {
        let args = ["fzgrep", "🐣🦀", "file1", "👨‍🔬.txt", "file3"];
        let request = make_request(args.into_iter().map(String::from));

        assert_eq!(request.query, "🐣🦀");
        assert_eq!(
            request.targets,
            Targets::Files(vec![
                PathBuf::from("file1"),
                PathBuf::from("👨‍🔬.txt"),
                PathBuf::from("file3")
            ])
        );
    }

    #[test]
    fn make_request_non_ascii_cyrillic() {
        let args = ["fzgrep", "тест", "file1", "тест.txt", "file3"];
        let request = make_request(args.into_iter().map(String::from));

        assert_eq!(request.query, "тест");
        assert_eq!(
            request.targets,
            Targets::Files(vec![
                PathBuf::from("file1"),
                PathBuf::from("тест.txt"),
                PathBuf::from("file3")
            ])
        );
    }

    #[test]
    fn make_request_non_ascii_chinese() {
        let args = ["fzgrep", "打电", "file1", "测试.txt", "file3"];
        let request = make_request(args.into_iter().map(String::from));

        assert_eq!(request.query, "打电");
        assert_eq!(
            request.targets,
            Targets::Files(vec![
                PathBuf::from("file1"),
                PathBuf::from("测试.txt"),
                PathBuf::from("file3")
            ])
        );
    }

    #[test]
    fn make_request_recursive_short() {
        let args = ["fzgrep", "-r", "query", "dir"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.targets,
            Targets::RecursiveEntries(vec![PathBuf::from("dir")])
        );
    }

    #[test]
    fn make_request_recursive_long() {
        let args = ["fzgrep", "--recursive", "query", "dir"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.targets,
            Targets::RecursiveEntries(vec![PathBuf::from("dir")])
        );
    }

    #[test]
    fn make_request_with_file_name_short() {
        let args = ["fzgrep", "-f", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert!(request.match_options.track_file_names);
    }

    #[test]
    fn make_request_with_file_name_long() {
        let args = ["fzgrep", "--with-filename", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert!(request.match_options.track_file_names);
    }

    #[test]
    fn make_request_no_file_name_short() {
        let args = ["fzgrep", "-F", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert!(!request.match_options.track_file_names);
    }

    #[test]
    fn make_request_no_file_name_long() {
        let args = ["fzgrep", "--no-filename", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert!(!request.match_options.track_file_names);
    }

    #[test]
    fn make_request_context_short() {
        let args = ["fzgrep", "-C", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.match_options.context_size,
            ContextSize {
                before: Lines(2),
                after: Lines(2),
            }
        );
    }

    #[test]
    fn make_request_context_long() {
        let args = ["fzgrep", "--context", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.match_options.context_size,
            ContextSize {
                before: Lines(2),
                after: Lines(2),
            }
        );
    }

    #[test]
    fn make_request_context_before_short() {
        let args = ["fzgrep", "-B", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.match_options.context_size,
            ContextSize {
                before: Lines(2),
                after: Lines(0),
            }
        );
    }

    #[test]
    fn make_request_context_before_long() {
        let args = ["fzgrep", "--before-context", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.match_options.context_size,
            ContextSize {
                before: Lines(2),
                after: Lines(0),
            }
        );
    }

    #[test]
    fn make_request_context_after_short() {
        let args = ["fzgrep", "-A", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.match_options.context_size,
            ContextSize {
                before: Lines(0),
                after: Lines(2),
            }
        );
    }

    #[test]
    fn make_request_context_after_long() {
        let args = ["fzgrep", "--after-context", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.match_options.context_size,
            ContextSize {
                before: Lines(0),
                after: Lines(2),
            }
        );
    }

    #[test]
    fn make_request_context_before_after_short() {
        let args = ["fzgrep", "-B", "1", "-A", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.match_options.context_size,
            ContextSize {
                before: Lines(1),
                after: Lines(2),
            }
        );
    }

    #[test]
    fn make_request_context_before_after_long() {
        let args = [
            "fzgrep",
            "--before-context",
            "1",
            "--after-context",
            "2",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.match_options.context_size,
            ContextSize {
                before: Lines(1),
                after: Lines(2),
            }
        );
    }

    #[test]
    fn make_request_quiet_short() {
        let args = ["fzgrep", "-q", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.output_behavior, OutputBehavior::Quiet);
        assert_eq!(request.log_verbosity, LevelFilter::Off);
    }

    #[test]
    fn make_request_quiet_long() {
        let args = ["fzgrep", "--quiet", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.output_behavior, OutputBehavior::Quiet);
        assert_eq!(request.log_verbosity, LevelFilter::Off);
    }

    #[test]
    fn make_request_silent_long() {
        let args = ["fzgrep", "--silent", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.output_behavior, OutputBehavior::Quiet);
        assert_eq!(request.log_verbosity, LevelFilter::Off);
    }

    #[test]
    fn make_request_verbose_short() {
        let args = ["fzgrep", "-v", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.log_verbosity, LevelFilter::Warn);
    }

    #[test]
    fn make_request_verbose_long() {
        let args = ["fzgrep", "--verbose", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.log_verbosity, LevelFilter::Warn);
    }

    #[test]
    fn make_request_verbose_info_short() {
        let args = ["fzgrep", "-vv", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.log_verbosity, LevelFilter::Info);
    }

    #[test]
    fn make_request_verbose_info_long() {
        let args = ["fzgrep", "--verbose", "--verbose", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.log_verbosity, LevelFilter::Info);
    }

    #[test]
    fn make_request_verbose_debug_short() {
        let args = ["fzgrep", "-vvv", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.log_verbosity, LevelFilter::Debug);
    }

    #[test]
    fn make_request_verbose_debug_long() {
        let args = [
            "fzgrep",
            "--verbose",
            "--verbose",
            "--verbose",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.log_verbosity, LevelFilter::Debug);
    }

    #[test]
    fn make_request_verbose_trace_short() {
        let args = ["fzgrep", "-vvvv", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.log_verbosity, LevelFilter::Trace);
    }

    #[test]
    fn make_request_verbose_trace_long() {
        let args = [
            "fzgrep",
            "--verbose",
            "--verbose",
            "--verbose",
            "--verbose",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.log_verbosity, LevelFilter::Trace);
    }

    #[test]
    fn make_request_verbose_extra_short() {
        let args = ["fzgrep", "-vvvvv", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.log_verbosity, LevelFilter::Trace);
    }

    #[test]
    fn make_request_verbose_extra_long() {
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
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.log_verbosity, LevelFilter::Trace);
    }

    #[test]
    fn make_request_color_auto() {
        let args = ["fzgrep", "--color", "auto", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.output_behavior,
            OutputBehavior::Normal(if atty::is(Stream::Stdout) {
                Formatting::On(FormattingOptions::default())
            } else {
                Formatting::Off
            })
        );
    }

    #[test]
    fn make_request_color_always() {
        let args = ["fzgrep", "--color", "always", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.output_behavior,
            OutputBehavior::Normal(Formatting::On(FormattingOptions::default()))
        );
    }

    #[test]
    fn make_request_color_never() {
        let args = ["fzgrep", "--color", "never", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.output_behavior,
            OutputBehavior::Normal(Formatting::Off)
        );
    }

    #[test]
    fn make_request_color_never_with_color_overrides() {
        let args = [
            "fzgrep",
            "--color",
            "never",
            "--color-overrides",
            "ms=1;33",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.output_behavior,
            OutputBehavior::Normal(Formatting::Off)
        );
    }

    #[test]
    fn make_request_color_overrides_selected_match() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=1;32;43",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request
                .output_behavior
                .formatting()
                .unwrap()
                .options()
                .unwrap()
                .selected_match,
            Style::new(Color::Green).bold().bg(Color::Yellow),
        );
    }

    #[test]
    fn make_request_color_overrides_line_number() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ln=1;32;43",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request
                .output_behavior
                .formatting()
                .unwrap()
                .options()
                .unwrap()
                .line_number,
            Style::new(Color::Green).bold().bg(Color::Yellow),
        );
    }

    #[test]
    fn make_request_color_overrides_file_name() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "fn=1;32;43",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request
                .output_behavior
                .formatting()
                .unwrap()
                .options()
                .unwrap()
                .file_name,
            Style::new(Color::Green).bold().bg(Color::Yellow),
        );
    }

    #[test]
    fn make_request_color_overrides_separator() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "se=1;32;43",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request
                .output_behavior
                .formatting()
                .unwrap()
                .options()
                .unwrap()
                .separator,
            Style::new(Color::Green).bold().bg(Color::Yellow),
        );
    }

    #[test]
    fn make_request_color_overrides_selected_line() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "sl=1;32;43",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request
                .output_behavior
                .formatting()
                .unwrap()
                .options()
                .unwrap()
                .selected_line,
            Style::new(Color::Green).bold().bg(Color::Yellow),
        );
    }

    #[test]
    fn make_request_color_overrides_context() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "cx=1;32;43",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request
                .output_behavior
                .formatting()
                .unwrap()
                .options()
                .unwrap()
                .context,
            Style::new(Color::Green).bold().bg(Color::Yellow),
        );
    }

    #[test]
    fn make_request_color_overrides_multiple_capabilities() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=1;32;43:ln=2;33;44:fn=3;34;45",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.output_behavior,
            OutputBehavior::Normal(Formatting::On(FormattingOptions {
                selected_match: Style::new(Color::Green).bold().bg(Color::Yellow),
                line_number: Style::new(Color::Yellow).dimmed().bg(Color::Blue),
                file_name: Style::new(Color::Blue).italic().bg(Color::Magenta),
                ..Default::default()
            }))
        );
    }

    #[test]
    fn make_request_color_overrides_all() {
        let args = [
            "fzgrep", "--color", "always",
            "--color-overrides",
            "ms=01;34;43:sl=02;37:cx=02;37:fn=04;38;5;51:ln=03;04;38;2;127;127;127:se=35;48;2;0;192;0",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.output_behavior,
            OutputBehavior::Normal(Formatting::On(FormattingOptions {
                selected_match: Style::new(Color::Blue).bg(Color::Yellow).bold(),
                selected_line: Style::new(Color::White).dimmed(),
                context: Style::new(Color::White).dimmed(),
                file_name: Style::new(Color::Fixed(51)).underline(),
                line_number: Style::new(Color::RGB(127, 127, 127)).italic().underline(),
                separator: Style::new(Color::Magenta).bg(Color::RGB(0, 192, 0))
            }))
        );
    }

    #[test]
    fn make_request_all_options_short() {
        let args = ["fzgrep", "-rnfv", "-B1", "-A2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Targets::RecursiveEntries(vec![PathBuf::from("file")]),
                output_behavior: OutputBehavior::Normal(Formatting::On(
                    FormattingOptions::default()
                )),
                match_options: MatchOptions {
                    track_line_numbers: true,
                    track_file_names: true,
                    context_size: ContextSize {
                        before: Lines(1),
                        after: Lines(2)
                    },
                },
                log_verbosity: LevelFilter::Warn,
            }
        );
    }

    #[test]
    fn make_request_all_options_long() {
        let args = [
            "fzgrep",
            "--recursive",
            "--line-number",
            "--with-filename",
            "--before-context",
            "1",
            "--after-context",
            "2",
            "--verbose",
            "--color",
            "always",
            "--color-overrides",
            "ms=05;34",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                query: String::from("query"),
                targets: Targets::RecursiveEntries(vec![PathBuf::from("file")]),
                output_behavior: OutputBehavior::Normal(Formatting::On(FormattingOptions {
                    selected_match: Style::new(Color::Blue).blink(),
                    ..Default::default()
                })),
                match_options: MatchOptions {
                    track_line_numbers: true,
                    track_file_names: true,
                    context_size: ContextSize {
                        before: Lines(1),
                        after: Lines(2)
                    },
                },
                log_verbosity: LevelFilter::Warn,
            }
        );
    }
}
