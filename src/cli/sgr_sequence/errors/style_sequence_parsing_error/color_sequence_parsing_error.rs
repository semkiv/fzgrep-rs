use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::num::ParseIntError;

/// Errors that might occur when parsing ASCII SGR color sequences.
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ColorSequenceParsingError {
    /// Given token is not a code. Codes are expected to be 8-bit unsigned integers (see ASCII SGR sequence).
    /// When a token cannot be parsed as such, this error is raised.
    ///
    /// # Fields
    ///   * a [`String`] containing the problematic token
    ///   * a [`ParseIntError`] containing exact error why parsing failed
    ///
    NotACode(String, ParseIntError),

    /// This error is raised when a sequence ends abruptly
    /// (i.e. more tokens are expected, but there aren't any more).
    ///
    IncompleteSequence,

    /// Raised in case of an unexpected non-standard color type.
    /// ASCII SGR specification has mentions of either `2` (true 24-bit color) or `5` (fixed 8-bit color)
    /// as valid options, so this error gets raised for any other color type codes.
    ///
    /// # Fields
    ///   * a [`u8`] with the offending code
    ///
    BadColorType(u8),

    /// Raised in case of an incorrect 8-bit fixed color sequence
    /// (e.g. when a color code is missing from the sequence).
    ///
    BadFixedColor,

    /// Raised in case of an incorrect 24-bit true color sequence
    /// (e.g. when there are too few color components in the sequence)
    ///
    BadTrueColor,
}

impl Display for ColorSequenceParsingError {
    #[expect(
        clippy::min_ident_chars,
        reason = "Corresponds to the name used in the trait"
    )]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BadColorType(code) => write!(
                f,
                "Code '{code}' is not a valid non-standard color type. Either '2' or '5' is expected"
            ),
            Self::BadFixedColor => write!(
                f,
                "Code '5' (fixed 8-bit color) is expected to be followed by a color code, but there is none"
            ),
            Self::BadTrueColor => write!(
                f,
                "Code '2' (true 24-bit color) is expected to be followed by 3 color components, but there too few"
            ),
            Self::IncompleteSequence => write!(
                f,
                "Code '8' (non-standard color) is expected to be followed by a type code, but there is none"
            ),
            Self::NotACode(seq, err) => write!(f, "'{seq}' is not an 8-bit code: {err}"),
        }
    }
}

impl Error for ColorSequenceParsingError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_bad_fixed_color() {
        let err = ColorSequenceParsingError::BadFixedColor;
        assert_eq!(
            format!("{err}"),
            "Code '5' (fixed 8-bit color) is expected to be followed by a color code, but there is none"
        );
    }

    #[test]
    fn fmt_bad_true_color() {
        let err = ColorSequenceParsingError::BadTrueColor;
        assert_eq!(
            format!("{err}"),
            "Code '2' (true 24-bit color) is expected to be followed by 3 color components, but there too few"
        );
    }

    #[test]
    fn fmt_incomplete_sequence() {
        let err = ColorSequenceParsingError::IncompleteSequence;
        assert_eq!(
            format!("{err}"),
            "Code '8' (non-standard color) is expected to be followed by a type code, but there is none"
        );
    }

    #[test]
    fn fmt_bad_color_type() {
        let err = ColorSequenceParsingError::BadColorType(42);
        assert_eq!(
            format!("{err}"),
            "Code '42' is not a valid non-standard color type. Either '2' or '5' is expected"
        );
    }

    #[test]
    fn fmt_not_a_code() {
        let bad_str = "Test";
        let parse_int_err = bad_str.parse::<u8>().err().unwrap();
        let err = ColorSequenceParsingError::NotACode(String::from(bad_str), parse_int_err.clone());
        assert_eq!(
            format!("{err}"),
            format!("'{bad_str}' is not an 8-bit code: {parse_int_err}")
        );
    }
}
