use yansi::{Color, Style};

/// Holds various output options.
///
/// Specifically:
///   * whether line numbers should be printed.
///   * whether file names should be printed.
///   * formatting options.
///
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OutputOptions {
    pub line_number: bool,
    pub file_name: bool,
    pub formatting: FormattingBehavior,
}

/// Holds formatting options for:
///   * the selected match and the selected line itself.
///   * the context match and the context itself.
///   * the line number.
///   * the file name.
///   * the separator between file name/line number/line itself/etc.
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FormattingOptions {
    pub selected_match: Style,
    pub context_match: Style,
    pub line_number: Style,
    pub file_name: Style,
    pub separator: Style,
    pub selected_line: Style,
    pub context: Style,
}

/// Defines rich text output formatting behavior.
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FormattingBehavior {
    /// The formatting should be applied regardless of the output sink type.
    Always(FormattingOptions),
    /// The text should only be formatted if the output is connected to a terminal.
    Auto(FormattingOptions),
    /// The formatting should not be applied at all.
    Never,
}

impl FormattingOptions {
    /// Plain text without any formatting.
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::FormattingOptions;
    /// use yansi::Style;
    ///
    /// let plain = FormattingOptions::plain();
    /// assert_eq!(plain.selected_match, Style::default());
    /// ```
    ///
    pub fn plain() -> Self {
        Self {
            selected_match: Style::default(),
            context_match: Style::default(),
            line_number: Style::default(),
            file_name: Style::default(),
            separator: Style::default(),
            selected_line: Style::default(),
            context: Style::default(),
        }
    }
}

impl Default for FormattingOptions {
    /// Default formatting options that correspond to `grep`'s defaults.
    /// More info, see [`grep` source code](https://git.savannah.gnu.org/cgit/grep.git/tree/src/grep.c?id=102be2bfa571355ff44db39348438a0def1ab382#n299).
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::FormattingOptions;
    /// use yansi::{Color, Style};
    ///
    /// let default = FormattingOptions::default();
    /// assert_eq!(default.selected_match, Style::new(Color::Red).bold());
    /// ```
    ///
    fn default() -> Self {
        Self {
            selected_match: Style::new(Color::Red).bold(),
            context_match: Style::new(Color::Red).bold(),
            line_number: Style::new(Color::Green),
            file_name: Style::new(Color::Magenta),
            separator: Style::new(Color::Cyan),
            selected_line: Style::default(),
            context: Style::default(),
        }
    }
}

impl Default for FormattingBehavior {
    fn default() -> Self {
        FormattingBehavior::Auto(FormattingOptions::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn output_options_default() {
        let default = OutputOptions::default();
        assert!(!default.line_number);
        assert!(!default.file_name);
        assert_eq!(default.formatting, FormattingBehavior::default());
    }

    #[test]
    fn formatting_options_plain() {
        let plain = FormattingOptions::plain();
        assert_eq!(plain.selected_match, Style::default());
        assert_eq!(plain.context_match, Style::default());
        assert_eq!(plain.line_number, Style::default());
        assert_eq!(plain.file_name, Style::default());
        assert_eq!(plain.separator, Style::default());
        assert_eq!(plain.selected_line, Style::default());
        assert_eq!(plain.context, Style::default());
    }

    #[test]
    fn formatting_options_default() {
        let default = FormattingOptions::default();
        assert_eq!(default.selected_match, Style::new(Color::Red).bold());
        assert_eq!(default.context_match, Style::new(Color::Red).bold());
        assert_eq!(default.line_number, Style::new(Color::Green));
        assert_eq!(default.file_name, Style::new(Color::Magenta));
        assert_eq!(default.separator, Style::new(Color::Cyan));
        assert_eq!(default.selected_line, Style::default());
        assert_eq!(default.context, Style::default());
    }
}
