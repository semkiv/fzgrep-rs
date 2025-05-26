/// Context (surrounding lines) around a match (if available).
///
#[derive(Clone, Debug)]
pub struct Context {
    /// Lines preceding the matching line (if any).
    ///
    pub before: Option<Vec<String>>,

    /// Lines following the matching line (if any).
    ///
    pub after: Option<Vec<String>>,
}
