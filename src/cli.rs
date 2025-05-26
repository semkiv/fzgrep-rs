pub mod output;
pub mod request;

mod option_ids;
mod sgr_sequence;

use output::behavior::Behavior;
use output::formatting::{Formatting, StyleSet};
use request::Request;
use sgr_sequence::error::ColorOverrideParsingError;

use crate::request::Request as CoreRequest;
use crate::request::collection_strategy::CollectionStrategy;
use crate::request::match_options::context_size::ContextSize;
use crate::request::match_options::{LineNumberTracking, MatchOptions, SourceNameTracking};
use crate::request::targets::Targets;
use crate::request::targets::filter::Filter;

use clap::builder::EnumValueParser;
use clap::{Arg, ArgAction, ArgMatches, ColorChoice, Command, value_parser};
use glob::Pattern;
use log::LevelFilter;
use std::env;
use std::io::{self, IsTerminal as _};
use std::path::PathBuf;

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
/// use fzgrep::{
///     cli::{
///         args,
///         formatting::{Formatting, StyleSet},
///     },
///     ContextSize, MatchCollectionStrategy, MatchOptions, OutputBehavior, Request, Targets,
/// };
/// use log::LevelFilter;
/// use std::{
///     io::{self, IsTerminal},
///     path::PathBuf,
/// };
///
/// let args = ["fzgrep", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request,
///     Request {
///         query: String::from("query"),
///         targets: Targets::Files(vec![PathBuf::from("file")]),
///         collection_strategy: CollectionStrategy::CollectAll,
///         match_options: MatchOptions {
///             track_line_numbers: false,
///             track_source_names: false,
///             context_size: ContextSize {
///                 lines_before: 0,
///                 lines_after: 0,
///             },
///         },
///         output_behavior: Behavior::Normal(if io::stdout().is_terminal() {
///             Formatting::On(StyleSet::default())
///         } else {
///             Formatting::Off
///         }),
///         log_verbosity: LevelFilter::Error,
///     }
/// );
/// ```
///
/// ```
/// // no input files - use the standard input
/// use fzgrep::{cli::args, Targets};
///
/// let args = ["fzgrep", "query"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.targets, Targets::Stdin);
/// ```
///
/// ```
/// // no input files and `--recursive` flag - use current directory///
/// use fzgrep::{cli::args, Targets};
/// use std::{env, path::PathBuf};
///
/// let args = ["fzgrep", "--recursive", "query"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request.targets,
///     Targets::RecursiveEntries {
///         paths: vec![env::current_dir().unwrap()],
///         filter: None
///     }
/// );
/// ```
///
/// ```
/// // multiple input files
/// use fzgrep::{cli::args, Targets};
/// use std::path::PathBuf;
///
/// let args = ["fzgrep", "query", "file1", "file2", "file3"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request.targets,
///     Targets::Files(vec![
///         PathBuf::from("file1"),
///         PathBuf::from("file2"),
///         PathBuf::from("file3")
///     ])
/// );
/// // with more than one input file `--with-filename` is assumed
/// assert!(request.match_options.track_source_names);
/// ```
///
/// ```
/// // recursive mode
/// use fzgrep::{cli::args, Targets};
/// use std::path::PathBuf;
///
/// let args = ["fzgrep", "--recursive", "query", "."];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request.targets,
///     Targets::RecursiveEntries {
///         paths: vec![PathBuf::from(".")],
///         filter: None
///     }
/// );
/// ```
///
/// ```
/// // recursive mode, including only `.txt` files
/// use fzgrep::{cli::args, Filter, Targets};
/// use glob::Pattern;
/// use std::path::PathBuf;
///
/// let args = ["fzgrep", "--recursive", "--include", "*.txt", "query", "."];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request.targets,
///     Targets::RecursiveEntries {
///         paths: vec![PathBuf::from(".")],
///         filter: Some(Filter::with_include(vec![Pattern::new("*.txt").unwrap()]))
///     }
/// );
/// ```
///
/// ```
/// // recursive mode, excluding files in `build` directory
/// use fzgrep::{cli::args, Filter, Targets};
/// use glob::Pattern;
/// use std::path::PathBuf;
///
/// let args = [
///     "fzgrep",
///     "--recursive",
///     "--exclude",
///     "build/*",
///     "query",
///     ".",
/// ];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request.targets,
///     Targets::RecursiveEntries {
///         paths: vec![PathBuf::from(".")],
///         filter: Some(Filter::with_exclude(vec![Pattern::new("build/*").unwrap()]))
///     }
/// );
/// ```
///
/// ```
/// // recursive mode, including only `.txt` files except for those in `tests` directory
/// use fzgrep::{cli::args, Filter, Targets};
/// use glob::Pattern;
/// use std::path::PathBuf;
///
/// let args = [
///     "fzgrep",
///     "--recursive",
///     "--include",
///     "*.txt",
///     "--exclude",
///     "tests/*",
///     "query",
///     ".",
/// ];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request.targets,
///     Targets::RecursiveEntries {
///         paths: vec![PathBuf::from(".")],
///         filter: Some(Filter::new(
///             vec![Pattern::new("*.txt").unwrap()],
///             vec![Pattern::new("tests/*").unwrap()],
///         ))
///     }
/// );
/// ```
///
/// ```
/// // recursive mode, including only `.txt` and `.json` files except for those in `build` or `tests` directory
/// use fzgrep::{cli::args, Filter, Targets};
/// use glob::Pattern;
/// use std::path::PathBuf;
///
/// let args = [
///     "fzgrep",
///     "--recursive",
///     "--include",
///     "*.txt",
///     "--include",
///     "*.json",
///     "--exclude",
///     "build/*",
///     "--exclude",
///     "tests/*",
///     "query",
///     ".",
/// ];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request.targets,
///     Targets::RecursiveEntries {
///         paths: vec![PathBuf::from(".")],
///         filter: Some(Filter::new(
///             vec![
///                 Pattern::new("*.txt").unwrap(),
///                 Pattern::new("*.json").unwrap()
///             ],
///             vec![
///                 Pattern::new("build/*").unwrap(),
///                 Pattern::new("tests/*").unwrap()
///             ],
///         ))
///     }
/// );
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
/// assert!(request.match_options.track_source_names);
/// ```
///
/// ```
/// // with more than one input file `--with-filename` is assumed
/// // it is possible to override this by specifically opting out like so
/// use fzgrep::{cli::args, Targets};
/// use std::path::PathBuf;
///
/// let args = ["fzgrep", "--no-filename", "query", "file1", "file2"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request.targets,
///     Targets::Files(vec![PathBuf::from("file1"), PathBuf::from("file2")])
/// );
/// assert!(!request.match_options.track_source_names);
/// ```
///
/// ```
/// // symmetric context
/// use fzgrep::{cli::args, ContextSize};
///
/// let args = ["fzgrep", "--context", "2", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request.match_options.context_size,
///     ContextSize {
///         lines_before: 2,
///         lines_after: 2,
///     }
/// );
/// ```
///
/// ```
/// // asymmetric context
/// use fzgrep::{cli::args, ContextSize};
///
/// let args = [
///     "fzgrep",
///     "--before-context",
///     "1",
///     "--after-context",
///     "2",
///     "query",
///     "file",
/// ];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(
///     request.match_options.context_size,
///     ContextSize {
///         lines_before: 1,
///         lines_after: 2,
///     }
/// );
/// ```
///
/// ```
/// // collect only top 5 matches
/// use fzgrep::{cli::args, MatchCollectionStrategy};
///
/// let args = ["fzgrep", "--top", "5", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.strategy, CollectionStrategy::CollectTop(5));
/// ```
///
/// ```
/// // silence the output
/// use fzgrep::{cli::args, OutputBehavior};
/// use log::LevelFilter;
///
/// let args = ["fzgrep", "--quiet", "query", "file"];
/// let request = args::make_request(args.into_iter().map(String::from));
/// assert_eq!(request.output_behavior, Behavior::Quiet);
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
        core: CoreRequest {
            query: query_from(&matches),
            targets: targets_from(&matches),
            collection_strategy: strategy_from(&matches),
            match_options: match_options_from(&matches),
            log_verbosity: log_verbosity_from(&matches),
        },
        output_behavior: output_behavior_from(&matches),
    }
}

