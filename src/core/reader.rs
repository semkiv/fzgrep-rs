use std::{
    fs,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub struct Reader {
    displayed_name: String,
    source: Box<dyn BufRead>,
}

impl Reader {
    pub fn file_reader(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let file = fs::File::open(&path)?;
        let reader = Box::new(BufReader::new(file));
        Ok(Self {
            displayed_name: path.as_ref().to_string_lossy().into_owned(),
            source: reader,
        })
    }

    pub fn stdin_reader() -> Self {
        Self {
            displayed_name: String::from("(standard input)"),
            source: Box::new(BufReader::new(io::stdin())),
        }
    }

    pub fn displayed_name(&self) -> &String {
        &self.displayed_name
    }

    /// Just a getter that returns the underlying source.
    pub fn source(self) -> Box<dyn BufRead> {
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
        assert_eq!(file_reader.displayed_name(), &tmp.path().to_string_lossy());
        let stdin_reader = Reader::stdin_reader();
        assert_eq!(stdin_reader.displayed_name(), "(standard input)");
    }
}
