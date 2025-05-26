pub mod filter;

use filter::Filter;

use std::path::PathBuf;

/// Possible categories of input targets.
///
#[derive(Debug, Eq, PartialEq)]
pub enum Targets {
    /// A list of files to process.
    ///
    Files(Vec<PathBuf>),

    /// A list of files and/or directories to process.
    /// Files are processed normally, directories are descended into and processed recursively.
    ///
    RecursiveEntries {
        paths: Vec<PathBuf>,
        filter: Option<Filter>,
    },

    /// The standard input.
    ///
    Stdin,
}
