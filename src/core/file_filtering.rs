use glob::{MatchOptions, Pattern};
use path_slash::PathExt;
use std::path::Path;

/// Contains two sets of UNIX globs, one of include patterns and one of exclude ones.
/// A [`std::path::Path`] can be tested against those to see whether it should be allowed.
///
#[derive(Debug, PartialEq)]
pub struct Filter {
    include: Option<Vec<Pattern>>,
    exclude: Option<Vec<Pattern>>,
}

impl Filter {
    /// Constructs a [`Filter`] with include patterns from `include` and exclude patterns from `exclude`.
    ///
    #[must_use]
    pub const fn new(include: Vec<Pattern>, exclude: Vec<Pattern>) -> Self {
        Self {
            include: Some(include),
            exclude: Some(exclude),
        }
    }

    /// Constructs a [`Filter`] with include patterns from `include` and empty exclude patterns.
    ///
    #[must_use]
    pub const fn with_include(include: Vec<Pattern>) -> Self {
        Self {
            include: Some(include),
            exclude: None,
        }
    }

    /// Constructs a [`Filter`] with exclude patterns from `exclude` and empty include patterns.
    ///
    #[must_use]
    pub const fn with_exclude(exclude: Vec<Pattern>) -> Self {
        Self {
            include: None,
            exclude: Some(exclude),
        }
    }

    /// Test `path` against patterns of this [`Filter`].
    /// Returns `true` if `path` does not match any glob of the exclude ones
    /// and matches at least one glob of the include ones or the list of include globs is empty.
    /// Otherwise returns `false`.
    ///
    #[must_use]
    pub fn test(&self, path: &Path) -> bool {
        let normalized = path.to_slash_lossy();
        !self
            .exclude
            .iter()
            .flatten()
            .any(|p| p.matches(&normalized))
            && self.include.as_ref().is_none_or(|incl| {
                incl.iter().any(|p| {
                    p.matches_with(
                        &normalized,
                        MatchOptions {
                            require_literal_separator: true,
                            ..Default::default()
                        },
                    )
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_new() {
        let incl = vec![
            Pattern::new("*.txt").unwrap(),
            Pattern::new("*.json").unwrap(),
        ];
        let excl = vec![
            Pattern::new("build/*").unwrap(),
            Pattern::new("tests/*").unwrap(),
        ];
        let filter = Filter::new(incl.clone(), excl.clone());
        assert_eq!(
            filter,
            Filter {
                include: Some(incl),
                exclude: Some(excl),
            }
        );
    }

    #[test]
    fn constructor_with_include() {
        let incl = vec![
            Pattern::new("*.txt").unwrap(),
            Pattern::new("*.json").unwrap(),
        ];
        let filter = Filter::with_include(incl.clone());
        assert_eq!(
            filter,
            Filter {
                include: Some(incl),
                exclude: None,
            }
        );
    }

    #[test]
    fn constructor_with_exclude() {
        let excl = vec![
            Pattern::new("build/*").unwrap(),
            Pattern::new("tests/*").unwrap(),
        ];
        let filter = Filter::with_exclude(excl.clone());
        assert_eq!(
            filter,
            Filter {
                include: None,
                exclude: Some(excl),
            }
        );
    }

    #[test]
    fn test_single_include() {
        let filter = Filter {
            include: Some(vec![Pattern::new("*.txt").unwrap()]),
            exclude: None,
        };
        assert!(filter.test(Path::new("test.txt")));
        assert!(!filter.test(Path::new("whatever.json")));
    }

    #[test]
    fn test_multiple_include() {
        let filter = Filter {
            include: Some(vec![
                Pattern::new("*.txt").unwrap(),
                Pattern::new("*.rs").unwrap(),
            ]),
            exclude: None,
        };
        assert!(filter.test(Path::new("test.txt")));
        assert!(filter.test(Path::new("main.rs")));
        assert!(!filter.test(Path::new("src/main.rs")));
        assert!(!filter.test(Path::new("whatever.json")));
    }

    #[test]
    fn test_single_exclude() {
        let filter = Filter {
            include: None,
            exclude: Some(vec![Pattern::new("*.txt").unwrap()]),
        };
        assert!(!filter.test(Path::new("test.txt")));
        assert!(filter.test(Path::new("whatever.json")));
    }

    #[test]
    fn test_multiple_exclude() {
        let filter = Filter {
            include: None,
            exclude: Some(vec![
                Pattern::new("*.txt").unwrap(),
                Pattern::new("build/*").unwrap(),
            ]),
        };
        assert!(!filter.test(Path::new("test.txt")));
        assert!(!filter.test(Path::new("build/whatever.json")));
        assert!(filter.test(Path::new("whatever.json")));
    }

    #[test]
    fn test_mixed_include_exclude() {
        let filter = Filter {
            include: Some(vec![
                Pattern::new("*.txt").unwrap(),
                Pattern::new("*.rs").unwrap(),
            ]),
            exclude: Some(vec![
                Pattern::new("tests/*").unwrap(),
                Pattern::new("build/*").unwrap(),
            ]),
        };
        assert!(filter.test(Path::new("test.txt")));
        assert!(filter.test(Path::new("main.rs")));
        assert!(!filter.test(Path::new("src/main.rs")));
        assert!(!filter.test(Path::new("tests/test.txt")));
        assert!(!filter.test(Path::new("build/main.rs")));
        assert!(!filter.test(Path::new("/home/user/whatever/whatever.json")));
    }

    #[test]
    fn test_unix_paths() {
        let filter = Filter {
            include: Some(vec![Pattern::new("**/*.txt").unwrap()]),
            exclude: Some(vec![Pattern::new("**/*test*").unwrap()]),
        };

        assert!(filter.test(Path::new("whatever.txt")));
        assert!(filter.test(Path::new("whatever/whatever.txt")));
        assert!(filter.test(Path::new("/home/user/whatever/whatever.txt")));
        assert!(!filter.test(Path::new("whatever.json")));
        assert!(!filter.test(Path::new("whatever/whatever.json")));
        assert!(!filter.test(Path::new("/home/user/whatever/whatever.json")));
        assert!(!filter.test(Path::new("test.txt")));
        assert!(!filter.test(Path::new("whatever/test.txt")));
        assert!(!filter.test(Path::new("/home/user/whatever/test.txt")));
    }

    #[test]
    fn test_windows_paths() {
        let filter = Filter {
            include: Some(vec![Pattern::new("**/*.txt").unwrap()]),
            exclude: Some(vec![Pattern::new("**/*test*").unwrap()]),
        };

        assert!(filter.test(Path::new("whatever.txt")));
        assert!(filter.test(Path::new(r"whatever\whatever.txt")));
        assert!(filter.test(Path::new(r"C:\Users\user\whatever\whatever.txt")));
        assert!(!filter.test(Path::new("whatever.json")));
        assert!(!filter.test(Path::new(r"whatever\whatever.json")));
        assert!(!filter.test(Path::new(r"C:\Users\user\whatever\whatever.json")));
        assert!(!filter.test(Path::new("test.txt")));
        assert!(!filter.test(Path::new(r"whatever\test.txt")));
        assert!(!filter.test(Path::new(r"C:\Users\user\whatever\test.txt")));
    }
}
