use crate::matching_results::context_accumulators::SaturatingAccumulator;
use std::cmp::Ordering;
use vscode_fuzzy_score_rs::FuzzyMatch;

#[derive(Debug)]
pub struct MatchingResult {
    pub matching_line: String,
    pub fuzzy_match: FuzzyMatch,
    pub file_name: Option<String>,
    pub line_number: Option<usize>,
    pub context: Context,
}

#[derive(Debug)]
pub struct Context {
    pub before: Vec<String>,
    pub after: Vec<String>,
}

pub(crate) enum MatchingResultState {
    Complete(MatchingResult),
    Incomplete(PartialMatchingResult),
}

pub(crate) struct PartialMatchingResult {
    matching_line: String,
    fuzzy_match: FuzzyMatch,
    file_name: Option<String>,
    line_number: Option<usize>,
    partial_context: PartialContext,
}

enum ContextState {
    Complete(Context),
    Incomplete(PartialContext),
}

struct PartialContext {
    before: Vec<String>,
    after_accumulator: SaturatingAccumulator,
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

impl MatchingResultState {
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

impl ContextState {
    fn new(before: Vec<String>, after_size: usize) -> ContextState {
        let accumulator = SaturatingAccumulator::new(after_size);
        if accumulator.is_full() {
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
        if self.after_accumulator.is_full() {
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

todo!("documentation!");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matching_line_comparisons_ne() {
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
    fn matching_line_comparisons_eq() {
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
    fn matching_line_comparisons_lt() {
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
    fn matching_line_comparisons_gt() {
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
    fn matching_line_comparisons_le() {
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
    fn matching_line_comparisons_ge() {
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

    todo!("more tests!");
}
