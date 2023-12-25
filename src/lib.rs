mod cli;
mod core;
mod matching_results;

pub use cli::{Context, Formatting, FormattingOptions, OutputBehavior, OutputOptions, Request};
pub use core::exit_code::ExitCode;

use core::reader::Reader;
use log::debug;
use matching_results::matching_line::{Location, MatchingLine};
use std::{
    env, error,
    io::{self, BufRead, Write},
    iter,
    ops::Range,
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
///   * [`std::fmt::Error`] if encounters any formatting related issues.
///   * [`std::io::Error`] if encounters any I/O related issues.
///   * [`walkdir::Error`] if any errors related to recursive processing occur
///
pub fn run(
    request: &Request,
    dest: &mut impl Write,
) -> Result<Vec<MatchingLine>, Box<dyn error::Error>> {
    debug!("Running with the following configuration: {:?}", request);
    let matches = find_matches(request.query(), request.targets(), request.recursive())?;
    match request.output_behavior() {
        OutputBehavior::Full(options) => {
            if !matches.is_empty() {
                write!(dest, "{}", format_results(&matches, options))?;
            }
        }
        OutputBehavior::Quiet => {}
    }
    Ok(matches)
}

/// Find fuzzy matches using the configuration supplied `request`.
/// If there are no input files in the request, the standard input will be used.
///
/// # Errors
///
///   * [`io::Error`] if encounters any I/O related issues.
///   * [`walkdir::Error`] if any errors related to recursive processing occur
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
        let mut str_itr = content.chars();
        let mut previous_range_end = 0usize;
        for range in group_indices(fuzzy_match.positions()) {
            {
                let preceding_non_match = str_itr
                    .by_ref()
                    .take(range.start - previous_range_end)
                    .collect::<String>();
                // The check is needed because `yansi::Paint` inserts formatting sequence even for empty strings.
                // Visually it makes no difference, but there are extra characters in the output,
                // making it harder to validate and compare results.
                if !preceding_non_match.is_empty() {
                    match options.formatting {
                        Formatting::Off => {
                            colored_target.push_str(&preceding_non_match);
                        }
                        Formatting::On(formatting) => {
                            colored_target.push_str(
                                &Paint::new(preceding_non_match)
                                    .with_style(formatting.selected_line)
                                    .to_string(),
                            );
                        }
                    }
                }
            }
            {
                let matching_part = str_itr
                    .by_ref()
                    .take(range.end - range.start)
                    .collect::<String>();
                match options.formatting {
                    Formatting::Off => {
                        colored_target.push_str(&matching_part);
                    }
                    Formatting::On(formatting) => {
                        colored_target.push_str(
                            &Paint::new(matching_part)
                                .with_style(formatting.selected_match)
                                .to_string(),
                        );
                    }
                }
            }
            previous_range_end = range.end;
        }
        let remaining_non_match = str_itr.collect::<String>();
        // The check is needed because `yansi::Paint` inserts formatting sequence even for empty strings.
        // Visually it makes no difference, but there are extra characters in the output,
        // making it harder to validate and compare results.
        if !remaining_non_match.is_empty() {
            match options.formatting {
                Formatting::Off => colored_target.push_str(&remaining_non_match),
                Formatting::On(formatting) => colored_target.push_str(
                    &Paint::new(remaining_non_match)
                        .with_style(formatting.selected_line)
                        .to_string(),
                ),
            }
        }

        if options.file_name {
            match options.formatting {
                Formatting::Off => ret.push_str(&format!("{file_name}:")),
                Formatting::On(formatting) => {
                    ret.push_str(
                        &Paint::new(file_name)
                            .with_style(formatting.file_name)
                            .to_string(),
                    );
                    ret.push_str(&Paint::new(':').with_style(formatting.separator).to_string());
                }
            }
        }
        if options.line_number {
            match options.formatting {
                Formatting::Off => ret.push_str(&format!("{line_number}:")),
                Formatting::On(formatting) => {
                    ret.push_str(
                        &Paint::new(line_number)
                            .with_style(formatting.line_number)
                            .to_string(),
                    );
                    ret.push_str(&Paint::new(':').with_style(formatting.separator).to_string());
                }
            }
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

fn group_indices(indices: &[usize]) -> Vec<Range<usize>> {
    if indices.is_empty() {
        return Vec::new();
    }

    let mut ret = Vec::new();
    let mut itr = indices.iter();
    // we've already handled the case of an empty input, it is safe to unwrap
    let mut start = *itr.next().unwrap();

    for (i, x) in itr.enumerate() {
        if x - indices[i] != 1 {
            let end = indices[i];
            ret.push(Range {
                start,
                end: end + 1,
            });
            start = *x;
        }
    }
    // again, the case of an empty input is already handled so it is safe to unwrap here too
    ret.push(Range {
        start,
        end: indices.last().unwrap() + 1,
    });

    debug!("Match indices {:?} -> ranges {:?}", indices, ret);

    ret
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cli::output_options::FormattingOptions;
    use atty::Stream;
    use yansi::{Color, Style};

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
                "{}st\ntes{}\n{}s{}",
                if atty::is(Stream::Stdout) {
                    Paint::red("te").bold().to_string()
                } else {
                    String::from("te")
                },
                if atty::is(Stream::Stdout) {
                    Paint::red('t').bold().to_string()
                } else {
                    String::from("t")
                },
                if atty::is(Stream::Stdout) {
                    Paint::red("te").bold().to_string()
                } else {
                    String::from("te")
                },
                if atty::is(Stream::Stdout) {
                    Paint::red('t').bold().to_string()
                } else {
                    String::from("t")
                },
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
                "{}{}{}st\n{}{}tes{}\n{}{}{}s{}",
                Paint::green("42"),
                Paint::cyan(':'),
                Paint::red("te").bold(),
                Paint::green("100500"),
                Paint::cyan(':'),
                Paint::red('t').bold(),
                Paint::green("13"),
                Paint::cyan(':'),
                Paint::red("te").bold(),
                Paint::red('t').bold()
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
                "{}{}{}st\n{}{}tes{}\n{}{}{}s{}",
                Paint::magenta("First"),
                Paint::cyan(':'),
                Paint::red("te").bold(),
                Paint::magenta("Second"),
                Paint::cyan(':'),
                Paint::red('t').bold(),
                Paint::magenta("Third"),
                Paint::cyan(':'),
                Paint::red("te").bold(),
                Paint::red('t').bold(),
            )
        )
    }

    #[test]
    fn results_output_options_context() {
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
                    context: Context {
                        before: 1,
                        after: 2,
                    },
                    ..Default::default()
                }
            ),
            format!(
                "{}{}{}st\n{}{}tes{}\n{}{}{}s{}",
                Paint::magenta("First"),
                Paint::cyan(':'),
                Paint::red("te").bold(),
                Paint::magenta("Second"),
                Paint::cyan(':'),
                Paint::red('t').bold(),
                Paint::magenta("Third"),
                Paint::cyan(':'),
                Paint::red("te").bold(),
                Paint::red('t').bold(),
            )
        )
    }

    #[test]
    fn results_output_options_formatting_off() {
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
                    line_number: true,
                    context: Context {
                        before: 1,
                        after: 2
                    },
                    formatting: Formatting::Off
                }
            ),
            "First:42:test\nSecond:100500:test\nThird:13:test"
        )
    }

    #[test]
    fn results_output_options_formatting_plain() {
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
                    line_number: true,
                    context: Context {
                        before: 1,
                        after: 2
                    },
                    formatting: Formatting::On(FormattingOptions::plain())
                }
            ),
            "First:42:test\nSecond:100500:test\nThird:13:test"
        )
    }

    #[test]
    fn results_output_options_formatting_custom() {
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
                    line_number: true,
                    context: Context {
                        before: 1,
                        after: 2
                    },
                    formatting: Formatting::On(FormattingOptions {
                        selected_match: Style::new(Color::Green),
                        line_number: Style::new(Color::Cyan),
                        file_name: Style::new(Color::Cyan),
                        separator: Style::new(Color::Fixed(50)),
                        selected_line: Style::new(Color::RGB(127, 127, 127)).dimmed(),
                        context: Style::new(Color::RGB(127, 127, 127)).dimmed(),
                    })
                }
            ),
            format!(
                "{}{}{}{}{}{}\n{}{}{}{}{}{}\n{}{}{}{}{}{}{}",
                Paint::cyan("First"),
                Paint::fixed(50, ':'),
                Paint::cyan("42"),
                Paint::fixed(50, ':'),
                Paint::green("te"),
                Paint::rgb(127, 127, 127, "st").dimmed(),
                Paint::cyan("Second"),
                Paint::fixed(50, ':'),
                Paint::cyan("100500"),
                Paint::fixed(50, ':'),
                Paint::rgb(127, 127, 127, "tes").dimmed(),
                Paint::green('t'),
                Paint::cyan("Third"),
                Paint::fixed(50, ':'),
                Paint::cyan("13"),
                Paint::fixed(50, ':'),
                Paint::green("te"),
                Paint::rgb(127, 127, 127, 's').dimmed(),
                Paint::green('t')
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
                    file_name: true,
                    context: Context {
                        before: 1,
                        after: 2
                    },
                    formatting: Formatting::On(FormattingOptions {
                        selected_match: Style::new(Color::RGB(100, 150, 200))
                            .bg(Color::Yellow)
                            .italic(),
                        ..Default::default()
                    })
                }
            ),
            format!(
                "{}{}{}{}{}st\n{}{}{}{}tes{}\n{}{}{}{}{}s{}",
                Paint::magenta("First"),
                Paint::cyan(':'),
                Paint::green("42"),
                Paint::cyan(':'),
                Paint::rgb(100, 150, 200, "te").bg(Color::Yellow).italic(),
                Paint::magenta("Second"),
                Paint::cyan(':'),
                Paint::green("100500"),
                Paint::cyan(':'),
                Paint::rgb(100, 150, 200, 't').bg(Color::Yellow).italic(),
                Paint::magenta("Third"),
                Paint::cyan(':'),
                Paint::green("13"),
                Paint::cyan(':'),
                Paint::rgb(100, 150, 200, "te").bg(Color::Yellow).italic(),
                Paint::rgb(100, 150, 200, 't').bg(Color::Yellow).italic(),
            )
        )
    }

    #[test]
    fn no_results_output_options_default() {
        let results = vec![];
        assert_eq!(format_results(&results, &OutputOptions::default()), "")
    }

    #[test]
    fn no_results_output_options_all_options() {
        let results = vec![];
        assert_eq!(
            format_results(
                &results,
                &OutputOptions {
                    line_number: true,
                    file_name: true,
                    context: Context {
                        before: 1,
                        after: 2
                    },
                    formatting: Formatting::On(FormattingOptions {
                        selected_match: Style::new(Color::RGB(100, 150, 200))
                            .bg(Color::Yellow)
                            .italic(),
                        ..Default::default()
                    })
                }
            ),
            ""
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
