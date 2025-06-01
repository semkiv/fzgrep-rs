use crate::match_properties::MatchProperties as CoreMatchProperties;
use crate::match_properties::location::Location;
use crate::request::Request as CoreRequest;
use crate::request::match_options::MatchOptions;
use crate::request::targets::Targets;

use vscode_fuzzy_score_rs::FuzzyMatch;

/// Minimal matching request containing only essentials required for matching.
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    /// The query to match against.
    ///
    pub query: String,

    /// The input targets - files, directories or the standard input.
    ///
    pub targets: Targets,

    /// Additional data about the matches to be collected.
    ///
    pub options: MatchOptions,
}

/// Minimal match properties containing only essentials (i.e. without context).
///
#[derive(Clone, Debug)]
pub struct MatchProperties {
    /// The line that contains the match.
    ///
    pub matching_line: String,

    /// The properties of the match.
    ///
    pub fuzzy_match: FuzzyMatch,

    /// The location of the match.
    ///
    pub location: Location,
}

impl From<CoreRequest> for Request {
    fn from(value: CoreRequest) -> Self {
        Self {
            query: value.query,
            targets: value.targets,
            options: value.options,
        }
    }
}

impl From<CoreMatchProperties> for MatchProperties {
    fn from(value: CoreMatchProperties) -> Self {
        Self {
            matching_line: value.matching_line,
            fuzzy_match: value.fuzzy_match,
            location: value.location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::match_properties::context::Context;
    use crate::request::collection_strategy::CollectionStrategy;
    use crate::request::match_options::context_size::ContextSize;
    use crate::request::match_options::{LineNumberTracking, SourceNameTracking};

    use std::path::PathBuf;

    #[test]
    fn request_convert_core_into_basic() {
        let query = String::from("test");
        let targets = Targets::Files(vec![PathBuf::from("test.txt")]);
        let options = MatchOptions {
            line_number_tracking: LineNumberTracking::Off,
            source_name_tracking: SourceNameTracking::Off,
            context_size: ContextSize {
                lines_before: 1,
                lines_after: 2,
            },
        };

        let core = CoreRequest {
            query: query.clone(),
            targets: targets.clone(),
            options,
            strategy: CollectionStrategy::CollectAll,
        };

        let expected = Request {
            query,
            targets,
            options,
        };

        let basic = Request::from(core);

        assert_eq!(basic, expected);
    }

    #[test]
    fn match_properties_convert_core_into_basic() {
        let matching_line = String::from("test");
        let fuzzy_match = vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap();
        let location = Location {
            source_name: None,
            line_number: None,
        };

        let core = CoreMatchProperties {
            matching_line: matching_line.clone(),
            fuzzy_match: fuzzy_match.clone(),
            location: location.clone(),
            context: Context {
                before: None,
                after: None,
            },
        };

        let expected = MatchProperties {
            matching_line,
            fuzzy_match,
            location,
        };

        let basic = MatchProperties::from(core);

        assert_eq!(basic.matching_line, expected.matching_line);
        assert_eq!(basic.fuzzy_match, expected.fuzzy_match);
        assert_eq!(basic.location, expected.location);
    }
}
