/// Represents the size of the context surrounding the matching line.
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContextSize {
    /// (Maximum) number of lines preceding the matching line.
    ///
    pub lines_before: usize,

    /// (Maximum) number of lines following the matching line.
    pub lines_after: usize,
}
