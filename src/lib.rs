use clap::{App, Arg, ArgMatches};

use std::env;

/// This function handles all the application logic. The `main` function is merely a `run` call.
pub fn run() {
    let config = Config::new(env::args());
    println!("pattern: {}, file: {}", config.pattern, config.file);
}

#[derive(Debug, PartialEq)]
struct Config {
    pattern: String,
    file: String,
}

impl Config {
    fn new(args: impl Iterator<Item = String>) -> Config {
        let matches = parse_args(args);

        Config {
            pattern: String::from(matches.value_of("pattern").unwrap()),
            file: String::from(matches.value_of("file").unwrap()),
        }
    }
}

fn parse_args<'a>(args: impl Iterator<Item = String>) -> ArgMatches<'a> {
    App::new(option_env!("CARGO_NAME").unwrap_or("fzgrep"))
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
        .author(option_env!("CARGO_EMAIL").unwrap_or("Andrii Semkiv <semkiv@gmail.com>"))
        .arg(
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .help("Sets the pattern to match")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Sets the file to search in")
                .takes_value(true)
                .required(true),
        )
        .get_matches_from(args)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn positional_arguments_parsing() {
        let args = vec![
            String::from("fzgrep"),
            String::from("pattern"),
            String::from("file"),
        ];

        assert_eq!(
            Config::new(args.into_iter()),
            Config {
                pattern: String::from("pattern"),
                file: String::from("file"),
            },
        )
    }
}
