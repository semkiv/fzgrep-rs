use crate::cli::formatting::Formatting;
use crate::matching_results::result::{Context, MatchProperties};

use log::debug;
use std::ops::Range;
use vscode_fuzzy_score_rs::FuzzyMatch;
use yansi::{Paint as _, Style};

/// Formats supplied `matches` into a rich text string.
///
/// When grepping files the format is as follows:
/// ```text
/// <filename>:<line-number>:<colored-matching-line>
/// ```
/// where `colored-matching-line` is a matching line with matching characters painted blue.
/// Whether `<filename>` and `<line-number>` are printed depends on `options`.
///
pub(crate) fn format_results(matches: &[MatchProperties], formatting: &Formatting) -> String {
    let mut ret = String::new();
    for match_props in matches {
        let MatchProperties {
            matching_line,
            fuzzy_match,
            file_name,
            line_number,
            context:
                Context {
                    before: context_before,
                    after: context_after,
                },
        } = match_props;
        let file_name = file_name.as_ref();
        let line_number = line_number.as_ref().copied();

        for (index, context_line) in context_before.iter().enumerate() {
            #[expect(
                clippy::arithmetic_side_effects,
                reason = "`index` comes from enumerating `context_before` so it cannot be greater than `context_before.len()`"
            )]
            let offset = context_before.len() - index;
            let line_number = line_number.map(|num| {
                #[expect(
                    clippy::unwrap_used,
                    reason = "Context length and thus `offset` must not exceed line number. If it does it's a bug in the code that collects context (`result.rs`)"
                )]
                num.checked_sub(offset).unwrap()
            });
            ret.push_str(&format_context_line(
                context_line,
                file_name,
                line_number,
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
            let line_number = line_number.map(|num| num.wrapping_add(index).wrapping_add(1));
            ret.push_str(&format_context_line(
                context_line,
                file_name,
                line_number,
                formatting,
            ));
            ret.push('\n');
        }
    }

    ret
}

fn format_context_line(
    content: &str,
    file_name: Option<&String>,
    line_number: Option<usize>,
    formatting: &Formatting,
) -> String {
    let mut result = String::new();

    if let Some(prefix) = format_line_prefix(file_name, line_number, formatting) {
        result.push_str(&prefix);
    }

    result.push_str(&format_one_piece(
        content,
        formatting.options().map(|styleset| styleset.context),
    ));

    result
}

fn format_selected_line(
    content: &str,
    fuzzy_match: &FuzzyMatch,
    file_name: Option<&String>,
    line_number: Option<usize>,
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
            .take(
                #[expect(
                    clippy::unwrap_used,
                    reason = "The range is not supposed to start before the previous one ends. If it happens, it's a bug in the indices grouping code"
                )]
                range.start.checked_sub(previous_range_end).unwrap()
            )
            .collect::<String>();
        // The check is needed because `yansi::Paint` inserts formatting sequence even for empty strings.
        // Visually it makes no difference, but there are extra characters in the output,
        // making it harder to validate and compare results.
        if !preceding_non_match.is_empty() {
            result.push_str(&format_one_piece(
                &preceding_non_match,
                options.map(|styleset| styleset.selected_line),
            ));
        }

        let matching_part = str_itr
            .by_ref()
            .take(
                #[expect(
                    clippy::unwrap_used,
                    reason = "The range is not supposed to end before it starts. If it happens, it's a bug in the indices grouping code"
                )]
                range.end.checked_sub(range.start).unwrap()
            )
            .collect::<String>();
        result.push_str(&format_one_piece(
            &matching_part,
            options.map(|styleset| styleset.selected_match),
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
            options.map(|styleset| styleset.selected_line),
        ));
    }

    result
}

fn format_line_prefix(
    file_name: Option<&String>,
    line_number: Option<usize>,
    formatting: &Formatting,
) -> Option<String> {
    let mut result = None;
    let options = formatting.options();

    if let Some(file_name) = file_name {
        let result = result.get_or_insert(String::new());
        result.push_str(&format_one_piece(
            file_name,
            options.map(|styleset| styleset.file_name),
        ));
        result.push_str(&format_one_piece(
            ":",
            options.map(|styleset| styleset.separator),
        ));
    }

    if let Some(line_number) = line_number {
        let result = result.get_or_insert(String::new());
        result.push_str(&format_one_piece(
            &line_number.to_string(),
            options.map(|styleset| styleset.line_number),
        ));
        result.push_str(&format_one_piece(
            ":",
            options.map(|styleset| styleset.separator),
        ));
    }

    result
}

