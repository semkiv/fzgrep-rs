/// Holds various output options.
///
/// Specifically:
///   * whether line numbers should be printed.
///   * whether file names should be printed.
///
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OutputOptions {
    pub line_number: bool,
    pub file_name: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn options_default() {
        let default = OutputOptions::default();
        assert!(!default.line_number);
        assert!(!default.file_name);
    }
}
