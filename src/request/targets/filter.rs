use glob::{MatchOptions, Pattern};
use path_slash::PathExt as _;
use std::path::Path;

const MATCH_OPTIONS: MatchOptions = MatchOptions {
    require_literal_separator: true,
    // ..Default::default()
    case_sensitive: true,
    require_literal_leading_dot: false,
};

/// Contains two sets of UNIX globs, one of include patterns and one of exclude ones.
/// A [`std::path::Path`] can be tested against those to see whether it should be allowed.
///
#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub fn is_allowed(&self, path: &Path) -> bool {
        let normalized = path.to_slash_lossy();
        !self.is_disallowed_by_exclude_str(&normalized)
            && self.is_allowed_by_include_str(&normalized)
    }

    /// Test `path` against the list of include globs.
    /// Returns `true` if `path` matches at least one pattern in the list of include globs
    /// or the list of include globs is empty. Otherise returns `false`.
    ///
    #[must_use]
    pub fn is_allowed_by_include(&self, path: &Path) -> bool {
        let normalized = path.to_slash_lossy();
        self.is_allowed_by_include_str(&normalized)
    }

    /// Test `path` against the list of exclude globs.
    /// Returns `true` if the list of exclude globs is non-empty and `path` matches at least
    /// one pattern. Otherwise returns `false`.
    ///
    #[must_use]
    pub fn is_disallowed_by_exclude(&self, path: &Path) -> bool {
        let normalized = path.to_slash_lossy();
        self.is_disallowed_by_exclude_str(&normalized)
    }

    fn is_allowed_by_include_str(&self, path_str: &str) -> bool {
        self.include.as_ref().is_none_or(|incl| {
            incl.iter()
                .any(|pattern| pattern.matches_with(path_str, MATCH_OPTIONS))
        })
    }

    fn is_disallowed_by_exclude_str(&self, path_str: &str) -> bool {
        self.exclude.as_ref().is_some_and(|excl| {
            excl.iter()
                .any(|pattern| pattern.matches_with(path_str, MATCH_OPTIONS))
        })
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::cognitive_complexity, reason = "It's tests, who cares?")]
    #![expect(clippy::shadow_unrelated, reason = "It's tests, who cares?")]

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
    fn is_allowed_single_include() {
        let filter = Filter {
            include: Some(vec![Pattern::new("*.txt").unwrap()]),
            exclude: None,
        };

        let path = Path::new("test.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));

        let path = Path::new("whatever.json");
        assert!(!filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));
    }

    #[test]
    fn is_allowed_multiple_include() {
        let filter = Filter {
            include: Some(vec![
                Pattern::new("*.txt").unwrap(),
                Pattern::new("*.rs").unwrap(),
            ]),
            exclude: None,
        };

        let path = Path::new("test.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));

        let path = Path::new("main.rs");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));

        let path = Path::new("src/main.rs");
        assert!(!filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("whatever.json");
        assert!(!filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));
    }

    #[test]
    fn is_allowed_single_exclude() {
        let filter = Filter {
            include: None,
            exclude: Some(vec![Pattern::new("*.txt").unwrap()]),
        };

        let path = Path::new("test.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("whatever.json");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));
    }

    #[test]
    fn is_allowed_multiple_exclude() {
        let filter = Filter {
            include: None,
            exclude: Some(vec![
                Pattern::new("*.txt").unwrap(),
                Pattern::new("build/*").unwrap(),
            ]),
        };

        let path = Path::new("test.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("build/whatever.json");
        assert!(filter.is_allowed_by_include(path));
        assert!(filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("whatever.json");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));
    }

    #[test]
    fn is_allowed_mixed_include_exclude() {
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

        let path = Path::new("test.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));

        let path = Path::new("main.rs");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));

        let path = Path::new("src/main.rs");
        assert!(!filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("tests/test.txt");
        assert!(!filter.is_allowed_by_include(path));
        assert!(filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("build/main.rs");
        assert!(!filter.is_allowed_by_include(path));
        assert!(filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("/home/user/whatever/whatever.json");
        assert!(!filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));
    }

    #[test]
    fn is_allowed_unix_paths() {
        let filter = Filter {
            include: Some(vec![Pattern::new("**/*.txt").unwrap()]),
            exclude: Some(vec![Pattern::new("**/*test*").unwrap()]),
        };

        let path = Path::new("whatever.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));

        let path = Path::new("whatever/whatever.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));

        let path = Path::new("/home/user/whatever/whatever.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));

        let path = Path::new("whatever.json");
        assert!(!filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("whatever/whatever.json");
        assert!(!filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("/home/user/whatever/whatever.json");
        assert!(!filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("test.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("whatever/test.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("/home/user/whatever/test.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));
    }

    #[test]
    fn is_allowed_windows_paths() {
        let filter = Filter {
            include: Some(vec![Pattern::new("**/*.txt").unwrap()]),
            exclude: Some(vec![Pattern::new("**/*test*").unwrap()]),
        };

        let path = Path::new("whatever.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));

        let path = Path::new(r"whatever\whatever.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));

        let path = Path::new(r"C:\Users\user\whatever\whatever.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(filter.is_allowed(path));

        let path = Path::new("whatever.json");
        assert!(!filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new(r"whatever\whatever.json");
        assert!(!filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new(r"C:\Users\user\whatever\whatever.json");
        assert!(!filter.is_allowed_by_include(path));
        assert!(!filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new("test.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new(r"whatever\test.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));

        let path = Path::new(r"C:\Users\user\whatever\test.txt");
        assert!(filter.is_allowed_by_include(path));
        assert!(filter.is_disallowed_by_exclude(path));
        assert!(!filter.is_allowed(path));
    }
}
