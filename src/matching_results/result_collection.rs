use crate::{MatchingResult, matching_results::top_bracket::TopBracket};


pub trait ResultCollection {
    fn push(&mut self, result: MatchingResult);
    fn merge(&mut self, other: Self);
    fn is_full(&self) -> bool;
}

impl ResultCollection for Vec<MatchingResult> {
    fn push(&mut self, result: MatchingResult) {
        self.push(result);
    }

    fn merge(&mut self, mut other: Self) {
        self.append(&mut other);
    }

    fn is_full(&self) -> bool {
        false
    }
}

impl ResultCollection for TopBracket {
    fn push(&mut self, result: MatchingResult) {
        self.push(result);
    }

    fn merge(&mut self, other: Self) {
        self.merge(other);
    }

    fn is_full(&self) -> bool {
        self.is_full()
    }
}
