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
            let line_number = line_number.and_then(|l| Some(l - matches.len() + index + 1));
            ret.push_str(&format_context_line(
                context_line,
                file_name,
                &line_number,
                formatting,
            ));
            ret.push('\n');
        }

        ret.push_str(&format_selected_line(
            matching_line,
            fuzzy_match,
            file_name,
            line_number,
            formatting,
        ));
        ret.push('\n');

        for (index, context_line) in context_after.iter().enumerate() {
            let line_number = line_number.and_then(|l| Some(l + index + 1));
            ret.push_str(&format_context_line(
                context_line,
                file_name,
                &line_number,
                formatting,
            ));
            ret.push('\n');
        }
    }

    ret
}

fn format_context_line(
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

fn format_line_prefix(
    file_name: &Option<String>,
    line_number: &Option<usize>,
    formatting: &Formatting,
) -> Option<String> {
    let mut result = None;
    let options = formatting.options();

    if let Some(file_name) = file_name {
        let result = result.get_or_insert(String::new());
        result.push_str(&format_one_piece(file_name, options.map(|o| o.file_name)));
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

fn format_one_piece(s: &str, style: Option<Style>) -> String {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::cli::formatting::FormattingOptions;
    use atty::Stream;
    use yansi::Color;

    #[test]
    fn results_output_minimal_default() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
        ];
        assert_eq!(
            format_results(&results, &Formatting::On(FormattingOptions::default())),
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
    fn results_output_minimal_off() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
        ];
        assert_eq!(
            format_results(&results, &Formatting::Off),
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
    fn results_output_selected_line_default() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
        ];
        assert_eq!(
            format_results(&results, &Formatting::On(FormattingOptions::default())),
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
    fn results_output_selected_line_off() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
        ];
        assert_eq!(
            format_results(&results, &Formatting::Off),
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
    fn results_output_selected_line_custom() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
        ];
        assert_eq!(
            format_results(
                &results,
                &Formatting::On(FormattingOptions {
                    selected_line: Style::new(Color::Green),
                    ..Default::default()
                })
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
    fn results_output_line_number_default() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: Some(42),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: Some(100500),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: None,
                line_number: Some(13),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
        ];
        assert_eq!(
            format_results(&results, &Formatting::On(FormattingOptions::default())),
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
    fn results_output_line_number_off() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: Some(42),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: Some(100500),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: None,
                line_number: Some(13),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
        ];
        assert_eq!(
            format_results(&results, &Formatting::Off),
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
    fn results_output_line_number_custom() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: Some(42),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: Some(100500),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: None,
                line_number: Some(13),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
        ];
        assert_eq!(
            format_results(
                &results,
                &Formatting::On(FormattingOptions {
                    line_number: Style::new(Color::Cyan),
                    ..Default::default()
                })
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
    fn results_output_file_name_default() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: Some(String::from("First")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: Some(String::from("Second")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: Some(String::from("Third")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
        ];
        assert_eq!(
            format_results(&results, &Formatting::On(FormattingOptions::default())),
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
    fn results_output_file_name_off() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: Some(String::from("First")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: Some(String::from("Second")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: Some(String::from("Third")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
        ];
        assert_eq!(
            format_results(&results, &Formatting::Off),
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
    fn results_output_file_name_custom() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: Some(String::from("First")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: Some(String::from("Second")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: Some(String::from("Third")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
        ];
        assert_eq!(
            format_results(
                &results,
                &Formatting::On(FormattingOptions {
                    file_name: Style::new(Color::Cyan),
                    ..Default::default()
                })
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
    fn results_output_context_default() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![
                        String::from("first_before_one"),
                        String::from("first_before_two"),
                    ],
                    after: vec![
                        String::from("first_after_one"),
                        String::from("first_after_two"),
                    ],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![
                        String::from("second_before_one"),
                        String::from("second_before_two"),
                    ],
                    after: vec![
                        String::from("second_after_one"),
                        String::from("second_after_two"),
                    ],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![
                        String::from("third_before_one"),
                        String::from("third_before_two"),
                    ],
                    after: vec![
                        String::from("third_after_one"),
                        String::from("third_after_two"),
                    ],
                },
            },
        ];
        assert_eq!(
            format_results(&results, &Formatting::On(FormattingOptions::default())),
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
    fn results_output_context_off() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![
                        String::from("first_before_one"),
                        String::from("first_before_two"),
                    ],
                    after: vec![
                        String::from("first_after_one"),
                        String::from("first_after_two"),
                    ],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![
                        String::from("second_before_one"),
                        String::from("second_before_two"),
                    ],
                    after: vec![
                        String::from("second_after_one"),
                        String::from("second_after_two"),
                    ],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![
                        String::from("third_before_one"),
                        String::from("third_before_two"),
                    ],
                    after: vec![
                        String::from("third_after_one"),
                        String::from("third_after_two"),
                    ],
                },
            },
        ];
        assert_eq!(
            format_results(&results, &Formatting::Off),
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
    fn results_output_context_custom() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![
                        String::from("first_before_one"),
                        String::from("first_before_two"),
                    ],
                    after: vec![
                        String::from("first_after_one"),
                        String::from("first_after_two"),
                    ],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![
                        String::from("second_before_one"),
                        String::from("second_before_two"),
                    ],
                    after: vec![
                        String::from("second_after_one"),
                        String::from("second_after_two"),
                    ],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![
                        String::from("third_before_one"),
                        String::from("third_before_two"),
                    ],
                    after: vec![
                        String::from("third_after_one"),
                        String::from("third_after_two"),
                    ],
                },
            },
        ];
        assert_eq!(
            format_results(
                &results,
                &Formatting::On(FormattingOptions {
                    context: Style::new(Color::RGB(127, 127, 127)).dimmed(),
                    ..Default::default()
                })
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
    fn results_output_all_default() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: Some(String::from("First")),
                line_number: Some(42),
                context: Context {
                    before: vec![
                        String::from("first_before_one"),
                        String::from("first_before_two"),
                    ],
                    after: vec![
                        String::from("first_after_one"),
                        String::from("first_after_two"),
                    ],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: Some(String::from("Second")),
                line_number: Some(100500),
                context: Context {
                    before: vec![
                        String::from("second_before_one"),
                        String::from("second_before_two"),
                    ],
                    after: vec![
                        String::from("second_after_one"),
                        String::from("second_after_two"),
                    ],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: Some(String::from("Third")),
                line_number: Some(13),
                context: Context {
                    before: vec![
                        String::from("third_before_one"),
                        String::from("third_before_two"),
                    ],
                    after: vec![
                        String::from("third_after_one"),
                        String::from("third_after_two"),
                    ],
                },
            },
        ];
        assert_eq!(
            format_results(&results, &Formatting::On(FormattingOptions::default())),
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
    fn results_output_all_off() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: Some(String::from("First")),
                line_number: Some(42),
                context: Context {
                    before: vec![
                        String::from("first_before_one"),
                        String::from("first_before_two"),
                    ],
                    after: vec![
                        String::from("first_after_one"),
                        String::from("first_after_two"),
                    ],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: Some(String::from("Second")),
                line_number: Some(100500),
                context: Context {
                    before: vec![
                        String::from("second_before_one"),
                        String::from("second_before_two"),
                    ],
                    after: vec![
                        String::from("second_after_one"),
                        String::from("second_after_two"),
                    ],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: Some(String::from("Third")),
                line_number: Some(13),
                context: Context {
                    before: vec![
                        String::from("third_before_one"),
                        String::from("third_before_two"),
                    ],
                    after: vec![
                        String::from("third_after_one"),
                        String::from("third_after_two"),
                    ],
                },
            },
        ];
        assert_eq!(
            format_results(&results, &Formatting::Off),
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
    fn results_output_all_custom() {
        let results = vec![
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: Some(String::from("First")),
                line_number: Some(42),
                context: Context {
                    before: vec![
                        String::from("first_before_one"),
                        String::from("first_before_two"),
                    ],
                    after: vec![
                        String::from("first_after_one"),
                        String::from("first_after_two"),
                    ],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: Some(String::from("Second")),
                line_number: Some(100500),
                context: Context {
                    before: vec![
                        String::from("second_before_one"),
                        String::from("second_before_two"),
                    ],
                    after: vec![
                        String::from("second_after_one"),
                        String::from("second_after_two"),
                    ],
                },
            },
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                file_name: Some(String::from("Third")),
                line_number: Some(13),
                context: Context {
                    before: vec![
                        String::from("third_before_one"),
                        String::from("third_before_two"),
                    ],
                    after: vec![
                        String::from("third_after_one"),
                        String::from("third_after_two"),
                    ],
                },
            },
        ];
        assert_eq!(
            format_results(
                &results,
                &Formatting::On(FormattingOptions {
                    selected_match: Style::new(Color::Green),
                    line_number: Style::new(Color::Cyan),
                    file_name: Style::new(Color::Cyan),
                    separator: Style::new(Color::Fixed(50)),
                    selected_line: Style::new(Color::RGB(127, 127, 127)).dimmed(),
                    context: Style::new(Color::RGB(127, 127, 127)).dimmed(),
                })
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
    fn no_results_output_default() {
        let results = vec![];
        assert_eq!(
            format_results(&results, &Formatting::On(FormattingOptions::default())),
            ""
        );
    }

    #[test]
    fn no_results_output_off() {
        let results = vec![];
        assert_eq!(format_results(&results, &Formatting::Off), "");
    }

    #[test]
    fn no_results_output_custom() {
        let results = vec![];
        assert_eq!(
            format_results(
                &results,
                &Formatting::On(FormattingOptions {
                    selected_match: Style::new(Color::Green),
                    line_number: Style::new(Color::Cyan),
                    file_name: Style::new(Color::Cyan),
                    separator: Style::new(Color::Fixed(50)),
                    selected_line: Style::new(Color::RGB(127, 127, 127)).dimmed(),
                    context: Style::new(Color::RGB(127, 127, 127)).dimmed(),
                })
            ),
            ""
        )
    }
}