// TODO: split the builder

#[expect(clippy::too_many_lines, reason = "Cannot do much about this builder")]
fn match_command_line(args: impl Iterator<Item = String>) -> ArgMatches {
    Command::new(option_env!("CARGO_NAME").unwrap_or("fzgrep"))
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
        .author(option_env!("CARGO_EMAIL").unwrap_or("Andrii Semkiv <semkiv@gmail.com>"))
        .after_help(
            "With more than one FILEs assume -f.\n\
            Exit status is 0 if any match is found, 1 otherwise; if any error(s) occur, the exit status is 2."
        )
        .arg(
            Arg::new(option_ids::PATTERN)
                .value_name("PATTERN")
                .required(true)
                .help("Pattern to match"),
        )
        .arg(
            Arg::new(option_ids::TARGET)
                .value_name("TARGET")
                .num_args(0..)
                .help(
                    "Targets (file or directories) to search in;\n\
                    if none provided uses current working directory with `--recursive`,\n\
                    and the standard input otherwise"
                ),
        )
        .arg(
            Arg::new(option_ids::RECURSIVE)
                .short('r')
                .long("recursive")
                .action(ArgAction::SetTrue)
                .help("Recurse directories. '--exclude' and '--include' can be used for more fine-grained control")
        )
        .arg(
            Arg::new(option_ids::EXCLUDE)
                .long("exclude")
                .action(ArgAction::Append)
                .value_parser(Pattern::new)
                .help(
                    "A UNIX globs. Files matching this glob will be ignored.\n\
                    Can be specified multiple times and combined with '--include' option."
                )
        )
        .arg(
            Arg::new(option_ids::INCLUDE)
                .long("include")
                .action(ArgAction::Append)
                .value_parser(Pattern::new)
                .help(
                    "A UNIX globs. Files matching this glob will be ignored.\n\
                    Can be specified multiple times and combined with '--exclude' option."
                )
        )
        .arg(
            Arg::new(option_ids::LINE_NUMBER)
                .short('n')
                .long("line-number")
                .action(ArgAction::SetTrue)
                .help("Print line number with matching lines"),
        )
        .arg(
            Arg::new(option_ids::WITH_FILENAME)
                .short('f')
                .long("with-filename")
                .action(ArgAction::SetTrue)
                .conflicts_with(option_ids::NO_FILENAME)
                .help("Print file name with output lines"),
        )
        .arg(
            Arg::new(option_ids::NO_FILENAME)
                .short('F')
                .long("no-filename")
                .action(ArgAction::SetTrue)
                .conflicts_with(option_ids::WITH_FILENAME)
                .help("Suppress the file name prefix on output"),
        )
        .arg(
            Arg::new(option_ids::CONTEXT)
                .short('C')
                .long("context")
                .value_name("NUM")
                .value_parser(value_parser!(usize))
                .conflicts_with_all([option_ids::BEFORE_CONTEXT, option_ids::AFTER_CONTEXT])
                .help("Print NUM lines of surrounding context")
        )
        .arg(
            Arg::new(option_ids::BEFORE_CONTEXT)
                .short('B')
                .long("before-context")
                .value_name("NUM")
                .value_parser(value_parser!(usize))
                .conflicts_with(option_ids::CONTEXT)
                .help("Print NUM lines of leading context")
        )
        .arg(
            Arg::new(option_ids::AFTER_CONTEXT)
                .short('A')
                .long("after-context")
                .value_name("NUM")
                .value_parser(value_parser!(usize))
                .conflicts_with(option_ids::CONTEXT)
                .help("Print NUM lines of trailing context")
        )
        .arg(
            Arg::new(option_ids::TOP)
                .long("top")
                .value_name("N")
                .value_parser(value_parser!(usize))
                .help("Fetch only top N results")
        )
        .arg(
            Arg::new(option_ids::QUIET)
                .short('q')
                .long("quiet")
                .visible_alias("silent")
                .action(ArgAction::SetTrue)
                .conflicts_with(option_ids::VERBOSE)
                .help("Suppress all output")
        )
        .arg(
            Arg::new(option_ids::VERBOSE)
                .short('v')
                .long("verbose")
                .action(ArgAction::Count)
                .conflicts_with(option_ids::QUIET)
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
            Arg::new(option_ids::COLOR)
                .long("color")
                .visible_alias("colour")
                .value_name("WHEN")
                .value_parser(EnumValueParser::<ColorChoice>::new())
                .default_value("auto")
                .help(
                    "Display matched strings, lines, context, file names, line numbers and separators in color.\n\
                    With 'auto' the output is colored only when the standard input is connected to a terminal."
                )
        )
        .arg(
            Arg::new(option_ids::COLOR_OVERRIDES)
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

fn color_overrides_parser(grep_sequence: &str) -> Result<StyleSet, ColorOverrideParsingError> {
    let mut options = StyleSet::default();

    for token in grep_sequence.split(':') {
        if let Some((cap, sgr)) = token.split_once('=') {
            match cap {
                "ms" => {
                    options.selected_match = sgr_sequence::style_from(sgr)
                        .map_err(ColorOverrideParsingError::BadStyleSequence)?;
                }
                "ln" => {
                    options.line_number = sgr_sequence::style_from(sgr)
                        .map_err(ColorOverrideParsingError::BadStyleSequence)?;
                }
                "fn" => {
                    options.source_name = sgr_sequence::style_from(sgr)
                        .map_err(ColorOverrideParsingError::BadStyleSequence)?;
                }
                "se" => {
                    options.separator = sgr_sequence::style_from(sgr)
                        .map_err(ColorOverrideParsingError::BadStyleSequence)?;
                }
                "sl" => {
                    options.selected_line = sgr_sequence::style_from(sgr)
                        .map_err(ColorOverrideParsingError::BadStyleSequence)?;
                }
                "cx" => {
                    options.context = sgr_sequence::style_from(sgr)
                        .map_err(ColorOverrideParsingError::BadStyleSequence)?;
                }
                "bn" | "mt" => {
                    return Err(ColorOverrideParsingError::UnsupportedCapability(
                        cap.to_owned(),
                    ));
                }
                _ => {
                    return Err(ColorOverrideParsingError::BadCapability(cap.to_owned()));
                }
            }
        } else {
            return Err(ColorOverrideParsingError::NotAnOverride(token.to_owned()));
        }
    }

    Ok(options)
}

fn query_from(matches: &ArgMatches) -> String {
    #[expect(
        clippy::expect_used,
        reason = "QUERY argument is required, it cannot be empty, `clap` should've taken care of this"
    )]
    let query = matches
        .get_one::<String>(option_ids::PATTERN)
        .expect("QUERY argument is required, but is missing - a bug in `clap`?");
    query.clone()
}

