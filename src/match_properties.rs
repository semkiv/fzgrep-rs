pub mod context;
pub mod location;

use context::Context;
use location::Location;
use std::cmp::Ordering;
use vscode_fuzzy_score_rs::FuzzyMatch;

/// Stores a full result of matching.
///
#[derive(Clone, Debug)]
pub struct MatchProperties {
    /// The line that contains the match.
    ///
    pub matching_line: String,

    /// The properties of the match.
    ///
    pub fuzzy_match: FuzzyMatch,

    /// Location of the match.
    ///
    pub location: Location,

    /// Context surrounding the match.
    ///
    pub context: Context,
}

impl PartialEq for MatchProperties {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_match.eq(&other.fuzzy_match)
    }
}

impl PartialOrd for MatchProperties {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for MatchProperties {}

impl Ord for MatchProperties {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fuzzy_match.cmp(&other.fuzzy_match)
    }
}

#[cfg(test)]
mod test {
    #![expect(clippy::min_ident_chars, reason = "It's tests, who cares?")]

    use super::*;

    #[test]
    fn comparisons_ne() {
        let m1 = MatchProperties {
            matching_line: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
            location: Location {
                source_name: None,
                line_number: Some(42),
            },
            context: Context {
                before: Some(vec![String::from("before")]),
                after: Some(vec![String::from("after")]),
            },
        };
        let m2 = MatchProperties {
            matching_line: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tes", "test").unwrap(),
            location: Location {
                source_name: None,
                line_number: Some(42),
            },
            context: Context {
                before: Some(vec![String::from("before")]),
                after: Some(vec![String::from("after")]),
            },
        };
        assert_ne!(m1, m2);
    }

    #[test]
    fn comparisons_eq() {
        let m1 = MatchProperties {
            matching_line: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
            location: Location {
                source_name: Some(String::from("test.txt")),
                line_number: None,
            },
            context: Context {
                before: Some(vec![String::from("before1")]),
                after: Some(vec![String::from("after1")]),
            },
        };
        let m2 = MatchProperties {
            matching_line: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
            location: Location {
                source_name: None,
                line_number: Some(42),
            },
            context: Context {
                before: Some(vec![String::from("before2")]),
                after: Some(vec![String::from("after2")]),
            },
        };
        assert_eq!(m1, m2);
    }

    #[test]
    fn comparisons_lt() {
        let m1 = MatchProperties {
            matching_line: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test1").unwrap(),
            location: Location {
                source_name: Some(String::from("test.txt")),
                line_number: None,
            },
            context: Context {
                before: Some(vec![String::from("before1")]),
                after: Some(vec![String::from("after1")]),
            },
        };
        let m2 = MatchProperties {
            matching_line: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
            location: Location {
                source_name: None,
                line_number: Some(42),
            },
            context: Context {
                before: Some(vec![String::from("before2")]),
                after: Some(vec![String::from("after2")]),
            },
        };
        assert!(m1 < m2);
    }

    #[test]
    fn comparisons_gt() {
        let m1 = MatchProperties {
            matching_line: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
            location: Location {
                source_name: Some(String::from("test1.txt")),
                line_number: Some(41),
            },
            context: Context {
                before: Some(vec![String::from("before1")]),
                after: Some(vec![String::from("after1")]),
            },
        };
        let m2 = MatchProperties {
            matching_line: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test2").unwrap(),
            location: Location {
                source_name: Some(String::from("test2.txt")),
                line_number: Some(42),
            },
            context: Context {
                before: Some(vec![String::from("before2")]),
                after: Some(vec![String::from("after2")]),
            },
        };
        assert!(m1 > m2);
    }

    #[test]
    fn comparisons_le() {
        let m1 = MatchProperties {
            matching_line: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
            location: Location {
                source_name: None,
                line_number: None,
            },
            context: Context {
                before: None,
                after: None,
            },
        };
        let m2 = MatchProperties {
            matching_line: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
            location: Location {
                source_name: None,
                line_number: None,
            },
            context: Context {
                before: None,
                after: None,
            },
        };
        assert!(m1 <= m2);
    }

    #[test]
    fn comparisons_ge() {
        let m1 = MatchProperties {
            matching_line: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
            location: Location {
                source_name: None,
                line_number: None,
            },
            context: Context {
                before: None,
                after: None,
            },
        };
        let m2 = MatchProperties {
            matching_line: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
            location: Location {
                source_name: None,
                line_number: None,
            },
            context: Context {
                before: None,
                after: None,
            },
        };
        assert!(m1 >= m2);
    }
}
