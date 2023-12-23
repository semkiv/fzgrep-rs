use fzgrep::Request;
use log::error;
use std::{env, process};

fn main() -> process::ExitCode {
    let request = Request::new(env::args());
    // initialize logger
    env_logger::Builder::new()
        .filter_level(request.verbosity())
        .init();

    match fzgrep::run(&request) {
        Ok(matches) => {
            if !matches.is_empty() {
                process::ExitCode::from(fzgrep::ExitCode::SUCCESS)
            } else {
                process::ExitCode::from(fzgrep::ExitCode::NO_MATCHES)
            }
        }
        Err(err) => {
            error!("Error: {err}");
            process::ExitCode::from(fzgrep::ExitCode::FAILURE)
        }
    }
}
