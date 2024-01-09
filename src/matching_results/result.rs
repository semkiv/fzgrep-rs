use crate::matching_results::context_accumulators::SaturatingAccumulator;
use std::cmp::Ordering;
use vscode_fuzzy_score_rs::FuzzyMatch;

/// Stores a full result of matching.
///
#[derive(Debug)]
pub struct MatchingResult {
    /// The line that contains the match.
    ///
    pub matching_line: String,

    /// The properties of the match.
    ///
    pub fuzzy_match: FuzzyMatch,

    /// An optional file name (if file names tracking was requested).
    ///
    pub file_name: Option<String>,

    /// An optional line number (if line numbers tracking was requested).
    ///
    pub line_number: Option<usize>,

    /// Context surrounding the match.
    ///
    pub context: Context,
}

/// Context (surrounding lines) around a match
///
#[derive(Debug)]
pub struct Context {
    /// Lines preceding the matching line.
    ///
    pub before: Vec<String>,

    /// Lines following the matching line.
    ///
    pub after: Vec<String>,
}

/// Represents possible states of a matching result.
///
pub(crate) enum MatchingResultState {
    /// The result is complete and ready to use.
    ///
    Complete(MatchingResult),

    /// The result is incomplete (has only partial context).
    ///
    Incomplete(PartialMatchingResult),
}

/// A partial (with a partial context) matching result.
///
#[derive(Debug, PartialEq)]
pub(crate) struct PartialMatchingResult {
    /// The line that contains the match.
    ///
    matching_line: String,

    /// The properties of the match.
    ///
    fuzzy_match: FuzzyMatch,

    /// An optional file name (if file names tracking was requested).
    ///
    file_name: Option<String>,

    /// An optional line number (if line numbers tracking was requested).
    ///
    line_number: Option<usize>,

    /// Partial context (the trailing context is not fully accumulated).
    ///
    partial_context: PartialContext,
}

enum ContextState {
    Complete(Context),
    Incomplete(PartialContext),
}

#[derive(Debug, PartialEq)]
struct PartialContext {
    before: Vec<String>,
    after_accumulator: SaturatingAccumulator,
}

impl MatchingResultState {
    /// Creates a result state based on the parameters.
    /// Effectively the only case when it can return [`MatchingResultState::Complete`] is `after_context_size` being `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let matching_line = String::from("test");
    /// let fuzzy_match = vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap();
    /// let file_name = None;
    /// let line_number = None;
    /// let before_context = vec![String::from("line1"), String::from("line2")];///
    /// match MatchingResultState::new(
    ///     matching_line,
    ///     fuzzy_match,
    ///     file_name,
    ///     line_number,
    ///     before_context,
    ///     2,
    /// ) {
    ///     MatchingResultState::Complete(_) => unreachable!(),
    ///     MatchingResultState::Incomplete(partial_result) => {
    ///         assert_eq!(
    ///             partial_result,
    ///             PartialMatchingResult {
    ///                 matching_line: String::from("test"),
    ///                 fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
    ///                 file_name: None,
    ///                 line_number: None,
    ///                 partial_context: PartialContext {
    ///                     before: vec![String::from("line1"), String::from("line2")],
    ///                     after_accumulator: SaturatingAccumulator::new(2)
    ///                 },
    ///             }
    ///         )
    ///     }
    /// }
    /// ```
    ///
    /// ```
    /// let matching_line = String::from("test");
    /// let fuzzy_match = vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap();
    /// let file_name = None;
    /// let line_number = None;
    /// let before_context = vec![String::from("line1"), String::from("line2")];
    ///
    /// match MatchingResultState::new(
    ///     matching_line,
    ///     fuzzy_match,
    ///     file_name,
    ///     line_number,
    ///     before_context,
    ///     0,
    /// ) {
    ///     MatchingResultState::Complete(result) => {
    ///         assert_eq!(
    ///             result,
    ///             MatchingResult {
    ///                 matching_line: String::from("test"),
    ///                 fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
    ///                 file_name: None,
    ///                 line_number: None,
    ///                 context: Context {
    ///                     before: vec![String::from("line1"), String::from("line2")],
    ///                     after: vec![],
    ///                 },
    ///             }
    ///         )
    ///     }
    ///     MatchingResultState::Incomplete(_) => unreachable!(),
    /// }
    /// ```
    ///
    pub(crate) fn new(
        matching_line: String,
        fuzzy_match: FuzzyMatch,
        file_name: Option<String>,
        line_number: Option<usize>,
        before_context: Vec<String>,
        after_context_size: usize,
    ) -> Self {
        match ContextState::new(before_context, after_context_size) {
            ContextState::Complete(context) => Self::Complete(MatchingResult {
                matching_line,
                fuzzy_match,
                file_name,
                line_number,
                context,
            }),
            ContextState::Incomplete(partial_context) => Self::Incomplete(PartialMatchingResult {
                matching_line,
                fuzzy_match,
                file_name,
                line_number,
                partial_context,
            }),
        }
    }
}

