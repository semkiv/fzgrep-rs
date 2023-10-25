use colored::Colorize;
use details::Config;
use std::env;
use vscode_fuzzy_score_rs;

/// This function handles all the application logic. The `main` function is merely a `run` call.
/// The query and the target are taken from positional command line argument,
/// the first and the second ones respectively.
pub fn run() {
    let config = Config::new(env::args());
    let fuzzy_match = vscode_fuzzy_score_rs::fuzzy_match(&config.query, &config.target);

    match fuzzy_match {
        None => println!("'{}' does not match '{}'", config.query, config.target),
        Some(fuzzy_match) => {
            let mut colored_target = String::new();
            let mut matches_it = fuzzy_match.positions().iter().peekable();
            for (index, ch) in config.target.chars().enumerate() {
                if matches_it.peek().is_some_and(|pos| **pos == index) {
                    colored_target.push_str(&ch.to_string().blue().to_string());
                    matches_it.next();
                } else {
                    colored_target.push(ch);
                }
            }

            println!("{}, score: {}", colored_target, fuzzy_match.score());
        }
    }
}

mod details {
    use clap::{Arg, ArgMatches, Command};

    #[derive(Debug, PartialEq)]
    pub struct Config {
        pub query: String,
        pub target: String,
    }

    impl Config {
        pub fn new(args: impl Iterator<Item = String>) -> Config {
            let matches = parse_args(args);

            Config {
                query: matches.get_one::<String>("pattern").unwrap().clone(),
                target: matches.get_one::<String>("text").unwrap().clone(),
            }
        }
    }

    fn parse_args(args: impl Iterator<Item = String>) -> ArgMatches {
        Command::new(option_env!("CARGO_NAME").unwrap_or("fzgrep"))
            .version(option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
            .author(option_env!("CARGO_EMAIL").unwrap_or("Andrii Semkiv <semkiv@gmail.com>"))
            .arg(
                Arg::new("pattern")
                    .value_name("PATTERN")
                    .help("Sets the pattern to match")
                    .required(true),
            )
            .arg(
                Arg::new("text")
                    .value_name("TEXT")
                    .help("Sets the text to search in")
                    .required(true),
            )
            .get_matches_from(args)
    }
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
                query: String::from("pattern"),
                target: String::from("file"),
            },
        )
    }
}
