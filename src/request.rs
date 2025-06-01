pub mod collection_strategy;
pub mod match_options;
pub mod targets;

use collection_strategy::CollectionStrategy;
use match_options::MatchOptions;
use targets::Targets;

/// Represents a run configuration.
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
}
