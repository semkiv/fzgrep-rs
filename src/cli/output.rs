use crate::{
    cli::formatting::Formatting,
    matching_results::result::{Context, MatchingResult},
};
use log::debug;
use std::ops::Range;
use vscode_fuzzy_score_rs::FuzzyMatch;
use yansi::{Paint, Style};

/// Formats supplied `matches` into a rich text string.
///
/// When grepping files the format is as follows:
/// ```text
/// <filename>:<line-number>:<colored-matching-line>
/// ```
/// where `colored-matching-line` is a matching line with matching characters painted blue.
/// Whether `<filename>` and `<line-number>` are printed depends on `options`.
///
pub(crate) fn format_results(matches: &[MatchingResult], formatting: &Formatting) -> String {
    let mut ret = String::new();
    for m in matches.iter() {
        let MatchingResult {
            matching_line,
            fuzzy_match,
            file_name,
            line_number,
            context:
                Context {
                    before: context_before,
                    after: context_after,
                },
        } = m;

        for (index, context_line) in context_before.iter().enumerate() {
            ret.push_str(&format_context_line(
                context_line,
                file_name,
                &line_number.and_then(|l| Some(l - matches.len() + index)),
                formatting,
            ));
            ret.push('\n');
        }

        ret.push_str(&format_selected_line(
            &matching_line,
            fuzzy_match,
            file_name,
            line_number,
            formatting,
        ));
        ret.push('\n');

        for (index, context_line) in context_after.iter().enumerate() {
            ret.push_str(&format_context_line(
                context_line,
                file_name,
                &line_number.and_then(|l| Some(l + index)),
                formatting,
            ));
            ret.push('\n');
        }
    }

    ret
}

const fn format_context_line(
    content: &str,
    file_name: &Option<String>,
    line_number: &Option<usize>,
    formatting: &Formatting,
) -> String {
    let mut result = String::new();

    if let Some(prefix) = format_line_prefix(file_name, line_number, formatting) {
        result.push_str(&prefix);
    }

    result.push_str(&format_one_piece(
        content,
        formatting.options().map(|o| o.context),
    ));

    result
}

fn format_selected_line(
    content: &str,
    fuzzy_match: &FuzzyMatch,
    file_name: &Option<String>,
    line_number: &Option<usize>,
    formatting: &Formatting,
) -> String {
    let mut result = String::new();

    if let Some(prefix) = format_line_prefix(file_name, line_number, formatting) {
        result.push_str(&prefix);
    }

    let options = formatting.options();
    let mut str_itr = content.chars();
    let mut previous_range_end = 0;
    for range in group_indices(fuzzy_match.positions()) {
        let preceding_non_match = str_itr
            .by_ref()
            .take(range.start - previous_range_end)
            .collect::<String>();
        // The check is needed because `yansi::Paint` inserts formatting sequence even for empty strings.
        // Visually it makes no difference, but there are extra characters in the output,
        // making it harder to validate and compare results.
        if !preceding_non_match.is_empty() {
            result.push_str(&format_one_piece(
                &preceding_non_match,
                options.map(|o| o.selected_line),
            ))
        }

        let matching_part = str_itr
            .by_ref()
            .take(range.end - range.start)
            .collect::<String>();
        result.push_str(&format_one_piece(
            &matching_part,
            options.map(|o| o.selected_match),
        ));

        previous_range_end = range.end;
    }

    let remaining_non_match = str_itr.collect::<String>();
    // The check is needed because `yansi::Paint` inserts formatting sequence even for empty strings.
    // Visually it makes no difference, but there are extra characters in the output,
    // making it harder to validate and compare results.
    if !remaining_non_match.is_empty() {
        result.push_str(&format_one_piece(
            &remaining_non_match,
            options.map(|o| o.selected_line),
        ));
    }

    result
}

const fn format_line_prefix(
    file_name: &Option<String>,
    line_number: &Option<usize>,
    formatting: &Formatting,
) -> Option<String> {
    let mut result = None;
    let options = formatting.options();

    if let Some(file_name) = file_name {
        let result = result.get_or_insert(String::new());
        result.push_str(&format_one_piece(&file_name, options.map(|o| o.file_name)));
        result.push_str(&format_one_piece(":", options.map(|o| o.separator)));
    }

    if let Some(line_number) = line_number {
        let result = result.get_or_insert(String::new());
        result.push_str(&format_one_piece(
            &line_number.to_string(),
            options.map(|o| o.line_number),
        ));
        result.push_str(&format_one_piece(":", options.map(|o| o.separator)));
    }

    result
}

const fn format_one_piece(s: &str, style: Option<Style>) -> String {
    match style {
        Some(style) => Paint::new(s).with_style(style).to_string(),
        None => s.to_string(),
    }
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

mod test {
    use super::*;

    #[test]
    fn results_output_options_default() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                location: Location {
                    file_name: String::from("First"),
                    line_number: 42,
                },
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
}
