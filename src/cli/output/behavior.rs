use crate::output::formatting::Formatting;

/// Behavior of the program with respect to the output
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Behavior {
    /// Output normally.
    ///
    /// # Fields
    ///   * the formatting to use to decorate the output
    ///
    Normal(Formatting),

    /// Output is suppressed, return code can be used to categorize the run results.
    ///
    Quiet,
}
