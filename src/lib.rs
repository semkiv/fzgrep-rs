mod fuzzy_score;

use fuzzy_score::FuzzyMatch;

use clap::{App, Arg, ArgMatches};
use colored::Colorize;

use std::env;

/// This function handles all the application logic. The `main` function is merely a `run` call.
pub fn run() {
    let config = Config::new(env::args());
    let fuzzy_match = FuzzyMatch::new(&config.pattern, &config.file);
    let score = fuzzy_match.score();
    let mut s = String::new();
    for item in fuzzy_match.matches().iter() {
        if item.is_match {
            s.push_str(&item.character.to_string().red().to_string());
        } else {
            s.push(item.character);
        }
    }
    println!("{}, score: {}", s, score.to_string().bold().blue());
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
            file: String::from(matches.value_of("text").unwrap()),
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
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Sets the text to search in")
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
