pub mod collection_strategy;
pub mod match_options;
pub mod targets;

use collection_strategy::CollectionStrategy;
use match_options::MatchOptions;
use targets::Targets;

use log::LevelFilter;

/// Represents a run configuration.
///
#[derive(Debug, Eq, PartialEq)]
pub struct Request {
    /// The query to match against.
    ///
    pub query: String,

    /// The input targets - files, directories or the standard input.
    ///
    pub targets: Targets,

    /// Matches collection strategy,
    ///
    pub collection_strategy: CollectionStrategy,

    /// Additional data about the matches to be collected.
    ///
    pub match_options: MatchOptions,

    /// Control the verbosity of the logs.
    ///
    // TODO: move this to cli::Request
    pub log_verbosity: LevelFilter,
}