fn targets_from(matches: &ArgMatches) -> Targets {
    matches.get_many::<String>(option_ids::TARGET).map_or_else(
        || {
            if matches.get_flag(option_ids::RECURSIVE) {
                Targets::RecursiveEntries {
                    paths: vec![env::current_dir().unwrap_or_else(|_| PathBuf::from("."))],
                    filter: filter_from(matches),
                }
            } else {
                Targets::Stdin
            }
        },
        |targets| {
            let targets = targets.map(PathBuf::from).collect::<Vec<_>>();
            if matches.get_flag(option_ids::RECURSIVE) {
                Targets::RecursiveEntries {
                    paths: targets,
                    filter: filter_from(matches),
                }
            } else {
                Targets::Files(targets)
            }
        },
    )
}

fn filter_from(matches: &ArgMatches) -> Option<Filter> {
    let exclude_globs = matches.get_many(option_ids::EXCLUDE);
    let include_globs = matches.get_many(option_ids::INCLUDE);
    match (exclude_globs, include_globs) {
        (Some(excl), Some(incl)) => Some(Filter::new(
            incl.cloned().collect(),
            excl.cloned().collect(),
        )),
        (Some(excl), None) => Some(Filter::with_exclude(excl.cloned().collect())),
        (None, Some(incl)) => Some(Filter::with_include(incl.cloned().collect())),
        (None, None) => None,
    }
}

