use std::fmt::{Display, Formatter, Result as FmtResult};
use std::{error::Error, num::ParseIntError};

/// Errors that can occur when parsing `grep` formatting sequences.
/// (see [`grep` documentation](https://man7.org/linux/man-pages/man1/grep.1.html#ENVIRONMENT) for more information)
#[derive(Debug)]
pub(crate) enum ColorOverrideParsingError {
    /// Raised if the given string is not a valid override (i.e. a '<capability>=<formatting>' pair).
    ///
    /// # Fields
    ///   * a [`String`] containing the offending string
    ///
    NotAnOverride(String),
    /// Raised if the style sequence is invalid.
    ///
    /// # Fields:
    ///   * a [`StyleSequenceParsingError`] with a more detailed error
    ///
    BadStyleSequence(StyleSequenceParsingError),
    /// Raised if the requested capability is generally supported by `grep`, but not the program.
    ///
    /// # Fields
    ///   * a [`String`] containing the requested capability
    ///
    UnsupportedCapability(String),
    /// Raised if the requested is not valid.
    /// See [`grep` documentation](https://man7.org/linux/man-pages/man1/grep.1.html#ENVIRONMENT) for the list of possible capabilities.
    ///
    /// # Fields:
    ///   * a [`String`] containing the capability
    ///
    BadCapability(String),
}

/// Errors that might occur when parsing ASCII SGR style sequences.
#[derive(Debug)]
pub(crate) enum StyleSequenceParsingError {
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

/// Errors that might occur when parsing ASCII SGR color sequences.
#[derive(Debug)]
pub(crate) enum ColorSequenceParsingError {
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

impl Display for ColorOverrideParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BadCapability(s) => write!(f, "Invalid capability '{s}'"),
            Self::BadStyleSequence(e) => write!(f, "Invalid style sequence: {e}"),
            Self::NotAnOverride(s) => write!(
                f,
                "Incorrect format: expected '<capability>=<sgr_sequence>', got '{s}'"
            ),
            Self::UnsupportedCapability(s) => write!(f, "Capability '{s}' is not supported"),
        }
    }
}

impl Error for ColorOverrideParsingError {}

impl Display for StyleSequenceParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BadCode(code) => write!(f, "Code '{code}' is invalid"),
            Self::BadColorSequence(e) => write!(f, "Invalid color sequence: {e}"),
            Self::NotACode(s, e) => write!(f, "'{s}' is not an 8-bit code: {e}"),
            Self::UnsupportedCode(code) => write!(f, "Code '{code} is unsupported"),
        }
    }
}

impl Error for StyleSequenceParsingError {}

impl Display for ColorSequenceParsingError {
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
            Self::NotACode(s, e) => write!(f, "'{s}' is not an 8-bit code: {e}"),
        }
    }
}

impl Error for ColorSequenceParsingError {}