fn format_one_piece(piece: &str, style: Option<Style>) -> String {
    style.map_or_else(|| piece.to_owned(), |style| piece.paint(style).to_string())
}

fn group_indices(indices: &[usize]) -> Vec<Range<usize>> {
    if indices.is_empty() {
        return Vec::new();
    }

    let mut ret = Vec::new();
    let make_range = |first_idx: usize, last_idx: usize| {
        #[expect(
            clippy::expect_used,
            reason = "We can no longer guarantee that the range starts before it ends otherwise"
        )]
        let one_past_last_idx = last_idx
            .checked_add(1)
            .expect("Integer overflow occured when constructing a range");
        Range {
            start: first_idx,
            end: one_past_last_idx,
        }
    };
    let mut itr = indices.iter();
    #[expect(
        clippy::unwrap_used,
        reason = "The case of an empty input is already handled"
    )]
    let mut range_start = *itr.next().unwrap();
    let mut prev_idx = range_start;
    for idx in itr {
        #[expect(
            clippy::expect_used,
            reason = "Indices must be monothonic. If they are not, it's a bug in the fuzzy matching lib"
        )]
        let diff = idx.checked_sub(prev_idx).expect(
            "Indices of matching characters are not monothonic - a bug in `vscode-fuzzy-score-rs`?",
        );
        if diff > 1 {
            ret.push(make_range(range_start, prev_idx));
            range_start = *idx;
        }

        prev_idx = *idx;
    }

    ret.push(make_range(range_start, prev_idx));

    debug!("Match indices {indices:?} -> ranges {ret:?}");

    ret
}

#[cfg(test)]
mod test {
    #![expect(clippy::too_many_lines, reason = "It's tests, who cares?")]

    use super::*;
    use crate::cli::formatting::StyleSet;

