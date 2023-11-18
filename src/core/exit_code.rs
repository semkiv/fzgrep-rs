pub struct ExitCode;

impl ExitCode {
    pub const SUCCESS: u8 = 0;
    pub const NO_MATCHES: u8 = 1;
    pub const FAILURE: u8 = 2;
}
