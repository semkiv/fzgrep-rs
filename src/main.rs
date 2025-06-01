use fzgrep::cli;
use fzgrep::exit_code;

use log::error;
use std::{env, io, process};

fn main() -> process::ExitCode {
    let request = cli::make_request(env::args());
    // initialize logger
    env_logger::Builder::new()
        .filter_level(request.log_verbosity)
        .init();

    match fzgrep::run(request, &mut io::stdout()) {
        Ok(matches) => {
            if matches.is_empty() {
                process::ExitCode::from(exit_code::NO_MATCHES)
            } else {
                process::ExitCode::from(exit_code::SUCCESS)
            }
        }
        Err(err) => {
            error!("Error: {err}");
            process::ExitCode::from(exit_code::FAILURE)
        }
    }
}
