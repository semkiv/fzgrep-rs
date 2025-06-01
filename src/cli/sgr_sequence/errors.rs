pub mod style_sequence_parsing_error;

use style_sequence_parsing_error::StyleSequenceParsingError;

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Errors that can occur when parsing `grep` formatting sequences.
/// (see [`grep` documentation](https://man7.org/linux/man-pages/man1/grep.1.html#ENVIRONMENT) for more information)
///
#[derive(Debug)]
pub enum ColorOverrideParsingError {
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

impl Display for ColorOverrideParsingError {
    #[expect(
        clippy::min_ident_chars,
        reason = "Corresponds to the name used in the trait"
    )]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BadCapability(cap) => write!(f, "Invalid capability '{cap}'"),
            Self::BadStyleSequence(seq) => write!(f, "Invalid style sequence: {seq}"),
            Self::NotAnOverride(seq) => write!(
                f,
                "Incorrect format: expected '<capability>=<sgr_sequence>', got '{seq}'"
            ),
            Self::UnsupportedCapability(cap) => write!(f, "Capability '{cap}' is not supported"),
        }
    }
}

impl Error for ColorOverrideParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::BadStyleSequence(style_sequence_parsing_error) => {
                Some(style_sequence_parsing_error)
            }
            Self::UnsupportedCapability(_) | Self::NotAnOverride(_) | Self::BadCapability(_) => {
                None
            }
        }
    }
}

impl From<StyleSequenceParsingError> for ColorOverrideParsingError {
    fn from(value: StyleSequenceParsingError) -> Self {
        Self::BadStyleSequence(value)
    }
}

// TODO: test for trait impls
