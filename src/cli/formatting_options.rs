/// Holds various formatting options.
///
/// Specifically:
///   * whether line numbers should be printed.
///
/// Use [`FormattingOptionsBuilder`] to configure [`FormattingOptions`] if needed (using the builder pattern).
///
/// # Examples
///
/// ```
/// let options = fzgrep::FormattingOptionsBuilder::new()
///     .line_number(true)
///     .build();
/// assert!(options.line_number());
/// ```
///
#[derive(Clone, Copy, Debug, Default)]
pub struct FormattingOptions {
    line_number: bool,
}

/// A simple getter that just returns the value of `--line-number` flag.
///
/// # Examples
///
/// ```
/// let options = fzgrep::FormattingOptions::default();
/// assert!(!options.line_number());
/// ```
///
/// ```
/// let options = fzgrep::FormattingOptionsBuilder::new()
///     .line_number(true)
///     .build();
/// assert!(options.line_number());
/// ```
///
impl FormattingOptions {
    pub fn line_number(&self) -> bool {
        self.line_number
    }
}

/// A builder that can be used to build [`Options`] objects.
///
/// # Examples
/// ```
/// let options = fzgrep::FormattingOptionsBuilder::new()
///     .line_number(true)
///     .build();
/// assert!(options.line_number());
/// ```
///
#[derive(Debug, Default, PartialEq)]
pub struct FormattingOptionsBuilder {
    line_number: bool,
}

impl FormattingOptionsBuilder {
    /// Creates a new builder with default options.
    ///
    /// # Examples
    /// ```
    /// let builder = fzgrep::FormattingOptionsBuilder::new();
    /// let options = builder.build();
    /// assert!(!options.line_number());
    /// ```
    ///
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `line_number` option to the provided value.
    ///
    /// # Examples
    /// ```
    /// let options = fzgrep::FormattingOptionsBuilder::new()
    ///     .line_number(true)
    ///     .build();
    /// assert!(options.line_number());
    /// ```
    ///
    pub fn line_number(mut self, line_number: bool) -> Self {
        self.line_number = line_number;
        self
    }

    /// Consumes the builder object in exchange for a configured [`FormattingOptions`] object.
    ///
    /// # Examples
    /// ```
    /// let builder = fzgrep::FormattingOptionsBuilder::new().line_number(true);
    /// let options = builder.build();
    /// assert!(options.line_number());
    /// ```
    ///
    pub fn build(self) -> FormattingOptions {
        FormattingOptions {
            line_number: self.line_number,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn builder_constructor() {
        let builder = FormattingOptionsBuilder::new();
        assert_eq!(builder, FormattingOptionsBuilder { line_number: false });
    }

    #[test]
    fn builder_line_number() {
        let builder = FormattingOptionsBuilder::new().line_number(false);
        assert_eq!(builder, FormattingOptionsBuilder { line_number: false });
        let builder = builder.line_number(true);
        assert_eq!(builder, FormattingOptionsBuilder { line_number: true });
    }

    #[test]
    fn builder_build() {
        let options = FormattingOptionsBuilder::new().line_number(true).build();
        assert!(options.line_number());
    }

    #[test]
    fn options_default() {
        let default = FormattingOptions::default();
        assert!(!default.line_number());
    }

    #[test]
    fn options_line_number() {
        let line_number_off = FormattingOptionsBuilder::new().line_number(false).build();
        assert!(!line_number_off.line_number());
        let line_number_on = FormattingOptionsBuilder::new().line_number(true).build();
        assert!(line_number_on.line_number());
    }
}
