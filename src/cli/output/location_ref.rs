use crate::match_properties::location::Location;
// TODO: doc
#[derive(Debug, Eq, PartialEq)]
pub struct LocationRef<'original> {
    pub source_name: Option<&'original str>,
    pub line_number: Option<&'original usize>,
}

impl<'original> LocationRef<'original> {
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
