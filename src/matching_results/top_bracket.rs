use crate::MatchingResult;

pub(crate) struct TopBracket {
    capacity: usize,
    data: Vec<MatchingResult>,
}

impl TopBracket {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: Vec::with_capacity(capacity),
        }
    }

    pub(crate) fn push(&mut self, matching_result: MatchingResult) {
        if self.data.len() == self.capacity {
            if matching_result <= *self.data.last().unwrap() {
                return;
            }

            self.data.pop();
        }

        self.data.push(matching_result);
        self.data.sort_by(|a, b| b.cmp(a));
    }

    pub(crate) fn merge(&mut self, other: Self) {
        // TODO: this is a very suboptimal algorithm, it must be improved.
        for el in other.data {
            self.push(el);
        }
    }

    pub(crate) fn into_vec(self) -> Vec<MatchingResult> {
        self.data
    }

    pub(crate) fn is_full(&self) -> bool {
        self.data.len() == self.capacity
    }
}