    #[test]
    fn results_output_selected_match_default() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
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
            format_results(&results, &Formatting::On(StyleSet::default())),
            format!(
                "{}st\n\
                tes{}\n\
                {}s{}\n",
                "te".red().bold(),
                't'.red().bold(),
                "te".red().bold(),
                't'.red().bold(),
            )
        );
    }

    #[test]
    fn results_output_selected_match_off() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
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
            "test\n\
            test\n\
            test\n"
        );
    }

    #[test]
    fn results_output_selected_match_custom() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
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
                &Formatting::On(StyleSet {
                    selected_match: Style::new().yellow(),
                    ..Default::default()
                })
            ),
            format!(
                "{}st\n\
                tes{}\n\
                {}s{}\n",
                "te".yellow(),
                't'.yellow(),
                "te".yellow(),
                't'.yellow(),
            )
        );
    }

    #[test]
    fn results_output_selected_line_default() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
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
            format_results(&results, &Formatting::On(StyleSet::default())),
            format!(
                "{}st\n\
                tes{}\n\
                {}s{}\n",
                "te".red().bold(),
                't'.red().bold(),
                "te".red().bold(),
                't'.red().bold(),
            )
        );
    }

    #[test]
    fn results_output_selected_line_off() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
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
            "test\n\
            test\n\
            test\n"
        );
    }

    #[test]
    fn results_output_selected_line_custom() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
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
                &Formatting::On(StyleSet {
                    selected_line: Style::new().yellow(),
                    ..Default::default()
                })
            ),
            format!(
                "{}{}\n\
                {}{}\n\
                {}{}{}\n",
                "te".red().bold(),
                "st".yellow(),
                "tes".yellow(),
                't'.red().bold(),
                "te".red().bold(),
                's'.yellow(),
                't'.red().bold(),
            )
        );
    }

    #[test]
    fn results_output_line_number_default() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: Some(42),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: Some(100_500),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
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
            format_results(&results, &Formatting::On(StyleSet::default())),
            format!(
                "{}{}{}st\n\
                {}{}tes{}\n\
                {}{}{}s{}\n",
                "42".green(),
                ':'.cyan(),
                "te".red().bold(),
                "100500".green(),
                ':'.cyan(),
                't'.red().bold(),
                "13".green(),
                ':'.cyan(),
                "te".red().bold(),
                't'.red().bold()
            )
        );
    }

    #[test]
    fn results_output_line_number_off() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: Some(42),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: Some(100_500),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
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
            "42:test\n\
            100500:test\n\
            13:test\n"
        );
    }

    #[test]
    fn results_output_line_number_custom() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: None,
                line_number: Some(42),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: None,
                line_number: Some(100_500),
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
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
                &Formatting::On(StyleSet {
                    line_number: Style::new().yellow(),
                    ..Default::default()
                })
            ),
            format!(
                "{}{}{}st\n\
                {}{}tes{}\n\
                {}{}{}s{}\n",
                "42".yellow(),
                ':'.cyan(),
                "te".red().bold(),
                "100500".yellow(),
                ':'.cyan(),
                't'.red().bold(),
                "13".yellow(),
                ':'.cyan(),
                "te".red().bold(),
                't'.red().bold()
            )
        );
    }

    #[test]
    fn results_output_file_name_default() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: Some(String::from("First")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: Some(String::from("Second")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
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
            format_results(&results, &Formatting::On(StyleSet::default())),
            format!(
                "{}{}{}st\n\
                {}{}tes{}\n\
                {}{}{}s{}\n",
                "First".magenta(),
                ':'.cyan(),
                "te".red().bold(),
                "Second".magenta(),
                ':'.cyan(),
                't'.red().bold(),
                "Third".magenta(),
                ':'.cyan(),
                "te".red().bold(),
                't'.red().bold(),
            )
        );
    }

    #[test]
    fn results_output_file_name_off() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: Some(String::from("First")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: Some(String::from("Second")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
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
            "First:test\n\
            Second:test\n\
            Third:test\n"
        );
    }

    #[test]
    fn results_output_file_name_custom() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: Some(String::from("First")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: Some(String::from("Second")),
                line_number: None,
                context: Context {
                    before: vec![],
                    after: vec![],
                },
            },
            MatchProperties {
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
                &Formatting::On(StyleSet {
                    file_name: Style::new().yellow(),
                    ..Default::default()
                })
            ),
            format!(
                "{}{}{}st\n\
                {}{}tes{}\n\
                {}{}{}s{}\n",
                "First".yellow(),
                ':'.cyan(),
                "te".red().bold(),
                "Second".yellow(),
                ':'.cyan(),
                't'.red().bold(),
                "Third".yellow(),
                ':'.cyan(),
                "te".red().bold(),
                't'.red().bold(),
            )
        );
    }

    #[test]
    fn results_output_context_default() {
        let results = vec![
            MatchProperties {
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
            MatchProperties {
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
            MatchProperties {
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
            format_results(&results, &Formatting::On(StyleSet::default())),
            format!(
                "first_before_one\n\
                first_before_two\n\
                {}st\n\
                first_after_one\n\
                first_after_two\n\
                second_before_one\n\
                second_before_two\n\
                tes{}\n\
                second_after_one\n\
                second_after_two\n\
                third_before_one\n\
                third_before_two\n\
                {}s{}\n\
                third_after_one\n\
                third_after_two\n",
                "te".red().bold(),
                't'.red().bold(),
                "te".red().bold(),
                't'.red().bold(),
            )
        );
    }

    #[test]
    fn results_output_context_off() {
        let results = vec![
            MatchProperties {
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
            MatchProperties {
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
            MatchProperties {
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
            "first_before_one\n\
            first_before_two\n\
            test\n\
            first_after_one\n\
            first_after_two\n\
            second_before_one\n\
            second_before_two\n\
            test\n\
            second_after_one\n\
            second_after_two\n\
            third_before_one\n\
            third_before_two\n\
            test\n\
            third_after_one\n\
            third_after_two\n",
        );
    }

    #[test]
    fn results_output_context_custom() {
        let results = vec![
            MatchProperties {
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
            MatchProperties {
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
            MatchProperties {
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
                &Formatting::On(StyleSet {
                    context: Style::new().rgb(127, 127, 127).dim(),
                    ..Default::default()
                })
            ),
            format!(
                "{}\n\
                {}\n\
                {}st\n\
                {}\n\
                {}\n\
                {}\n\
                {}\n\
                tes{}\n\
                {}\n\
                {}\n\
                {}\n\
                {}\n\
                {}s{}\n\
                {}\n\
                {}\n",
                "first_before_one".rgb(127, 127, 127).dim(),
                "first_before_two".rgb(127, 127, 127).dim(),
                "te".red().bold(),
                "first_after_one".rgb(127, 127, 127).dim(),
                "first_after_two".rgb(127, 127, 127).dim(),
                "second_before_one".rgb(127, 127, 127).dim(),
                "second_before_two".rgb(127, 127, 127).dim(),
                't'.red().bold(),
                "second_after_one".rgb(127, 127, 127).dim(),
                "second_after_two".rgb(127, 127, 127).dim(),
                "third_before_one".rgb(127, 127, 127).dim(),
                "third_before_two".rgb(127, 127, 127).dim(),
                "te".red().bold(),
                't'.red().bold(),
                "third_after_one".rgb(127, 127, 127).dim(),
                "third_after_two".rgb(127, 127, 127).dim(),
            )
        );
    }

    #[test]
    fn results_output_all_default() {
        let results = vec![
            MatchProperties {
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
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: Some(String::from("Second")),
                line_number: Some(100_500),
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
            MatchProperties {
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
            format_results(&results, &Formatting::On(StyleSet::default())),
            format!(
                "{}{}{}{}first_before_one\n\
                {}{}{}{}first_before_two\n\
                {}{}{}{}{}st\n\
                {}{}{}{}first_after_one\n\
                {}{}{}{}first_after_two\n\
                {}{}{}{}second_before_one\n\
                {}{}{}{}second_before_two\n\
                {}{}{}{}tes{}\n\
                {}{}{}{}second_after_one\n\
                {}{}{}{}second_after_two\n\
                {}{}{}{}third_before_one\n\
                {}{}{}{}third_before_two\n\
                {}{}{}{}{}s{}\n\
                {}{}{}{}third_after_one\n\
                {}{}{}{}third_after_two\n",
                // first before context line
                "First".magenta(),
                ':'.cyan(),
                "40".green(),
                ':'.cyan(),
                // second before context line
                "First".magenta(),
                ':'.cyan(),
                "41".green(),
                ':'.cyan(),
                // selected line
                "First".magenta(),
                ':'.cyan(),
                "42".green(),
                ':'.cyan(),
                "te".red().bold(),
                // first after context line
                "First".magenta(),
                ':'.cyan(),
                "43".green(),
                ':'.cyan(),
                // second after context line
                "First".magenta(),
                ':'.cyan(),
                "44".green(),
                ':'.cyan(),
                // first before context line
                "Second".magenta(),
                ':'.cyan(),
                "100498".green(),
                ':'.cyan(),
                // second before context line
                "Second".magenta(),
                ':'.cyan(),
                "100499".green(),
                ':'.cyan(),
                // selected line
                "Second".magenta(),
                ':'.cyan(),
                "100500".green(),
                ':'.cyan(),
                't'.red().bold(),
                // first after context line
                "Second".magenta(),
                ':'.cyan(),
                "100501".green(),
                ':'.cyan(),
                // second after context line
                "Second".magenta(),
                ':'.cyan(),
                "100502".green(),
                ':'.cyan(),
                // first before context line
                "Third".magenta(),
                ':'.cyan(),
                "11".green(),
                ':'.cyan(),
                // second before context line
                "Third".magenta(),
                ':'.cyan(),
                "12".green(),
                ':'.cyan(),
                // selected line
                "Third".magenta(),
                ':'.cyan(),
                "13".green(),
                ':'.cyan(),
                "te".red().bold(),
                't'.red().bold(),
                // first after context line
                "Third".magenta(),
                ':'.cyan(),
                "14".green(),
                ':'.cyan(),
                // second after context line
                "Third".magenta(),
                ':'.cyan(),
                "15".green(),
                ':'.cyan(),
            )
        );
    }

    #[test]
    fn results_output_all_off() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: Some(String::from("First")),
                line_number: Some(42),
                context: Context {
                    before: vec![String::from("first_before_one")],
                    after: vec![
                        String::from("first_after_one"),
                        String::from("first_after_two"),
                        String::from("first_after_three"),
                    ],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: Some(String::from("Second")),
                line_number: Some(100_500),
                context: Context {
                    before: vec![
                        String::from("second_before_one"),
                        String::from("second_before_two"),
                        String::from("second_before_three"),
                    ],
                    after: vec![String::from("second_after_one")],
                },
            },
            MatchProperties {
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
            "First:41:first_before_one\n\
            First:42:test\n\
            First:43:first_after_one\n\
            First:44:first_after_two\n\
            First:45:first_after_three\n\
            Second:100497:second_before_one\n\
            Second:100498:second_before_two\n\
            Second:100499:second_before_three\n\
            Second:100500:test\n\
            Second:100501:second_after_one\n\
            Third:11:third_before_one\n\
            Third:12:third_before_two\n\
            Third:13:test\n\
            Third:14:third_after_one\n\
            Third:15:third_after_two\n"
        );
    }

    #[test]
    fn results_output_all_custom() {
        let results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                file_name: Some(String::from("First")),
                line_number: Some(42),
                context: Context {
                    before: vec![String::from("first_before_one")],
                    after: vec![
                        String::from("first_after_one"),
                        String::from("first_after_two"),
                        String::from("first_after_three"),
                    ],
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                file_name: Some(String::from("Second")),
                line_number: Some(100_500),
                context: Context {
                    before: vec![
                        String::from("second_before_one"),
                        String::from("second_before_two"),
                        String::from("second_before_three"),
                    ],
                    after: vec![String::from("second_after_one")],
                },
            },
            MatchProperties {
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
                &Formatting::On(StyleSet {
                    selected_match: Style::new().yellow().italic(),
                    line_number: Style::new().cyan(),
                    file_name: Style::new().cyan(),
                    separator: Style::new().fixed(50),
                    selected_line: Style::new().rgb(127, 127, 127).dim(),
                    context: Style::new().rgb(127, 127, 127).dim(),
                })
            ),
            format!(
                "{}{}{}{}{}\n\
                {}{}{}{}{}{}\n\
                {}{}{}{}{}\n\
                {}{}{}{}{}\n\
                {}{}{}{}{}\n\
                {}{}{}{}{}\n\
                {}{}{}{}{}\n\
                {}{}{}{}{}\n\
                {}{}{}{}{}{}\n\
                {}{}{}{}{}\n\
                {}{}{}{}{}\n\
                {}{}{}{}{}\n\
                {}{}{}{}{}{}{}\n\
                {}{}{}{}{}\n\
                {}{}{}{}{}\n",
                // first before context line
                "First".cyan(),
                ':'.fixed(50),
                "41".cyan(),
                ':'.fixed(50),
                "first_before_one".rgb(127, 127, 127).dim(),
                // selected line
                "First".cyan(),
                ':'.fixed(50),
                "42".cyan(),
                ':'.fixed(50),
                "te".yellow().italic(),
                "st".rgb(127, 127, 127).dim(),
                // first after context line
                "First".cyan(),
                ':'.fixed(50),
                "43".cyan(),
                ':'.fixed(50),
                "first_after_one".rgb(127, 127, 127).dim(),
                // second after context line
                "First".cyan(),
                ':'.fixed(50),
                "44".cyan(),
                ':'.fixed(50),
                "first_after_two".rgb(127, 127, 127).dim(),
                // third after context line
                "First".cyan(),
                ':'.fixed(50),
                "45".cyan(),
                ':'.fixed(50),
                "first_after_three".rgb(127, 127, 127).dim(),
                // first before context line
                "Second".cyan(),
                ':'.fixed(50),
                "100497".cyan(),
                ':'.fixed(50),
                "second_before_one".rgb(127, 127, 127).dim(),
                // second before context line
                "Second".cyan(),
                ':'.fixed(50),
                "100498".cyan(),
                ':'.fixed(50),
                "second_before_two".rgb(127, 127, 127).dim(),
                // third before context line
                "Second".cyan(),
                ':'.fixed(50),
                "100499".cyan(),
                ':'.fixed(50),
                "second_before_three".rgb(127, 127, 127).dim(),
                // selected line
                "Second".cyan(),
                ':'.fixed(50),
                "100500".cyan(),
                ':'.fixed(50),
                "tes".rgb(127, 127, 127).dim(),
                't'.yellow().italic(),
                // first after context line
                "Second".cyan(),
                ':'.fixed(50),
                "100501".cyan(),
                ':'.fixed(50),
                "second_after_one".rgb(127, 127, 127).dim(),
                // first before context line
                "Third".cyan(),
                ':'.fixed(50),
                "11".cyan(),
                ':'.fixed(50),
                "third_before_one".rgb(127, 127, 127).dim(),
                // second before context line
                "Third".cyan(),
                ':'.fixed(50),
                "12".cyan(),
                ':'.fixed(50),
                "third_before_two".rgb(127, 127, 127).dim(),
                // selected line
                "Third".cyan(),
                ':'.fixed(50),
                "13".cyan(),
                ':'.fixed(50),
                "te".yellow().italic(),
                "s".rgb(127, 127, 127).dim(),
                't'.yellow().italic(),
                // first after context line
                "Third".cyan(),
                ':'.fixed(50),
                "14".cyan(),
                ':'.fixed(50),
                "third_after_one".rgb(127, 127, 127).dim(),
                // second after context line
                "Third".cyan(),
                ':'.fixed(50),
                "15".cyan(),
                ':'.fixed(50),
                "third_after_two".rgb(127, 127, 127).dim(),
            )
        );
    }

    #[test]
    fn no_results_output_default() {
        let results = vec![];
        assert_eq!(
            format_results(&results, &Formatting::On(StyleSet::default())),
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
                &Formatting::On(StyleSet {
                    selected_match: Style::new().green(),
                    line_number: Style::new().cyan(),
                    file_name: Style::new().cyan(),
                    separator: Style::new().fixed(50),
                    selected_line: Style::new().rgb(127, 127, 127).dim(),
                    context: Style::new().rgb(127, 127, 127).dim(),
                })
            ),
            ""
        );
    }
}
