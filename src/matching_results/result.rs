use vscode_fuzzy_score_rs::FuzzyMatch;

use crate::matching_results::context_accumulators::SaturatingAccumulator;

pub struct MatchingResult {
    pub matching_line: String,
    pub fuzzy_match: FuzzyMatch,
    pub file_name: Option<String>,
    pub line_number: Option<usize>,
    pub context: Context,
}

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

impl MatchingResultState {
    pub(crate) const fn new(
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

    pub(crate) const fn feed(self, line: String) -> Self {
        match self {
            Self::Complete(_) => panic!("Attempted to feed a line to a complete matching result!"),
            Self::Incomplete(partial_matching_result) => partial_matching_result.feed(line),
        }
    }
}

impl PartialMatchingResult {
    pub(crate) const fn feed(self, line: String) -> MatchingResultState {
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

    pub(crate) const fn complete(self) -> MatchingResult {
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
    const fn new(before: Vec<String>, after_size: usize) -> ContextState {
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
    const fn feed(mut self, line: String) -> ContextState {
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

    const fn complete(self) -> Context {
        Context {
            before: self.before,
            after: self.after_accumulator.consume(),
        }
    }
}

todo!("documentation!");

#[cfg(test)]
mod test {
    todo!("tests!");
}
