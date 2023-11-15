use env_logger;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let request = fzgrep::Request::new(env::args())?;
    env_logger::Builder::new()
        .filter_level(request.verbosity())
        .init();
    fzgrep::run(request)?;
    Ok(())
}
