use crate::match_properties::location::Location;

/// A helper wrapper that facilitates easier sharing of [`Location`].
/// Within one source, the displayed name remains the same whereas the line numbers of matching lines,
/// meaning that the location information cannot be efficiently passed around using [`Location`] as a whole:
/// it would result in unnecessary copies of the source name. Alternative is to decouple the source name
/// and the line number, but it leads to not-so-nice APIs. Here's where this type comes into play:
/// it enables cheaply shared views of the location data while preserving clean APIs.
///
#[derive(Debug, Eq, PartialEq)]
pub struct LocationRef<'original> {
    /// A view of the original location's source name (if available in the original).
    ///
    pub source_name: Option<&'original str>,

    /// A view of the original location's line number (if available in the original).
    ///
    pub line_number: Option<&'original usize>,
}

impl<'original> LocationRef<'original> {
    /// Creates a view of the given location.
    ///
    pub fn new(location: &'original Location) -> Self {
        Self {
            source_name: location.source_name.as_deref(),
            line_number: location.line_number.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_empty() {
        let loc = Location {
            source_name: None,
            line_number: None,
        };

        assert_eq!(
            LocationRef::new(&loc),
            LocationRef {
                source_name: None,
                line_number: None,
            }
        );
    }

    #[test]
    fn constructor_non_empty() {
        let loc = Location {
            source_name: Some(String::from("whatever")),
            line_number: Some(42),
        };

        assert_eq!(
            LocationRef::new(&loc),
            LocationRef {
                source_name: Some(&String::from("whatever")),
                line_number: Some(&42),
            }
        );
    }
}
