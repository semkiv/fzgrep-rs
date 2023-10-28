use env_logger;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    fzgrep::run(env::args())
}
