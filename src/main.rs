use fzgrep::Request;
use log::error;
use std::{env, process};

fn main() -> process::ExitCode {
    match Request::new(env::args()) {
        Ok(request) => {
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
        // logger is not initialized at this point, cannot report error
        Err(_) => process::ExitCode::from(fzgrep::ExitCode::FAILURE),
    }
}
