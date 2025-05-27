use crate::cli::output::behavior::Behavior;
use crate::request::Request as CoreRequest;

use log::LevelFilter;

#[derive(Debug, Eq, PartialEq)]
pub struct Request {
    /// Core (i.e. related to the processing itself) part of the request.
    ///
    // TODO: rename?
    pub core: CoreRequest,

    /// Determines the behavior of the program with respect to the output.
    /// [`OutputBehavior::Normal`] means normal output
    /// whereas in case of [`OutputBehavior::Quiet`] the output is fully suppressed
    /// (program exit code can still be used to categorize the run results).
    ///
    pub output_behavior: Behavior,

    /// Controls the verbosity of the logs.
    ///
    pub log_verbosity: LevelFilter,
}
