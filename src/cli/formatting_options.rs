/// Holds various formatting options.
///
/// Specifically:
///   * whether line numbers should be printed.
///   * whether file names should be printed.
///
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FormattingOptions {
    pub line_number: bool,
    pub file_name: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn options_default() {
        let default = FormattingOptions::default();
        assert!(!default.line_number);
        assert!(!default.file_name);
    }
}
