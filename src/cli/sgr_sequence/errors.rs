pub mod style_sequence_parsing_error;

use style_sequence_parsing_error::StyleSequenceParsingError;

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Errors that can occur when parsing `grep` formatting sequences.
/// (see [`grep` documentation](https://man7.org/linux/man-pages/man1/grep.1.html#ENVIRONMENT) for more information)
///
#[derive(Clone, Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_bad_capability() {
        let err = ColorOverrideParsingError::BadCapability(String::from("Test"));
        assert_eq!(format!("{err}"), "Invalid capability 'Test'");
    }

    #[test]
    fn fmt_bad_style_sequence() {
        let bad_style_seq_err = StyleSequenceParsingError::BadCode(100);
        let err = ColorOverrideParsingError::BadStyleSequence(bad_style_seq_err.clone());
        assert_eq!(
            format!("{err}"),
            format!("Invalid style sequence: {bad_style_seq_err}")
        );
    }

    #[test]
    fn fmt_not_an_override() {
        let err = ColorOverrideParsingError::NotAnOverride(String::from("Test"));
        assert_eq!(
            format!("{err}"),
            "Incorrect format: expected '<capability>=<sgr_sequence>', got 'Test'"
        );
    }

    #[test]
    fn fmt_unsupported_capability() {
        let err = ColorOverrideParsingError::UnsupportedCapability(String::from("Test"));
        assert_eq!(format!("{err}"), "Capability 'Test' is not supported");
    }

    #[test]
    fn source_bad_capability() {
        let err = ColorOverrideParsingError::BadCapability(String::from("Test"));
        assert!(err.source().is_none());
    }

    #[test]
    fn source_bad_style_sequence() {
        let source_err = StyleSequenceParsingError::BadCode(200);
        let err = ColorOverrideParsingError::BadStyleSequence(source_err.clone());
        assert_eq!(
            err.source()
                .unwrap()
                .downcast_ref::<StyleSequenceParsingError>()
                .unwrap(),
            &source_err
        );
    }

    #[test]
    fn source_not_an_override() {
        let err = ColorOverrideParsingError::NotAnOverride(String::from("Test"));
        assert!(err.source().is_none());
    }

    #[test]
    fn source_unsupported_capability() {
        let err = ColorOverrideParsingError::UnsupportedCapability(String::from("Test"));
        assert!(err.source().is_none());
    }

    #[test]
    fn from_style_sequence_parsing_error() {
        let err = StyleSequenceParsingError::BadCode(200);
        assert_eq!(
            ColorOverrideParsingError::from(err.clone()),
            ColorOverrideParsingError::BadStyleSequence(err)
        );
    }
}
