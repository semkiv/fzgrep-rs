use crate::cli::formatting::Formatting;
use log::LevelFilter;
use std::path::PathBuf;

/// Matches collection behavior.
///
#[derive(Debug, PartialEq)]
pub enum MatchCollectionStrategy {
    /// All matches must be kept.
    ///
    CollectAll,

    /// Only a number best matches should be kept.
    /// Note that the intended use for this is to reduce the strain on the memory
    /// when the expected number of matches is very high.
    /// Running with this strategy causes more sort operations to happen
    /// so it might even turn out to be slower than collecting all matches
    /// if the total number of matches is relatively low.
    ///
    CollectTop(usize)
}

/// Behavior of the program with respect to the output
///
#[derive(Debug, PartialEq)]
pub enum OutputBehavior {
    /// Output normally.
    ///
    Normal(Formatting),

    /// Output is suppressed, return code can be used to categorize the run results.
    ///
    Quiet,
}

/// Possible categories of input targets.
///
#[derive(Debug, PartialEq)]
pub enum Targets {
    /// A list of files to process.
    ///
    Files(Vec<PathBuf>),

    /// A list of files and/or directories to process.
    /// Files are processed normally, directories are descended into and processed recursively.
    ///
    RecursiveEntries(Vec<PathBuf>),

    /// The standard input.
    ///
    Stdin,
}

/// Represents a run configuration.
///
#[derive(Debug, PartialEq)]
pub struct Request {
    /// The query to match against.
    ///
    pub query: String,

    /// The input targets - files, directories or the standard input.
    ///
    pub targets: Targets,

    /// Matches collection strategy,
    ///
    pub strategy: MatchCollectionStrategy,

    /// Additional data about the matches to be collected.
    ///
    pub match_options: MatchOptions,

    /// Determines the behavior of the program with respect to the output.
    /// [`OutputBehavior::Normal`] means normal output
    /// whereas in case of [`OutputBehavior::Quiet`] the output is fully suppressed
    /// (program exit code can still be used to categorize the run results).
    ///
    pub output_behavior: OutputBehavior,

    /// Control the verbosity of the logs.
    ///
    pub log_verbosity: LevelFilter,
}

/// Represents a set of options that control how the additional data about matches is collected.
#[derive(Debug, PartialEq)]
pub struct MatchOptions {
    /// Determines whether the numbers of matching lines are of interest and should be tracked during processing.
    ///
    pub track_line_numbers: bool,

    /// Determines whether the names of the files containing matching lines are of interest
    /// and should be tracked during processing.
    ///
    pub track_file_names: bool,

    /// Controls the size (numbers of lines before and after) of the context surrounding the matching line.
    ///
    pub context_size: ContextSize,
}

/// A thin new-type wrapper that represents a number of lines of text.
///
#[derive(Debug, PartialEq)]
pub struct Lines(pub usize);

/// Represents the size of the context surrounding the matching line.
///
#[derive(Debug, PartialEq)]
pub struct ContextSize {
    /// (Maximum) number of lines preceding the matching line.
    ///
    pub before: Lines,

    /// (Maximum) number of lines following the matching line.
    pub after: Lines,
}

impl OutputBehavior {
    /// Converts [`OutputBehavior`] to [`Option<Formatting>`].
    /// If `self` is [`OutputBehavior::Normal`] returns [`Some`] with the inner formatting, otherwise [`None`].
    /// Note that this method is not intended to be used in the real code, it is there to simplify the validation of tests.
    ///
    /// # Examples
    ///
    /// ```
    /// use fzgrep::{Formatting, FormattingOptions, OutputBehavior};
    ///
    /// let formatting = Formatting::On(FormattingOptions::default());
    /// assert_eq!(formatting.formatting().unwrap(), Formatting::On(FormattingOptions::default()));
    /// ```
    ///
    /// ```
    /// let behavior = OutputBehavior::Quiet;
    /// assert_eq!(behavior.formatting(), None);
    /// ```
    ///
    #[cfg(test)]
    pub(crate) const fn formatting(&self) -> Option<Formatting> {
        match self {
            OutputBehavior::Normal(formatting) => Some(*formatting),
            OutputBehavior::Quiet => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cli::formatting::FormattingOptions;

    #[test]
    fn output_behavior_to_formatting() {
        let behavior = OutputBehavior::Normal(Formatting::On(FormattingOptions::default()));
        assert_eq!(
            behavior.formatting(),
            Some(Formatting::On(FormattingOptions::default()))
        );
        let behavior = OutputBehavior::Quiet;
        assert_eq!(behavior.formatting(), None);
    }
}
