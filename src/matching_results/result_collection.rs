use crate::{MatchingResult, matching_results::top_bracket::TopBracket};


pub trait ResultCollection {
    fn push(&mut self, result: MatchingResult);
}

impl ResultCollection for Vec<MatchingResult> {
    fn push(&mut self, result: MatchingResult) {
        self.push(result);
    }
}

impl ResultCollection for TopBracket<MatchingResult> {
    fn push(&mut self, result: MatchingResult) {
        self.push(result);
    }
}
