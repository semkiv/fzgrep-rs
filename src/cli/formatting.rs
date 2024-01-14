use yansi::{Color, Style};

/// Controls output formatting.
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Formatting {
    /// Request the output text to be formatted according to the supplied options.
    ///
    On(FormattingOptions),

    /// Request formatting to be disabled and the output to be just plain text.
    ///
    Off,
}

/// Holds formatting options
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FormattingOptions {
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
    pub(crate) const fn options(&self) -> Option<FormattingOptions> {
        match self {
            Formatting::On(options) => Some(*options),
            Formatting::Off => None,
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
    /// use fzgrep::cli::formatting::FormattingOptions;
    /// use yansi::{Color, Style};
    ///
    /// let default = FormattingOptions::default();
    /// assert_eq!(default.selected_match, Style::new(Color::Red).bold());
    /// ```
    ///
    fn default() -> Self {
        Self {
            selected_match: Style::new(Color::Red).bold(),
            line_number: Style::new(Color::Green),
            file_name: Style::new(Color::Magenta),
            separator: Style::new(Color::Cyan),
            selected_line: Style::default(),
            context: Style::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn formatting_options_default() {
        let default = FormattingOptions::default();
        assert_eq!(default.selected_match, Style::new(Color::Red).bold());
        assert_eq!(default.line_number, Style::new(Color::Green));
        assert_eq!(default.file_name, Style::new(Color::Magenta));
        assert_eq!(default.separator, Style::new(Color::Cyan));
        assert_eq!(default.selected_line, Style::default());
        assert_eq!(default.context, Style::default());
    }

    #[test]
    fn formatting_on_options() {
        let formatting = Formatting::On(FormattingOptions {
            selected_match: Style::new(Color::Blue).bold(),
            ..Default::default()
        });
        assert_eq!(
            formatting.options().unwrap().selected_match,
            Style::new(Color::Blue).bold()
        );
    }

    #[test]
    fn formatting_off_options() {
        let formatting = Formatting::Off;
        assert_eq!(formatting.options(), None);
    }
}
