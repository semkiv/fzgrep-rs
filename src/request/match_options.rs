pub mod context_size;

use context_size::ContextSize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineNumberTracking {
    /// Enable tracking of line numbers.
    ///
    On,

    /// Disable tracking of line numbers.
    ///
    Off,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceNameTracking {
    /// Enable tracking of source (usually file) names.
    ///
    On,

    /// Disable tracking of source (usually file) names.
    ///
    Off,
}

/// Represents a set of options that control how the additional data about matches is collected.
#[derive(Debug, Eq, PartialEq)]
pub struct MatchOptions {
    /// Determines whether the numbers of matching lines are of interest and should be tracked during processing.
    ///
    pub line_number_tracking: LineNumberTracking,

    /// Determines whether the names of the sources (files or stdin) containing matching lines are of interest
    /// and should be tracked during processing.
    ///
    pub source_name_tracking: SourceNameTracking,

    /// Controls the size (numbers of lines before and after) of the context surrounding the matching line.
    ///
    pub context_size: ContextSize,
}
