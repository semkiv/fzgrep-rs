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
    file_name: bool,
}

impl FormattingOptions {
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
    pub fn line_number(&self) -> bool {
        self.line_number
    }

    /// A simple getter for `--with-filename`/`--no-filename` flag.
    ///
    /// # Examples
    ///
    /// ```
    /// let options = fzgrep::FormattingOptions::default();
    /// assert!(!options.file_name());
    /// ```
    ///
    /// ```
    /// let options = fzgrep::FormattingOptionsBuilder::new()
    ///     .file_name(true)
    ///     .build();
    /// assert!(options.file_name());
    /// ```
    ///
    pub fn file_name(&self) -> bool {
        self.file_name
    }
}

/// A builder that can be used to build [`Options`] objects.
///
/// # Examples
/// ```
/// let options = fzgrep::FormattingOptionsBuilder::new()
///     .line_number(true)
///     .file_name(true)
///     .build();
/// assert!(options.line_number());
/// assert!(options.file_name())
/// ```
///
#[derive(Debug, Default, PartialEq)]
pub struct FormattingOptionsBuilder {
    line_number: bool,
    file_name: bool,
}

impl FormattingOptionsBuilder {
    /// Creates a new builder with default options.
    ///
    /// # Examples
    /// ```
    /// let builder = fzgrep::FormattingOptionsBuilder::new();
    /// let options = builder.build();
    /// assert!(!options.line_number());
    /// assert!(!options.file_name());
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

    /// Set `file_name` option to the provided value.
    ///
    /// # Examples
    /// ```
    /// let options = fzgrep::FormattingOptionsBuilder::new()
    ///     .file_name(true)
    ///     .build();
    /// assert!(options.file_name());
    /// ```
    ///
    pub fn file_name(mut self, file_name: bool) -> Self {
        self.file_name = file_name;
        self
    }

    /// Consumes the builder object in exchange for a configured [`FormattingOptions`] object.
    ///
    /// # Examples
    /// ```
    /// let builder = fzgrep::FormattingOptionsBuilder::new().line_number(true).file_name(true);
    /// let options = builder.build();
    /// assert!(options.line_number());
    /// assert!(options.file_name());
    /// ```
    ///
    pub fn build(self) -> FormattingOptions {
        FormattingOptions {
            line_number: self.line_number,
            file_name: self.file_name,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn builder_constructor() {
        let builder = FormattingOptionsBuilder::new();
        assert_eq!(
            builder,
            FormattingOptionsBuilder {
                line_number: false,
                file_name: false
            }
        );
    }

    #[test]
    fn builder_line_number() {
        let builder = FormattingOptionsBuilder::new().line_number(false);
        assert_eq!(
            builder,
            FormattingOptionsBuilder {
                line_number: false,
                file_name: false
            }
        );
        let builder = builder.line_number(true);
        assert_eq!(
            builder,
            FormattingOptionsBuilder {
                line_number: true,
                file_name: false
            }
        );
    }

    #[test]
    fn builder_file_name() {
        let builder = FormattingOptionsBuilder::new().file_name(false);
        assert_eq!(
            builder,
            FormattingOptionsBuilder {
                line_number: false,
                file_name: false
            }
        );
        let builder = builder.file_name(true);
        assert_eq!(
            builder,
            FormattingOptionsBuilder {
                line_number: false,
                file_name: true
            }
        );
    }

    #[test]
    fn builder_build() {
        let options = FormattingOptionsBuilder::new()
            .line_number(true)
            .file_name(true)
            .build();
        assert!(options.line_number());
        assert!(options.file_name());
    }

    #[test]
    fn options_default() {
        let default = FormattingOptions::default();
        assert!(!default.line_number());
        assert!(!default.file_name());
    }

    #[test]
    fn options_line_number() {
        let line_number_off = FormattingOptionsBuilder::new().line_number(false).build();
        assert!(!line_number_off.line_number());
        let line_number_on = FormattingOptionsBuilder::new().line_number(true).build();
        assert!(line_number_on.line_number());
    }

    #[test]
    fn options_file_name() {
        let file_name_off = FormattingOptionsBuilder::new().file_name(false).build();
        assert!(!file_name_off.file_name());
        let file_name_on = FormattingOptionsBuilder::new().file_name(true).build();
        assert!(file_name_on.file_name());
    }
}
