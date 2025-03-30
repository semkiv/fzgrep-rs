use yansi::Style;

/// Controls output formatting.
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Formatting {
    /// Request the output text to be formatted according to the supplied options.
    ///
    On(StyleSet),

    /// Request formatting to be disabled and the output to be just plain text.
    ///
    Off,
}

/// Holds formatting options
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StyleSet {
    /// Style of the selected match (the matching portion of the line)
    ///
    pub selected_match: Style,

    /// Style of the line number.
    ///
    pub line_number: Style,

    /// Style of the file name.
    ///
    pub file_name: Style,

    /// Style of the separator between file name/line number/line itself/etc.
    ///
    pub separator: Style,

    /// Style of the selected line (non-matching part)
    ///
    pub selected_line: Style,

    /// Style of the surrounding context
    ///
    pub context: Style,
}

impl Formatting {
    /// Converts [`Formatting`] to [`Option<FormattingOptions>`].
    /// If `self` is [`Formatting::On`] returns [`Some`] with the inner options, otherwise [`None`].
    ///
    pub(crate) const fn options(&self) -> Option<StyleSet> {
        match self {
            Self::On(options) => Some(*options),
            Self::Off => None,
        }
    }
}

impl Default for StyleSet {
    /// Default formatting options that correspond to `grep`'s defaults.
    /// More info, see [`grep` source code](https://git.savannah.gnu.org/cgit/grep.git/tree/src/grep.c?id=102be2bfa571355ff44db39348438a0def1ab382#n299).
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::cli::formatting::StyleSet;
    /// use yansi::Style;
    ///
    /// let default = StyleSet::default();
    /// assert_eq!(default.selected_match, Style::new().red().bold());
    /// ```
    ///
    fn default() -> Self {
        Self {
            selected_match: Style::new().red().bold(),
            line_number: Style::new().green(),
            file_name: Style::new().magenta(),
            separator: Style::new().cyan(),
            selected_line: Style::new(),
            context: Style::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn formatting_options_default() {
        let default = StyleSet::default();
        assert_eq!(default.selected_match, Style::new().red().bold());
        assert_eq!(default.line_number, Style::new().green());
        assert_eq!(default.file_name, Style::new().magenta());
        assert_eq!(default.separator, Style::new().cyan());
        assert_eq!(default.selected_line, Style::new());
        assert_eq!(default.context, Style::new());
    }

    #[test]
    fn formatting_on_options() {
        let formatting = Formatting::On(StyleSet {
            selected_match: Style::new().blue().bold(),
            ..Default::default()
        });
        assert_eq!(
            formatting.options().unwrap().selected_match,
            Style::new().blue().bold()
        );
    }

    #[test]
    fn formatting_off_options() {
        let formatting = Formatting::Off;
        assert_eq!(formatting.options(), None);
    }
}
