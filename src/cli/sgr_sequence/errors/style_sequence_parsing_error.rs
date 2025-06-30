pub mod color_sequence_parsing_error;

use color_sequence_parsing_error::ColorSequenceParsingError;

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::num::ParseIntError;

/// Errors that might occur when parsing ASCII SGR style sequences.
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StyleSequenceParsingError {
    /// Given token is not a code. Codes are expected to be 8-bit unsigned integers (see ASCII SGR sequence).
    /// When a token cannot be parsed as such, this error is raised.
    ///
    /// # Fields
    ///   * a [`String`] containing the problematic token
    ///   * a [`ParseIntError`] containing exact error why parsing failed
    ///
    NotACode(String, ParseIntError),

    /// Raised if the requested code is not supported by the program.
    /// Unlike in case of a [`StyleSequenceParsingError::BadCode`], the code is well within the specification,
    /// just not supported by the program due to internal limitations.
    ///
    /// # Fields
    ///   * a [`u8`] with the unsupported code
    ///
    UnsupportedCode(u8),

    /// Raised in case of a code that is not compliant with ASCII SGR specification.
    ///
    /// # Fields
    ///   * a [`u8`] with the offending code
    ///
    BadCode(u8),

    /// Raised if the color sequence is not valid.
    ///
    /// # Fields
    ///   * a [`ColorSequenceParsingError`] with more a detailed error
    ///
    BadColorSequence(ColorSequenceParsingError),
}

impl Display for StyleSequenceParsingError {
    #[expect(
        clippy::min_ident_chars,
        reason = "Corresponds to the name used in the trait"
    )]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BadCode(code) => write!(f, "Code '{code}' is invalid"),
            Self::BadColorSequence(seq) => write!(f, "Invalid color sequence: {seq}"),
            Self::NotACode(seq, err) => write!(f, "'{seq}' is not an 8-bit code: {err}"),
            Self::UnsupportedCode(code) => write!(f, "Code '{code} is unsupported"),
        }
    }
}

impl Error for StyleSequenceParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotACode(_, parse_int_error) => Some(parse_int_error),
            Self::BadColorSequence(color_sequence_parsing_error) => {
                Some(color_sequence_parsing_error)
            }
            Self::UnsupportedCode(_) | Self::BadCode(_) => None,
        }
    }
}

impl From<ColorSequenceParsingError> for StyleSequenceParsingError {
    fn from(value: ColorSequenceParsingError) -> Self {
        Self::BadColorSequence(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_bad_code() {
        let bad_code = 200;
        let err = StyleSequenceParsingError::BadCode(bad_code);
        assert_eq!(format!("{err}"), format!("Code '{bad_code}' is invalid"));
    }

    #[test]
    fn fmt_bad_color_sequence() {
        let color_seq_err = ColorSequenceParsingError::BadFixedColor;
        let err = StyleSequenceParsingError::BadColorSequence(color_seq_err.clone());
        assert_eq!(
            format!("{err}"),
            format!("Invalid color sequence: {color_seq_err}")
        );
    }

    #[test]
    fn fmt_not_a_code() {
        let bad_str = "Test";
        let parse_int_err = bad_str.parse::<u8>().err().unwrap();
        let err = StyleSequenceParsingError::NotACode(String::from(bad_str), parse_int_err.clone());
        assert_eq!(
            format!("{err}"),
            format!("'{bad_str}' is not an 8-bit code: {parse_int_err}")
        );
    }

    #[test]
    fn fmt_unsupported_code() {
        let unsupported_code = 100;
        let err = StyleSequenceParsingError::UnsupportedCode(unsupported_code);
        assert_eq!(
            format!("{err}"),
            format!("Code '{unsupported_code} is unsupported")
        );
    }

    #[test]
    fn source_bad_code() {
        let err = StyleSequenceParsingError::BadCode(200);
        assert!(err.source().is_none());
    }

    #[test]
    fn source_bad_color_sequence() {
        let color_seq_err = ColorSequenceParsingError::BadFixedColor;
        let err = StyleSequenceParsingError::BadColorSequence(color_seq_err.clone());
        assert_eq!(
            err.source()
                .unwrap()
                .downcast_ref::<ColorSequenceParsingError>()
                .unwrap(),
            &color_seq_err
        );
    }

    #[test]
    fn source_not_a_code() {
        let bad_str = "Test";
        let parse_int_err = bad_str.parse::<u8>().err().unwrap();
        let err = StyleSequenceParsingError::NotACode(String::from(bad_str), parse_int_err.clone());
        assert_eq!(
            err.source()
                .unwrap()
                .downcast_ref::<ParseIntError>()
                .unwrap(),
            &parse_int_err
        );
    }

    #[test]
    fn source_unsupported_code() {
        let err = StyleSequenceParsingError::UnsupportedCode(100);
        assert!(err.source().is_none());
    }

    #[test]
    fn from_style_sequence_parsing_error() {
        let err = ColorSequenceParsingError::BadFixedColor;
        assert_eq!(
            StyleSequenceParsingError::from(err.clone()),
            StyleSequenceParsingError::BadColorSequence(err)
        );
    }
}
