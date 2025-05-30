use crate::core::results_collection::ResultsCollection;
use crate::match_properties::MatchProperties;

/// A container that store up to a max count (capacity) of items sorted in descending order.
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopBracket<T> {
    capacity: usize,
    data: Vec<T>,
}

impl<T> TopBracket<T> {
    /// Creates a empty `TopBracket` with a capacity of `capacity`.
    ///
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: Vec::with_capacity(capacity),
        }
    }

    /// Converts this `TopBracket` into a [`Vec`].
    ///
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }
}

impl<T: Ord> TopBracket<T> {
    /// Tries to add an `item` to the container. If the container is not at capacity yet, the item is added right away.
    /// If the container is at capacity already one of the two happens:
    /// if the item is larger than the the smallest item present in the container,
    /// the smallest item is removed from the container and the new item is added,
    /// otherwise the new item is simply discarded.
    ///
    /// After each addition the container is sorted to ensure order.
    ///
    /// Returns `true` if the item was actually inserted, `false` otherwise.
    ///
    pub fn push(&mut self, item: T) -> bool {
        if self.capacity == 0 {
            return false;
        }

        if self.data.len() == self.capacity {
            #[expect(
                clippy::unwrap_used,
                reason = "The capacity is non-zero, so the deque is non-empty and there must at least one element"
            )]
            if item <= *self.data.last().unwrap() {
                return false;
            }

            self.data.pop();
        }

        self.data.push(item);
        self.data.sort_by(|a, b| b.cmp(a));
        true
    }
}

impl ResultsCollection for TopBracket<MatchProperties> {
    fn add(&mut self, result: MatchProperties) {
        self.push(result);
    }

    fn into_sorted_vec(self) -> Vec<MatchProperties> {
        self.into_vec()
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::default_numeric_fallback, reason = "It's tests, who cares?")]
    #![expect(clippy::shadow_unrelated, reason = "It's tests, who cares?")]

    use super::*;
    use crate::match_properties::context::Context;
    use crate::match_properties::location::Location;

    #[test]
    fn constructor() {
        let capacity = 4;
        let container = TopBracket::<i32>::new(capacity);
        assert_eq!(container.capacity, capacity);
        assert_eq!(container.data.len(), 0);
        assert_eq!(container.data.capacity(), capacity);
    }

    #[test]
    fn push() {
        let capacity = 4;
        let mut container = TopBracket::new(capacity);
        assert_eq!(container.capacity, capacity);
        assert_eq!(container.data.len(), 0);
        assert_eq!(container.data.capacity(), capacity);

        assert!(container.push(1));
        assert_eq!(container.data.len(), 1);
        assert_eq!(container.data, [1]);

        assert!(container.push(2));
        assert_eq!(container.data.len(), 2);
        assert_eq!(container.data, [2, 1]);

        assert!(container.push(0));
        assert_eq!(container.data.len(), 3);
        assert_eq!(container.data, [2, 1, 0]);

        assert!(container.push(1));
        assert_eq!(container.data.len(), 4);
        assert_eq!(container.data, [2, 1, 1, 0]);

        assert!(!container.push(-1));
        assert_eq!(container.data.len(), 4);
        assert_eq!(container.data, [2, 1, 1, 0]);

        assert!(!container.push(0));
        assert_eq!(container.data.len(), 4);
        assert_eq!(container.data, [2, 1, 1, 0]);

        assert!(container.push(2));
        assert_eq!(container.data.len(), 4);
        assert_eq!(container.data, [2, 2, 1, 1]);
    }

    #[test]
    fn into_vec() {
        let capacity = 4;
        let container = TopBracket::<i32>::new(capacity);
        assert_eq!(container.into_vec(), []);

        let mut container = TopBracket::new(capacity);
        container.push(1);
        container.push(2);
        assert_eq!(container.into_vec(), [2, 1]);

        let mut container = TopBracket::new(capacity);
        container.push(1);
        container.push(2);
        container.push(0);
        container.push(1);
        container.push(2);
        assert_eq!(container.into_vec(), [2, 2, 1, 1]);
    }

    #[test]
    fn add_to_top_bracket() {
        let mut top_bracket = TopBracket::new(1);
        top_bracket.push(MatchProperties {
            matching_line: String::from("test_top_bracket"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test_top_bracket", "test_top_bracket")
                .unwrap(),
            location: Location {
                source_name: None,
                line_number: None,
            },
            context: Context {
                before: None,
                after: None,
            },
        });
        let item = MatchProperties {
            matching_line: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
            location: Location {
                source_name: None,
                line_number: None,
            },
            context: Context {
                before: None,
                after: None,
            },
        };
        let expected = {
            let mut top_bracket = top_bracket.clone();
            top_bracket.push(item.clone());
            top_bracket
        };

        top_bracket.add(item);
        assert_eq!(top_bracket, expected);
    }

    #[test]
    fn top_bracket_into_sorted_vec() {
        let mut results = vec![
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
                location: Location {
                    source_name: None,
                    line_number: None,
                },
                context: Context {
                    before: None,
                    after: None,
                },
            },
            MatchProperties {
                matching_line: String::from("test_vec"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test_vec", "test_vec").unwrap(),
                location: Location {
                    source_name: None,
                    line_number: None,
                },
                context: Context {
                    before: None,
                    after: None,
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
                location: Location {
                    source_name: None,
                    line_number: None,
                },
                context: Context {
                    before: None,
                    after: None,
                },
            },
            MatchProperties {
                matching_line: String::from("test_vec"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test_vec", "test_vec").unwrap(),
                location: Location {
                    source_name: None,
                    line_number: None,
                },
                context: Context {
                    before: None,
                    after: None,
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
                location: Location {
                    source_name: None,
                    line_number: None,
                },
                context: Context {
                    before: None,
                    after: None,
                },
            },
            MatchProperties {
                matching_line: String::from("test"),
                fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
                location: Location {
                    source_name: None,
                    line_number: None,
                },
                context: Context {
                    before: None,
                    after: None,
                },
            },
        ];

        let mut top_bracket = TopBracket::new(4);
        for res in &results {
            top_bracket.push(res.clone());
        }

        results.sort_by(|a, b| b.cmp(a));

        assert_eq!(top_bracket.into_sorted_vec(), results);
    }
}
