/// Matches collection behavior.
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollectionStrategy {
    /// All matches must be kept.
    ///
    CollectAll,

    /// Only a number best matches should be kept.
    /// Note that the intended use for this is to reduce the strain on the memory
    /// when the expected number of matches is very high.
    /// Running with this strategy causes more sort operations to happen
    /// so it might even turn out to be slower than collecting all matches
    /// if the total number of matches is relatively low.
    ///
    /// # Fields
    ///   * the number of top matches
    ///
    CollectTop(usize),
}