fn strategy_from(matches: &ArgMatches) -> CollectionStrategy {
    matches
        .get_one::<usize>(option_ids::TOP)
        .map_or(CollectionStrategy::CollectAll, |cap| {
            CollectionStrategy::CollectTop(*cap)
        })
}

fn match_options_from(matches: &ArgMatches) -> MatchOptions {
    MatchOptions {
        line_number_tracking: if matches.get_flag(option_ids::LINE_NUMBER) {
            LineNumberTracking::On
        } else {
            LineNumberTracking::Off
        },
        source_name_tracking: track_source_name_from(matches),
        context_size: context_size_from(matches),
    }
}

fn track_source_name_from(matches: &ArgMatches) -> SourceNameTracking {
    // `--with-filename` flag has been specified -> file names *should* be tracked
    if matches.get_flag(option_ids::WITH_FILENAME) {
        return SourceNameTracking::On;
    }
    // `--no-filename` flag has been specified -> file names *should not* be tracked
    if matches.get_flag(option_ids::NO_FILENAME) {
        return SourceNameTracking::Off;
    }

    // `--recursive` flag has been specified -> file names *should* be tracked
    if matches.get_flag(option_ids::RECURSIVE) {
        return SourceNameTracking::On;
    }

    // no flags specified, but there are multiple input files -> file names *should* be tracked
    if matches
        .get_many::<String>(option_ids::TARGET)
        .is_some_and(|targets| targets.len() > 1)
    {
        return SourceNameTracking::On;
    }
    // default case -> file names *should not* be tracked
    SourceNameTracking::Off
}

