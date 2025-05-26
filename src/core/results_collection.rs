use crate::match_properties::MatchProperties;

/// A trait that generalizes interface between possible results containers
/// As it currently stands, only one method is required to be provided -
/// the one that adds an item into the container.
///
pub trait ResultsCollection {
    /// Adds an item to the collection.
    ///
    fn add(&mut self, result: MatchProperties);

    /// Consumes this collection and turns it into a sorted [`Vec<MatchProperties>`].
    ///
    fn into_sorted_vec(self) -> Vec<MatchProperties>;
}

impl ResultsCollection for Vec<MatchProperties> {
    fn add(&mut self, result: MatchProperties) {
        self.push(result);
    }

    fn into_sorted_vec(mut self) -> Vec<MatchProperties> {
        self.sort();
        self
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::min_ident_chars, reason = "It's tests, who cares?")]

    use super::*;
    use crate::match_properties::context::Context;
    use crate::match_properties::location::Location;

    #[test]
    fn add_to_vec() {
        let mut v = vec![MatchProperties {
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
        }];
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
            let mut v = v.clone();
            v.push(item.clone());
            v
        };

        v.add(item);
        assert_eq!(v, expected);
    }

    #[test]
    fn vec_into_sorted() {
        let v = vec![
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
        ];

        let mut expected = v.clone();
        expected.sort_by(|a, b| b.cmp(a));

        assert_eq!(v.into_sorted_vec(), expected);
    }
}