impl PartialMatchingResult {
    /// Feeds a line into a partial matching result. With each line fed a partial result may become complete
    /// (depending on the state of the underlying context accumulator).
    ///
    /// # Examples
    ///
    /// ```
    /// let matching_line = String::from("test");
    /// let fuzzy_match = vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap();
    /// let file_name = None;
    /// let line_number = None;
    /// let before_context = vec![String::from("line1"), String::from("line2")];
    /// match MatchingResultState::new(
    ///     matching_line,
    ///     fuzzy_match,
    ///     file_name,
    ///     line_number,
    ///     before_context,
    ///     2,
    /// ) {
    ///     // the context size of 2 is expected so right after the creation the state is incomplete
    ///     MatchingResultState::Incomplete(partial_result) => {
    ///         match partial_result.feed(String::from("line3")) {
    ///             // after one line has been push the context is still incomplete
    ///             MatchingResultState::Incomplete(partial_result) => {
    ///                 match partial_result.feed(String::from("line4")) {
    ///                     // finally after the second line has been pushed we have a complete context and thus result
    ///                     MatchingResultState::Complete(result) => {
    ///                         assert_eq!(
    ///                             result,
    ///                             MatchingResult {
    ///                                 matching_line: String::from("test"),
    ///                                 fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match(
    ///                                     "test", "test"
    ///                                 )
    ///                                 .unwrap(),
    ///                                 file_name: None,
    ///                                 line_number: None,
    ///                                 context: Context {
    ///                                     before: vec![
    ///                                         String::from("line1"),
    ///                                         String::from("line2")
    ///                                     ],
    ///                                     after: vec![
    ///                                         String::from("line3"),
    ///                                         String::from("line4")
    ///                                     ],
    ///                                 },
    ///                             }
    ///                         );
    ///                     }
    ///                     MatchingResultState::Incomplete(_) => unreachable!(),
    ///                 }
    ///             },
    ///             MatchingResultState::Complete(_) => unreachable!(),
    ///         }
    ///     },
    ///     MatchingResultState::Complete(_) => unreachable!(),
    /// }
    /// ```
    ///
    pub(crate) fn feed(self, line: String) -> MatchingResultState {
        match self.partial_context.feed(line) {
            ContextState::Complete(context) => MatchingResultState::Complete(MatchingResult {
                matching_line: self.matching_line,
                fuzzy_match: self.fuzzy_match,
                file_name: self.file_name,
                line_number: self.line_number,
                context,
            }),
            ContextState::Incomplete(partial_context) => {
                MatchingResultState::Incomplete(PartialMatchingResult {
                    matching_line: self.matching_line,
                    fuzzy_match: self.fuzzy_match,
                    file_name: self.file_name,
                    line_number: self.line_number,
                    partial_context,
                })
            }
        }
    }

    /// Forcibly turns a partial result into a complete one.
    /// This is useful when accumulator reaches the end of file and cannot possibly accumulate more lines.
    ///
    /// # Examples
    ///
    /// ```
    /// let matching_line = String::from("test");
    /// let fuzzy_match = vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap();
    /// let file_name = None;
    /// let line_number = None;
    /// let before_context = vec![String::from("line1"), String::from("line2")];
    /// match MatchingResultState::new(
    ///     matching_line,
    ///     fuzzy_match,
    ///     file_name,
    ///     line_number,
    ///     before_context,
    ///     2,
    /// ) {
    ///     // the context size of 2 is expected so right after the creation the state is incomplete
    ///     MatchingResultState::Incomplete(partial_result) => {
    ///         match partial_result.feed(String::from("line3")) {
    ///             // after one line has been push the context is still incomplete
    ///             MatchingResultState::Incomplete(partial_result) => {
    ///                 // let's complete it forcibly
    ///                 let result = partial_result.complete();
    ///                 assert_eq!(
    ///                     result,
    ///                     MatchingResult {
    ///                         matching_line: String::from("test"),
    ///                         fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
    ///                         file_name: None,
    ///                         line_number: None,
    ///                         context: Context {
    ///                             before: vec![String::from("line1"), String::from("line2")],
    ///                             after: vec![String::from("line3")],
    ///                         },
    ///                     }
    ///                 );
    ///             },
    ///             MatchingResultState::Complete(_) => unreachable!(),
    ///         }
    ///     },
    ///     MatchingResultState::Complete(_) => unreachable!(),
    /// }
    /// ```
    ///
    pub(crate) fn complete(self) -> MatchingResult {
        MatchingResult {
            matching_line: self.matching_line,
            fuzzy_match: self.fuzzy_match,
            file_name: self.file_name,
            line_number: self.line_number,
            context: self.partial_context.complete(),
        }
    }
}

