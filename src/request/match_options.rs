pub mod context_size;

use context_size::ContextSize;

/// A newtype wrapper that determines if the line numbers should be tracked and reported.
///
/// # Fields
///   * a boolean; if `true` the numbers of the matching lines will be tracked
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LineNumberTracking(pub bool);

/// A newtype wrapper that determines if the source names should be tracked and reported.
///
/// # Fields
///   * a boolean; if `true` the names of the matching sources will be tracked
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceNameTracking(pub bool);

/// Represents a set of options that control how the additional data about matches is collected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