fn context_size_from(matches: &ArgMatches) -> ContextSize {
    matches
        .get_one::<usize>(option_ids::CONTEXT)
        .copied()
        .map_or_else(
            || ContextSize {
                lines_before: matches
                    .get_one::<usize>(option_ids::BEFORE_CONTEXT)
                    .copied()
                    .unwrap_or(0),
                lines_after: matches
                    .get_one::<usize>(option_ids::AFTER_CONTEXT)
                    .copied()
                    .unwrap_or(0),
            },
            |num| ContextSize {
                lines_before: num,
                lines_after: num,
            },
        )
}

fn styleset_from(matches: &ArgMatches) -> StyleSet {
    matches
        .get_one::<StyleSet>(option_ids::COLOR_OVERRIDES)
        .copied()
        .unwrap_or_default()
}

fn formatting_from(matches: &ArgMatches) -> Formatting {
    matches
        .get_one::<ColorChoice>(option_ids::COLOR)
        .map_or_else(
            || Formatting::On(StyleSet::default()),
            |choice| match choice {
                ColorChoice::Always => Formatting::On(styleset_from(matches)),
                ColorChoice::Never => Formatting::Off,
                ColorChoice::Auto => {
                    if io::stdout().is_terminal() {
                        Formatting::On(styleset_from(matches))
                    } else {
                        Formatting::Off
                    }
                }
            },
        )
}

fn output_behavior_from(matches: &ArgMatches) -> Behavior {
    if matches.get_flag(option_ids::QUIET) {
        return Behavior::Quiet;
    }

    Behavior::Normal(formatting_from(matches))
}

