/// A location data of the line (if available).
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Location {
    /// An optional source (file or stdin) name (if file names tracking was requested).
    ///
    pub source_name: Option<String>,

    /// An optional line number (if line numbers tracking was requested).
    ///
    pub line_number: Option<usize>,
}
