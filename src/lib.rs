use colored::Colorize;
use details::{Config, FileMatch, MatchingLine};
use log::debug;
use std::error::Error;
use std::fs;
use std::io::{self, BufReader};

/// This function handles all the application logic. The `main` function is merely a `run` call.
/// The query and the target are taken from positional command line argument,
/// the first and the second ones respectively.
pub fn run(command_line_args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let config = Config::new(command_line_args)?;
    debug!("Running with the following configuration: {:?}", config);

    let matches = find_matches(&config)?;
    println!("{}", format_results(matches));

    Ok(())
}

pub fn find_matches(config: &Config) -> Result<Vec<FileMatch>, Box<dyn Error>> {
    let mut matches = Vec::new();
    if config.targets.is_empty() {
        // no files specified => default to stdin
        let stdin_reader = Box::new(BufReader::new(io::stdin()));
        for matching_line in details::process_one_target(&config.query, stdin_reader)
            .map_err(Box::<io::Error>::from)?
        {
            matches.push(FileMatch {
                name: None,
                matching_line,
            });
        }
    } else {
        for filename in &config.targets {
            let file_reader = Box::new(BufReader::new(fs::File::open(filename.clone())?));
            for matching_line in details::process_one_target(&config.query, file_reader)
                .map_err(Box::<io::Error>::from)?
            {
                matches.push(FileMatch {
                    name: Some(filename.clone()),
                    matching_line,
                });
            }
        }
    }

    matches.sort_by(|a, b| {
        b.matching_line
            .fuzzy_match
            .score()
            .cmp(&a.matching_line.fuzzy_match.score())
    });

    Ok(matches)
}

pub fn format_results(matches: Vec<FileMatch>) -> String {
    let mut ret = String::new();
    let mut match_itr = matches.iter().peekable();
    while let Some(m) = match_itr.next() {
        let FileMatch {
            name: filename,
            matching_line:
                MatchingLine {
                    number: line_number,
                    content: line,
                    fuzzy_match,
                },
        } = m;
        let mut colored_target = String::new();
        let mut matches_it = fuzzy_match.positions().iter().peekable();
        for (index, ch) in line.chars().enumerate() {
            if matches_it.peek().is_some_and(|pos| **pos == index) {
                colored_target.push_str(&ch.to_string().blue().to_string());
                matches_it.next();
            } else {
                colored_target.push(ch);
            }
        }

        if let Some(filename) = filename {
            ret.push_str(&format!("{}:", filename));
        }
        ret.push_str(&format!(
            "{}: {} (score {})",
            line_number,
            colored_target,
            fuzzy_match.score()
        ));

        if let Some(_) = match_itr.peek() {
            ret.push('\n');
        }
    }

    ret
}

mod details {
    use clap::{Arg, ArgMatches, Command};
    use std::io::BufRead;

    #[derive(Debug, PartialEq)]
    pub struct Config {
        pub query: String,
        pub targets: Vec<String>,
    }

    impl Config {
        pub fn new(args: impl Iterator<Item = String>) -> Result<Config, String> {
            let matches = parse_args(args);
            let query = matches
                .get_one::<String>("pattern")
                .ok_or(String::from("Missing QUERY argument (required)"))?;

            let targets = matches
                .get_many::<String>("file")
                .map_or(Vec::new(), |files| files.map(String::clone).collect());

            Ok(Config {
                query: query.clone(),
                targets,
            })
        }
    }

    pub struct MatchingLine {
        pub number: usize,
        pub content: String,
        pub fuzzy_match: vscode_fuzzy_score_rs::FuzzyMatch,
    }

    pub struct FileMatch {
        pub name: Option<String>,
        pub matching_line: MatchingLine,
    }

    pub fn process_one_target(
        query: &str,
        target: Box<dyn BufRead>,
    ) -> Result<Vec<MatchingLine>, std::io::Error> {
        let mut ret = Vec::new();
        for (index, line) in target.lines().enumerate() {
            let line = line?;
            if let Some(m) = vscode_fuzzy_score_rs::fuzzy_match(query, &line) {
                ret.push(MatchingLine {
                    number: index + 1,
                    content: line,
                    fuzzy_match: m,
                });
            }
        }

        Ok(ret)
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
                Arg::new("file")
                    .value_name("FILE")
                    .help("Sets the file to search in, if none provided uses stdin")
                    .num_args(0..),
            )
            .get_matches_from(args)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn results_formatting() {
        let results = vec![
            FileMatch {
                name: Some(String::from("First")),
                matching_line: MatchingLine {
                    number: 42,
                    content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                },
            },
            FileMatch {
                name: Some(String::from("Second")),
                matching_line: MatchingLine {
                    number: 100500,
                    content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                },
            },
            FileMatch {
                name: Some(String::from("Third")),
                matching_line: MatchingLine {
                    number: 13,
                    content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                },
            },
        ];
        assert_eq!(
            format_results(results),
            format!("First:42: {}{}st (score 17)\nSecond:100500: tes{} (score 2)\nThird:13: {}{}s{} (score 19)", "t".blue(), "e".blue(), "t".blue(), "t".blue(), "e".blue(), "t".blue())
        )
    }

    #[test]
    fn results_formatting_no_name() {
        let results = vec![
            FileMatch {
                name: None,
                matching_line: MatchingLine {
                    number: 42,
                    content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("te", "test").unwrap(),
                },
            },
            FileMatch {
                name: None,
                matching_line: MatchingLine {
                    number: 100500,
                    content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("t", "test").unwrap(),
                },
            },
            FileMatch {
                name: None,
                matching_line: MatchingLine {
                    number: 13,
                    content: String::from("test"),
                    fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("tet", "test").unwrap(),
                },
            },
        ];
        assert_eq!(
            format_results(results),
            format!(
                "42: {}{}st (score 17)\n100500: tes{} (score 2)\n13: {}{}s{} (score 19)",
                "t".blue(),
                "e".blue(),
                "t".blue(),
                "t".blue(),
                "e".blue(),
                "t".blue()
            )
        )
    }
}