fn log_verbosity_from(matches: &ArgMatches) -> LevelFilter {
    if matches.get_flag(option_ids::QUIET) {
        return LevelFilter::Off;
    }

    match matches.get_count(option_ids::VERBOSE) {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        4.. => LevelFilter::Trace,
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::non_ascii_literal, reason = "It's tests, who cares?")]

    use super::*;
    use yansi::Style;

    #[test]
    fn make_request_no_targets() {
        let args = ["fzgrep", "query"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request,
            Request {
                core: CoreRequest {
                    query: String::from("query"),
                    targets: Targets::Stdin,
                    collection_strategy: CollectionStrategy::CollectAll,
                    match_options: MatchOptions {
                        line_number_tracking: LineNumberTracking::Off,
                        source_name_tracking: SourceNameTracking::Off,
                        context_size: ContextSize {
                            lines_before: 0,
                            lines_after: 0,
                        },
                    },
                    log_verbosity: LevelFilter::Error,
                },
                output_behavior: Behavior::Normal(if io::stdout().is_terminal() {
                    Formatting::On(StyleSet::default())
                } else {
                    Formatting::Off
                }),
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
                core: CoreRequest {
                    query: String::from("query"),
                    targets: Targets::RecursiveEntries {
                        paths: vec![env::current_dir().unwrap()],
                        filter: None
                    },
                    collection_strategy: CollectionStrategy::CollectAll,
                    match_options: MatchOptions {
                        line_number_tracking: LineNumberTracking::Off,
                        source_name_tracking: SourceNameTracking::On,
                        context_size: ContextSize {
                            lines_before: 0,
                            lines_after: 0,
                        },
                    },
                    log_verbosity: LevelFilter::Error,
                },
                output_behavior: Behavior::Normal(if io::stdout().is_terminal() {
                    Formatting::On(StyleSet::default())
                } else {
                    Formatting::Off
                }),
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
                core: CoreRequest {
                    query: String::from("query"),
                    targets: Targets::Files(vec![PathBuf::from("file")]),
                    collection_strategy: CollectionStrategy::CollectAll,
                    match_options: MatchOptions {
                        line_number_tracking: LineNumberTracking::Off,
                        source_name_tracking: SourceNameTracking::Off,
                        context_size: ContextSize {
                            lines_before: 0,
                            lines_after: 0,
                        },
                    },
                    log_verbosity: LevelFilter::Error,
                },
                output_behavior: Behavior::Normal(if io::stdout().is_terminal() {
                    Formatting::On(StyleSet::default())
                } else {
                    Formatting::Off
                }),
            }
        );
    }

    #[test]
    fn make_request_multiple_targets_file_name_assumed() {
        let args = ["fzgrep", "query", "file1", "file2", "file3"];
        let request = make_request(args.into_iter().map(String::from));

        assert_eq!(
            request.core.targets,
            Targets::Files(vec![
                PathBuf::from("file1"),
                PathBuf::from("file2"),
                PathBuf::from("file3")
            ])
        );
        assert_eq!(
            request.core.match_options.source_name_tracking,
            SourceNameTracking::On
        );
    }

    #[test]
    fn make_request_recursive_file_name_assumed() {
        let args = ["fzgrep", "query", "--recursive", "."];
        let request = make_request(args.into_iter().map(String::from));

        assert_eq!(
            request.core.targets,
            Targets::RecursiveEntries {
                paths: vec![PathBuf::from(".")],
                filter: None
            }
        );
        assert_eq!(
            request.core.match_options.source_name_tracking,
            SourceNameTracking::On
        );
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
        assert_eq!(
            request.core.match_options.source_name_tracking,
            SourceNameTracking::Off
        );
    }

    #[test]
    fn make_request_non_ascii_emoji() {
        let args = ["fzgrep", "🐣🦀", "file1", "👨‍🔬.txt", "file3"];
        let request = make_request(args.into_iter().map(String::from));

        assert_eq!(request.core.query, "🐣🦀");
        assert_eq!(
            request.core.targets,
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

        assert_eq!(request.core.query, "тест");
        assert_eq!(
            request.core.targets,
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

        assert_eq!(request.core.query, "打电");
        assert_eq!(
            request.core.targets,
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
            request.core.targets,
            Targets::RecursiveEntries {
                paths: vec![PathBuf::from("dir")],
                filter: None
            }
        );
    }

    #[test]
    fn make_request_recursive_long() {
        let args = ["fzgrep", "--recursive", "query", "dir"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.targets,
            Targets::RecursiveEntries {
                paths: vec![PathBuf::from("dir")],
                filter: None
            }
        );
    }

    #[test]
    fn make_request_recursive_with_include() {
        let args = [
            "fzgrep",
            "--recursive",
            "--include",
            "*.txt",
            "query",
            "dir",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.targets,
            Targets::RecursiveEntries {
                paths: vec![PathBuf::from("dir")],
                filter: Some(Filter::with_include(vec![Pattern::new("*.txt").unwrap()]))
            }
        );
    }

    #[test]
    fn make_request_recursive_with_include_multiple_globs() {
        let args = [
            "fzgrep",
            "--recursive",
            "--include",
            "*.txt",
            "--include",
            "*.rs",
            "query",
            "dir",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.targets,
            Targets::RecursiveEntries {
                paths: vec![PathBuf::from("dir")],
                filter: Some(Filter::with_include(vec![
                    Pattern::new("*.txt").unwrap(),
                    Pattern::new("*.rs").unwrap()
                ]))
            }
        );
    }

    #[test]
    fn make_request_recursive_with_exclude() {
        let args = [
            "fzgrep",
            "--recursive",
            "--exclude",
            "build/*",
            "query",
            "dir",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.targets,
            Targets::RecursiveEntries {
                paths: vec![PathBuf::from("dir")],
                filter: Some(Filter::with_exclude(vec![Pattern::new("build/*").unwrap()]))
            }
        );
    }

    #[test]
    fn make_request_recursive_with_exclude_multiple_globs() {
        let args = [
            "fzgrep",
            "--recursive",
            "--exclude",
            "build/*",
            "--exclude",
            "tests/*",
            "query",
            "dir",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.targets,
            Targets::RecursiveEntries {
                paths: vec![PathBuf::from("dir")],
                filter: Some(Filter::with_exclude(vec![
                    Pattern::new("build/*").unwrap(),
                    Pattern::new("tests/*").unwrap()
                ]))
            }
        );
    }

    #[test]
    fn make_request_recursive_with_include_and_exclude() {
        let args = [
            "fzgrep",
            "--recursive",
            "--include",
            "*.txt",
            "--exclude",
            "tests/*",
            "query",
            "dir",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.targets,
            Targets::RecursiveEntries {
                paths: vec![PathBuf::from("dir")],
                filter: Some(Filter::new(
                    vec![Pattern::new("*.txt").unwrap()],
                    vec![Pattern::new("tests/*").unwrap()]
                ))
            }
        );
    }

    #[test]
    fn make_request_recursive_with_include_and_exclude_multiple_globs() {
        let args = [
            "fzgrep",
            "--recursive",
            "--include",
            "*.txt",
            "--include",
            "*.rs",
            "--exclude",
            "build/*",
            "--exclude",
            "tests/*",
            "query",
            "dir",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.targets,
            Targets::RecursiveEntries {
                paths: vec![PathBuf::from("dir")],
                filter: Some(Filter::new(
                    vec![
                        Pattern::new("*.txt").unwrap(),
                        Pattern::new("*.rs").unwrap()
                    ],
                    vec![
                        Pattern::new("build/*").unwrap(),
                        Pattern::new("tests/*").unwrap()
                    ]
                ))
            }
        );
    }

    #[test]
    fn make_request_line_number_short() {
        let args = ["fzgrep", "-n", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.line_number_tracking,
            LineNumberTracking::On
        );
    }

    #[test]
    fn make_request_line_number_long() {
        let args = ["fzgrep", "--line-number", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.line_number_tracking,
            LineNumberTracking::On
        );
    }

    #[test]
    fn make_request_with_file_name_short() {
        let args = ["fzgrep", "-f", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.source_name_tracking,
            SourceNameTracking::On
        );
    }

    #[test]
    fn make_request_with_file_name_long() {
        let args = ["fzgrep", "--with-filename", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.source_name_tracking,
            SourceNameTracking::On
        );
    }

    #[test]
    fn make_request_no_file_name_short() {
        let args = ["fzgrep", "-F", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.source_name_tracking,
            SourceNameTracking::Off
        );
    }

    #[test]
    fn make_request_no_file_name_long() {
        let args = ["fzgrep", "--no-filename", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.source_name_tracking,
            SourceNameTracking::Off
        );
    }

    #[test]
    fn make_request_context_short() {
        let args = ["fzgrep", "-C", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.context_size,
            ContextSize {
                lines_before: 2,
                lines_after: 2,
            }
        );
    }

    #[test]
    fn make_request_context_long() {
        let args = ["fzgrep", "--context", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.context_size,
            ContextSize {
                lines_before: 2,
                lines_after: 2,
            }
        );
    }

    #[test]
    fn make_request_context_before_short() {
        let args = ["fzgrep", "-B", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.context_size,
            ContextSize {
                lines_before: 2,
                lines_after: 0,
            }
        );
    }

    #[test]
    fn make_request_context_before_long() {
        let args = ["fzgrep", "--before-context", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.context_size,
            ContextSize {
                lines_before: 2,
                lines_after: 0,
            }
        );
    }

    #[test]
    fn make_request_context_after_short() {
        let args = ["fzgrep", "-A", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.context_size,
            ContextSize {
                lines_before: 0,
                lines_after: 2,
            }
        );
    }

    #[test]
    fn make_request_context_after_long() {
        let args = ["fzgrep", "--after-context", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.context_size,
            ContextSize {
                lines_before: 0,
                lines_after: 2,
            }
        );
    }

    #[test]
    fn make_request_context_before_after_short() {
        let args = ["fzgrep", "-B", "1", "-A", "2", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.match_options.context_size,
            ContextSize {
                lines_before: 1,
                lines_after: 2,
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
            request.core.match_options.context_size,
            ContextSize {
                lines_before: 1,
                lines_after: 2,
            }
        );
    }

    #[test]
    fn make_request_top() {
        let args = ["fzgrep", "--top", "10", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.core.collection_strategy,
            CollectionStrategy::CollectTop(10)
        );
    }

    #[test]
    fn make_request_quiet_short() {
        let args = ["fzgrep", "-q", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.output_behavior, Behavior::Quiet);
        assert_eq!(request.core.log_verbosity, LevelFilter::Off);
    }

    #[test]
    fn make_request_quiet_long() {
        let args = ["fzgrep", "--quiet", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.output_behavior, Behavior::Quiet);
        assert_eq!(request.core.log_verbosity, LevelFilter::Off);
    }

    #[test]
    fn make_request_silent_long() {
        let args = ["fzgrep", "--silent", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.output_behavior, Behavior::Quiet);
        assert_eq!(request.core.log_verbosity, LevelFilter::Off);
    }

    #[test]
    fn make_request_verbose_short() {
        let args = ["fzgrep", "-v", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.core.log_verbosity, LevelFilter::Warn);
    }

    #[test]
    fn make_request_verbose_long() {
        let args = ["fzgrep", "--verbose", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.core.log_verbosity, LevelFilter::Warn);
    }

    #[test]
    fn make_request_verbose_info_short() {
        let args = ["fzgrep", "-vv", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.core.log_verbosity, LevelFilter::Info);
    }

    #[test]
    fn make_request_verbose_info_long() {
        let args = ["fzgrep", "--verbose", "--verbose", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.core.log_verbosity, LevelFilter::Info);
    }

    #[test]
    fn make_request_verbose_debug_short() {
        let args = ["fzgrep", "-vvv", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.core.log_verbosity, LevelFilter::Debug);
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
        assert_eq!(request.core.log_verbosity, LevelFilter::Debug);
    }

    #[test]
    fn make_request_verbose_trace_short() {
        let args = ["fzgrep", "-vvvv", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.core.log_verbosity, LevelFilter::Trace);
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
        assert_eq!(request.core.log_verbosity, LevelFilter::Trace);
    }

    #[test]
    fn make_request_verbose_extra_short() {
        let args = ["fzgrep", "-vvvvv", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.core.log_verbosity, LevelFilter::Trace);
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
        assert_eq!(request.core.log_verbosity, LevelFilter::Trace);
    }

    #[test]
    fn make_request_color_auto() {
        let args = ["fzgrep", "--color", "auto", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.output_behavior,
            Behavior::Normal(if io::stdout().is_terminal() {
                Formatting::On(StyleSet::default())
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
            Behavior::Normal(Formatting::On(StyleSet::default()))
        );
    }

    #[test]
    fn make_request_color_never() {
        let args = ["fzgrep", "--color", "never", "query", "file"];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(request.output_behavior, Behavior::Normal(Formatting::Off));
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
        assert_eq!(request.output_behavior, Behavior::Normal(Formatting::Off));
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
            Style::new().green().on_yellow().bold(),
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
            Style::new().green().on_yellow().bold(),
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
                .source_name,
            Style::new().green().on_yellow().bold(),
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
            Style::new().green().on_yellow().bold(),
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
            Style::new().green().on_yellow().bold(),
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
            Style::new().green().on_yellow().bold(),
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
            Behavior::Normal(Formatting::On(StyleSet {
                selected_match: Style::new().green().on_yellow().bold(),
                line_number: Style::new().yellow().on_blue().dim(),
                source_name: Style::new().blue().on_magenta().italic(),
                ..Default::default()
            }))
        );
    }

    #[test]
    fn make_request_color_overrides_all() {
        let args = [
            "fzgrep",
            "--color",
            "always",
            "--color-overrides",
            "ms=01;34;43:sl=02;37:cx=02;37:fn=04;38;5;51:ln=03;04;38;2;127;127;127:se=35;48;2;0;192;0",
            "query",
            "file",
        ];
        let request = make_request(args.into_iter().map(String::from));
        assert_eq!(
            request.output_behavior,
            Behavior::Normal(Formatting::On(StyleSet {
                selected_match: Style::new().blue().on_yellow().bold(),
                selected_line: Style::new().white().dim(),
                context: Style::new().white().dim(),
                source_name: Style::new().fixed(51).underline(),
                line_number: Style::new().rgb(127, 127, 127).italic().underline(),
                separator: Style::new().magenta().on_rgb(0, 192, 0)
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
                core: CoreRequest {
                    query: String::from("query"),
                    targets: Targets::RecursiveEntries {
                        paths: vec![PathBuf::from("file")],
                        filter: None
                    },
                    collection_strategy: CollectionStrategy::CollectAll,
                    match_options: MatchOptions {
                        line_number_tracking: LineNumberTracking::On,
                        source_name_tracking: SourceNameTracking::On,
                        context_size: ContextSize {
                            lines_before: 1,
                            lines_after: 2
                        },
                    },
                    log_verbosity: LevelFilter::Warn,
                },
                output_behavior: Behavior::Normal(if io::stdout().is_terminal() {
                    Formatting::On(StyleSet::default())
                } else {
                    Formatting::Off
                }),
            }
        );
    }

    #[test]
    fn make_request_all_options_long() {
        let args = [
            "fzgrep",
            "--recursive",
            "--include",
            "*.txt",
            "--exclude",
            "tests/*",
            "--line-number",
            "--with-filename",
            "--before-context",
            "1",
            "--after-context",
            "2",
            "--top",
            "10",
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
                core: CoreRequest {
                    query: String::from("query"),
                    targets: Targets::RecursiveEntries {
                        paths: vec![PathBuf::from("file")],
                        filter: Some(Filter::new(
                            vec![Pattern::new("*.txt").unwrap(),],
                            vec![Pattern::new("tests/*").unwrap()]
                        ))
                    },
                    collection_strategy: CollectionStrategy::CollectTop(10),
                    match_options: MatchOptions {
                        line_number_tracking: LineNumberTracking::On,
                        source_name_tracking: SourceNameTracking::On,
                        context_size: ContextSize {
                            lines_before: 1,
                            lines_after: 2,
                        },
                    },
                    log_verbosity: LevelFilter::Warn,
                },
                output_behavior: Behavior::Normal(Formatting::On(StyleSet {
                    selected_match: Style::new().blue().blink(),
                    ..Default::default()
                })),
            }
        );
    }
}
