use std::cmp::Ordering;
use vscode_fuzzy_score_rs::FuzzyMatch;

#[derive(Debug, PartialEq)]
pub struct Location {
    pub file_name: String,
    pub line_number: usize,
}

#[derive(Debug)]
pub struct MatchingLine {
    pub location: Location,
    pub content: String,
    pub fuzzy_match: FuzzyMatch,
}

impl PartialEq for MatchingLine {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_match.eq(&other.fuzzy_match)
    }
}

impl PartialOrd for MatchingLine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.fuzzy_match.partial_cmp(&other.fuzzy_match)
    }
}

impl Eq for MatchingLine {}

impl Ord for MatchingLine {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fuzzy_match.cmp(&other.fuzzy_match)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_line_comparisons_ne() {
        let m1 = MatchingLine {
            location: Location {
                file_name: String::from("test.txt"),
                line_number: 42,
            },
            content: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
        };
        let m2 = MatchingLine {
            location: Location {
                file_name: String::from("test.txt"),
                line_number: 42,
            },
            content: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tes", "test").unwrap(),
        };
        assert_ne!(m1, m2);
    }

    #[test]
    fn matching_line_comparisons_eq() {
        let m1 = MatchingLine {
            location: Location {
                file_name: String::from("test1.txt"),
                line_number: 42,
            },
            content: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
        };
        let m2 = MatchingLine {
            location: Location {
                file_name: String::from("test2.txt"),
                line_number: 42,
            },
            content: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
        };
        assert_eq!(m1, m2);
    }

    #[test]
    fn matching_line_comparisons_lt() {
        let m1 = MatchingLine {
            location: Location {
                file_name: String::from("test1.txt"),
                line_number: 42,
            },
            content: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test1").unwrap(),
        };
        let m2 = MatchingLine {
            location: Location {
                file_name: String::from("test2.txt"),
                line_number: 41,
            },
            content: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
        };
        assert!(m1 < m2);
    }

    #[test]
    fn matching_line_comparisons_gt() {
        let m1 = MatchingLine {
            location: Location {
                file_name: String::from("test1.txt"),
                line_number: 42,
            },
            content: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
        };
        let m2 = MatchingLine {
            location: Location {
                file_name: String::from("test2.txt"),
                line_number: 41,
            },
            content: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test2").unwrap(),
        };
        assert!(m1 > m2);
    }

    #[test]
    fn matching_line_comparisons_le() {
        let m1 = MatchingLine {
            location: Location {
                file_name: String::from("test1.txt"),
                line_number: 42,
            },
            content: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test1").unwrap(),
        };
        let m2 = MatchingLine {
            location: Location {
                file_name: String::from("test2.txt"),
                line_number: 42,
            },
            content: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test2", "test2").unwrap(),
        };
        assert!(m1 <= m2);
    }

    #[test]
    fn matching_line_comparisons_ge() {
        let m1 = MatchingLine {
            location: Location {
                file_name: String::from("test1.txt"),
                line_number: 41,
            },
            content: String::from("test1"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test1", "test1").unwrap(),
        };
        let m2 = MatchingLine {
            location: Location {
                file_name: String::from("test2.txt"),
                line_number: 41,
            },
            content: String::from("test2"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test2").unwrap(),
        };
        assert!(m1 >= m2);
    }
}
