use crate::cli::output::behavior::Behavior;
use crate::request::Request as CoreRequest;
use crate::request::collection_strategy::CollectionStrategy;
use crate::request::match_options::MatchOptions;
use crate::request::targets::Targets;

use log::LevelFilter;

/// Represents a CLI run request. The majority fields correspond to those of [`crate::request::Request`].
/// Added are CLI-specific fields that control the program output and logging.
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

    /// Match collection strategy.
    ///
    pub strategy: CollectionStrategy,

    /// Determines the behavior of the program with respect to the output.
    /// [`OutputBehavior::Normal`] means normal output
    /// whereas in case of [`OutputBehavior::Quiet`] the output is fully suppressed
    /// (program exit code can still be used to categorize the run results).
    ///
    pub output_behavior: Behavior,

    /// Controls the verbosity of the logs.
    ///
    pub log_verbosity: LevelFilter,
}

impl From<Request> for CoreRequest {
    fn from(value: Request) -> Self {
        Self {
            query: value.query,
            targets: value.targets,
            options: value.options,
            strategy: value.strategy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::request::match_options::context_size::ContextSize;
    use crate::request::match_options::{LineNumberTracking, SourceNameTracking};

    use std::path::PathBuf;

    #[test]
    fn convert_to_core_request() {
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
        let strategy = CollectionStrategy::CollectAll;

        let cli_request = Request {
            query: query.clone(),
            targets: targets.clone(),
            options,
            strategy,
            output_behavior: Behavior::Quiet,
            log_verbosity: LevelFilter::Debug,
        };

        let expected = CoreRequest {
            query,
            targets,
            options,
            strategy,
        };

        let core = CoreRequest::from(cli_request);

        assert_eq!(core, expected);
    }
}
