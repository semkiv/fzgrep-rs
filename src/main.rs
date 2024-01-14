use fzgrep::cli::args;
use log::error;
use std::{env, io, process};

fn main() -> process::ExitCode {
    let request = args::make_request(env::args());
    // initialize logger
    env_logger::Builder::new()
        .filter_level(request.log_verbosity)
        .init();

    match fzgrep::run(&request, &mut io::stdout()) {
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
