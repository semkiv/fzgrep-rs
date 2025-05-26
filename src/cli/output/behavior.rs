use crate::output::formatting::Formatting;

/// Behavior of the program with respect to the output
///
#[derive(Debug, Eq, PartialEq)]
pub enum Behavior {
    /// Output normally.
    ///
    Normal(Formatting),

    /// Output is suppressed, return code can be used to categorize the run results.
    ///
    Quiet,
}

impl Behavior {
    /// Converts [`OutputBehavior`] to [`Option<Formatting>`].
    /// If `self` is [`OutputBehavior::Normal`] returns [`Some`] with the inner formatting, otherwise [`None`].
    /// Note that this method is not intended to be used in the real code, it is there to simplify the validation of tests.
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::{Formatting, FormattingOptions, OutputBehavior};
    ///
    /// let formatting = Formatting::On(FormattingOptions::default());
    /// assert_eq!(
    ///     formatting.formatting().unwrap(),
    ///     Formatting::On(FormattingOptions::default())
    /// );
    /// ```
    ///
    /// ```
    /// let behavior = OutputBehavior::Quiet;
    /// assert_eq!(behavior.formatting(), None);
    /// ```
    ///
    #[cfg(test)]
    pub(crate) const fn formatting(&self) -> Option<Formatting> {
        match self {
            Self::Normal(formatting) => Some(*formatting),
            Self::Quiet => None,
        }
    }
}

#[cfg(test)]
mod test {
    #![expect(clippy::shadow_unrelated, reason = "It's tests, who cares?")]

    use super::*;
    use crate::cli::output::formatting::StyleSet;

    #[test]
    fn output_behavior_to_formatting() {
        let behavior = Behavior::Normal(Formatting::On(StyleSet::default()));
        assert_eq!(
            behavior.formatting(),
            Some(Formatting::On(StyleSet::default()))
        );
        let behavior = Behavior::Quiet;
        assert_eq!(behavior.formatting(), None);
    }
}
