use crate::{matching_results::top_bracket::TopBracket, MatchingResult};

/// A trait that generalizes interface between possible results containers
/// As it currently stands, only one method is required to be provided -
/// the one that adds an item into the container.
///
pub(crate) trait ResultCollection {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matching_results::result::Context;

    fn do_push<T: ResultCollection>(tested: &mut T, item: MatchingResult) -> &T {
        tested.push(item);
        tested
    }

    #[test]
    fn push_vec() {
        let mut v = vec![MatchingResult {
            matching_line: String::from("test_vec"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test_vec", "test_vec").unwrap(),
            file_name: None,
            line_number: None,
            context: Context {
                before: Vec::new(),
                after: Vec::new(),
            },
        }];
        let item = MatchingResult {
            matching_line: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
            file_name: None,
            line_number: None,
            context: Context {
                before: Vec::new(),
                after: Vec::new(),
            },
        };
        let expected = {
            let mut v = v.clone();
            v.push(item.clone());
            v
        };

        assert_eq!(*do_push(&mut v, item), expected);
    }

    #[test]
    fn push_top_bracket() {
        let mut tb = TopBracket::new(1);
        tb.push(MatchingResult {
            matching_line: String::from("test_top_bracket"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test_top_bracket", "test_top_bracket")
                .unwrap(),
            file_name: None,
            line_number: None,
            context: Context {
                before: Vec::new(),
                after: Vec::new(),
            },
        });
        let item = MatchingResult {
            matching_line: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
            file_name: None,
            line_number: None,
            context: Context {
                before: Vec::new(),
                after: Vec::new(),
            },
        };
        let expected = {
            let mut tb = tb.clone();
            tb.push(item.clone());
            tb
        };

        assert_eq!(*do_push(&mut tb, item.clone()), expected);
    }
}
