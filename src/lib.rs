mod fuzzy_score;

use fuzzy_score::FuzzyMatch;

use ansi_term::{self, Color};
use clap::{App, Arg, ArgMatches};

use std::env;

/// This function handles all the application logic. The `main` function is merely a `run` call.
pub fn run() {
    let config = Config::new(env::args());
    let fuzzy_match = FuzzyMatch::new(&config.pattern, &config.file);
    let score = fuzzy_match.score();
    let mut s = String::new();
    for item in fuzzy_match.matches().iter() {
        if item.is_match {
            s.push_str(&Color::Red.paint(item.character.to_string()).to_string());
        } else {
            s.push(item.character);
        }
    }
    println!(
        "{}, score: {}",
        s,
        Color::Blue.bold().paint(score.to_string())
    );
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