impl PartialEq for MatchingResult {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_match.eq(&other.fuzzy_match)
    }
}

impl PartialOrd for MatchingResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for MatchingResult {}

impl Ord for MatchingResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fuzzy_match.cmp(&other.fuzzy_match)
    }
}

impl ContextState {
    fn new(before: Vec<String>, after_size: usize) -> ContextState {
        let accumulator = SaturatingAccumulator::new(after_size);
        if accumulator.is_saturated() {
            Self::Complete(Context {
                before,
                after: accumulator.consume(),
            })
        } else {
            Self::Incomplete(PartialContext {
                before,
                after_accumulator: accumulator,
            })
        }
    }
}

impl PartialContext {
    fn feed(mut self, line: String) -> ContextState {
        self.after_accumulator.feed(line);
        if self.after_accumulator.is_saturated() {
            ContextState::Complete(Context {
                before: self.before,
                after: self.after_accumulator.consume(),
            })
        } else {
            ContextState::Incomplete(self)
        }
    }

    fn complete(self) -> Context {
        Context {
            before: self.before,
            after: self.after_accumulator.consume(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matching_result_state_constructor_complete() {
        let matching_line = String::from("test");
        let fuzzy_match = vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap();
        let file_name = None;
        let line_number = None;
        let before_context = vec![String::from("line1"), String::from("line2")];

        match MatchingResultState::new(
            matching_line,
            fuzzy_match,
            file_name,
            line_number,
            before_context,
            0,
        ) {
            MatchingResultState::Complete(result) => {
                assert_eq!(
                    result,
                    MatchingResult {
                        matching_line: String::from("test"),
                        fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
                        file_name: None,
                        line_number: None,
                        context: Context {
                            before: vec![String::from("line1"), String::from("line2")],
                            after: vec![],
                        },
                    }
                )
            }
            MatchingResultState::Incomplete(_) => unreachable!(),
        }
    }

    #[test]
    fn matching_result_state_constructor_incomplete() {
        let matching_line = String::from("test");
        let fuzzy_match = vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap();
        let file_name = None;
        let line_number = None;
        let before_context = vec![String::from("line1"), String::from("line2")];

        match MatchingResultState::new(
            matching_line,
            fuzzy_match,
            file_name,
            line_number,
            before_context,
            2,
        ) {
            MatchingResultState::Complete(_) => unreachable!(),
            MatchingResultState::Incomplete(partial_result) => {
                assert_eq!(
                    partial_result,
                    PartialMatchingResult {
                        matching_line: String::from("test"),
                        fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
                        file_name: None,
                        line_number: None,
                        partial_context: PartialContext {
                            before: vec![String::from("line1"), String::from("line2")],
                            after_accumulator: SaturatingAccumulator::new(2)
                        },
                    }
                )
            }
        }
    }

    #[test]
    fn partial_matching_result_feed() {
        let matching_line = String::from("test");
        let fuzzy_match = vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap();
        let file_name = None;
        let line_number = None;
        let before_context = vec![String::from("line1"), String::from("line2")];
        match MatchingResultState::new(
            matching_line,
            fuzzy_match,
            file_name,
            line_number,
            before_context,
            2,
        ) {
            MatchingResultState::Incomplete(partial_result) => {
                match partial_result.feed(String::from("line3")) {
                    MatchingResultState::Incomplete(partial_result) => {
                        match partial_result.feed(String::from("line4")) {
                            MatchingResultState::Complete(result) => {
                                assert_eq!(
                                    result,
                                    MatchingResult {
                                        matching_line: String::from("test"),
                                        fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match(
                                            "test", "test"
                                        )
                                        .unwrap(),
                                        file_name: None,
                                        line_number: None,
                                        context: Context {
                                            before: vec![
                                                String::from("line1"),
                                                String::from("line2")
                                            ],
                                            after: vec![
                                                String::from("line3"),
                                                String::from("line4")
                                            ],
                                        },
                                    }
                                );
                            }
                            MatchingResultState::Incomplete(_) => unreachable!(),
                        }
                    }
                    MatchingResultState::Complete(_) => unreachable!(),
                }
            }
            MatchingResultState::Complete(_) => unreachable!(),
        }
    }

    #[test]
    fn partial_matching_result_complete() {
        let mut partial_result = PartialMatchingResult {
            matching_line: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
            file_name: None,
            line_number: None,
            partial_context: PartialContext {
                before: vec![String::from("line1"), String::from("line2")],
                after_accumulator: SaturatingAccumulator::new(2),
            },
        };
        partial_result
            .partial_context
            .after_accumulator
            .feed(String::from("line3"));
        let result = partial_result.complete();
        assert_eq!(
            result,
            MatchingResult {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
                file_name: None,
                line_number: None,
                context: Context {
                    before: vec![String::from("line1"), String::from("line2")],
                    after: vec![String::from("line3")],
                },
            }
        )
    }

    #[test]
    fn matching_result_comparisons_ne() {
        let m1 = MatchingResult {
            matching_line: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
            file_name: None,
            line_number: Some(42),
            context: Context {
                before: vec![String::from("before")],
                after: vec![String::from("after")],
            },
        };
        let m2 = MatchingResult {
            matching_line: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tes", "test").unwrap(),
            file_name: None,
            line_number: Some(42),
            context: Context {
                before: vec![String::from("before")],
                after: vec![String::from("after")],
            },
        };
        assert_ne!(m1, m2);
    }

    #[test]
    fn matching_result_comparisons_eq() {
        let m1 = MatchingResult {
            matching_line: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
            file_name: Some(String::from("test.txt")),
            line_number: None,
            context: Context {
                before: vec![String::from("before1")],
                after: vec![String::from("after1")],
            },
        };
        let m2 = MatchingResult {
            matching_line: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
            file_name: None,
            line_number: Some(42),
            context: Context {
                before: vec![String::from("before2")],
                after: vec![String::from("after2")],
            },
        };
        assert_eq!(m1, m2);
    }

    #[test]
    fn matching_result_comparisons_lt() {
        let m1 = MatchingResult {
            matching_line: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test1").unwrap(),
            file_name: Some(String::from("test.txt")),
            line_number: None,
            context: Context {
                before: vec![String::from("before1")],
                after: vec![String::from("after1")],
            },
        };
        let m2 = MatchingResult {
            matching_line: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
            file_name: None,
            line_number: Some(42),
            context: Context {
                before: vec![String::from("before2")],
                after: vec![String::from("after2")],
            },
        };
        assert!(m1 < m2);
    }

    #[test]
    fn matching_result_comparisons_gt() {
        let m1 = MatchingResult {
            matching_line: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
            file_name: Some(String::from("test1.txt")),
            line_number: Some(41),
            context: Context {
                before: vec![String::from("before1")],
                after: vec![String::from("after1")],
            },
        };
        let m2 = MatchingResult {
            matching_line: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test2").unwrap(),
            file_name: Some(String::from("test2.txt")),
            line_number: Some(42),
            context: Context {
                before: vec![String::from("before2")],
                after: vec![String::from("after2")],
            },
        };
        assert!(m1 > m2);
    }

    #[test]
    fn matching_result_comparisons_le() {
        let m1 = MatchingResult {
            matching_line: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
            file_name: None,
            line_number: None,
            context: Context {
                before: vec![],
                after: vec![],
            },
        };
        let m2 = MatchingResult {
            matching_line: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
            file_name: None,
            line_number: None,
            context: Context {
                before: vec![],
                after: vec![],
            },
        };
        assert!(m1 <= m2);
    }

    #[test]
    fn matching_result_comparisons_ge() {
        let m1 = MatchingResult {
            matching_line: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
            file_name: None,
            line_number: None,
            context: Context {
                before: vec![],
                after: vec![],
            },
        };
        let m2 = MatchingResult {
            matching_line: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
            file_name: None,
            line_number: None,
            context: Context {
                before: vec![],
                after: vec![],
            },
        };
        assert!(m1 >= m2);
    }
}
