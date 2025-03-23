/// A tag struct to give exit code constants a nice namespace.
///
pub struct ExitCode;

impl ExitCode {
    /// Run was successful and at least one match was found.
    ///
    pub const SUCCESS: u8 = 0;
    /// Run was successful but no matches were found.
    ///
    pub const NO_MATCHES: u8 = 1;
    /// Run failed, for the reason check error messages and/or logs.
    ///
    pub const FAILURE: u8 = 2;
}
