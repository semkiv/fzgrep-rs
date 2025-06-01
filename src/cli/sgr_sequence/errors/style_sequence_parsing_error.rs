pub mod color_sequence_parsing_error;

use color_sequence_parsing_error::ColorSequenceParsingError;

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::num::ParseIntError;

/// Errors that might occur when parsing ASCII SGR style sequences.
///
#[derive(Debug)]
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
