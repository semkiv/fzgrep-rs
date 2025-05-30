pub mod prospective;
pub mod results_collection;
pub mod sliding_accumulator;

use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// A common abstraction over possible content sources: `stdin` or file on disk.
///
pub struct Reader {
    displayed_name: String,
    source: Box<dyn BufRead>,
}

impl Reader {
    /// Creates a [`Reader`] that reads from a file at `path`.
    ///
    /// # Errors:
    ///   * [`std::io::Error`] in case of any I/O errors.
    ///
    pub fn file_reader(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let file = fs::File::open(&path)?;
        let reader = Box::new(BufReader::new(file));
        Ok(Self {
            displayed_name: path.as_ref().to_string_lossy().into_owned(),
            source: reader,
        })
    }

    /// Creates a [`Reader`] that reads from the standard input.
    ///
    pub fn stdin_reader() -> Self {
        Self {
            displayed_name: String::from("(standard input)"),
            source: Box::new(BufReader::new(io::stdin())),
        }
    }

    /// Just a getter for the display name, which is used to differentiate the readers (primarily when logging).
    ///
    pub const fn display_name(&self) -> &String {
        &self.displayed_name
    }

    /// Just a getter that returns the underlying source.
    ///
    pub fn into_source(self) -> Box<dyn BufRead> {
        self.source
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn file_constructor() {
        let tmp = NamedTempFile::new().unwrap();
        let reader = Reader::file_reader(tmp.path()).unwrap();
        assert_eq!(reader.displayed_name, tmp.path().to_string_lossy());
    }

    #[test]
    fn stdin_constructor() {
        let reader = Reader::stdin_reader();
        assert_eq!(reader.displayed_name, "(standard input)");
    }

    #[test]
    fn displayed_name() {
        let tmp = NamedTempFile::new().unwrap();
        let file_reader = Reader::file_reader(tmp.path()).unwrap();
        assert_eq!(file_reader.display_name(), &tmp.path().to_string_lossy());
        let stdin_reader = Reader::stdin_reader();
        assert_eq!(stdin_reader.display_name(), "(standard input)");
    }
}
