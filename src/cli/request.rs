use crate::cli::formatting_options::{FormattingOptions, FormattingOptionsBuilder};
use clap::{Arg, ArgAction, ArgMatches, Command};

/// Represents a run configuration.
///
/// Holds the query, the list of files and the output formatting options.
///
#[derive(Debug)]
pub struct Request {
    query: String,
    targets: Vec<String>,
    formatting_options: FormattingOptions,
}

impl Request {
    /// A constructor that parses a [`String`] iterator into a run configuration.
    ///
    /// `args` can technically be anything that satisfies the requirements,
    /// but in practice it is used just with [`std::env::args`].
    ///
    /// # Errors:
    ///
    ///   * [`Err<String>`] if parsing `args` fails. This can happen in theory fail,
    ///     but it practice it can be caused only by a violation of the constraints imposed by the parser,
    /// in which case it should exit using other mechanism (see below).
    ///   * If `args` do not satisfy internal invariant (e.g. there are too few arguments),
    ///     the parser will cause the program to exit fast using [`std::process::exit`].
    ///
    /// For more info see the [`clap`] crate documentation.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = ["fzgrep", "query", "file1", "file2", "file3"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &vec![String::from("file1"), String::from("file2"), String::from("file3")]);
    /// assert!(!request.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--line-number", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &vec![String::from("file")]);
    /// assert!(request.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--with-filename", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &vec![String::from("file")]);
    /// assert!(request.formatting_options().file_name());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--no-filename", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// assert_eq!(request.targets(), &vec![String::from("file")]);
    /// assert!(!request.formatting_options().file_name());
    /// ```
    ///
    pub fn new(args: impl Iterator<Item = String>) -> Result<Request, String> {
        let matches = parse_args(args);
        let query = matches
            .get_one::<String>("pattern")
            .ok_or(String::from("Missing QUERY argument (required)"))?;

        let targets = matches
            .get_many::<String>("file")
            .map_or(Vec::new(), |files| files.map(String::clone).collect());

        let file_name = matches.get_flag("with_filename")
            || (!matches.get_flag("no_filename") && targets.len() > 1);

        let formatting_options_builder = FormattingOptionsBuilder::new()
            .line_number(matches.get_flag("line_number"))
            .file_name(file_name);

        Ok(Request {
            query: query.clone(),
            targets,
            formatting_options: formatting_options_builder.build(),
        })
    }

    /// A simple getter that just returns the query.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = ["fzgrep", "query"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.query(), "query");
    /// ```
    ///
    pub fn query(&self) -> &str {
        &self.query
    }

    /// A simple getter that just returns the list of files.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = ["fzgrep", "query", "file1", "file2", "file3"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert_eq!(request.targets(), &vec![String::from("file1"), String::from("file2"), String::from("file3")]);
    /// ```
    ///
    pub fn targets(&self) -> &Vec<String> {
        &self.targets
    }

    /// A simple getter that just returns formatting options.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = ["fzgrep", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(!request.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "-n", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--line-number", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.formatting_options().line_number());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--with-filename", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(request.formatting_options().file_name());
    /// ```
    ///
    /// ```
    /// let args = ["fzgrep", "--no-filename", "query", "file"];
    /// let request = fzgrep::Request::new(args.into_iter().map(String::from)).unwrap();
    /// assert!(!request.formatting_options().file_name());
    /// ```
    ///
    pub fn formatting_options(&self) -> FormattingOptions {
        self.formatting_options
    }
}

fn parse_args(args: impl Iterator<Item = String>) -> ArgMatches {
    Command::new(option_env!("CARGO_NAME").unwrap_or("fzgrep"))
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
        .author(option_env!("CARGO_EMAIL").unwrap_or("Andrii Semkiv <semkiv@gmail.com>"))
        .after_help("With more than one FILEs assume -f.")
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .required(true)
                .help("Pattern to match"),
        )
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .num_args(0..)
                .help("Files to search in; if none provided uses standard input"),
        )
        .arg(
            Arg::new("line_number")
                .short('n')
                .long("line-number")
                .action(ArgAction::SetTrue)
                .help("Print line number with matching lines"),
        )
        .arg(
            Arg::new("with_filename")
                .short('f')
                .long("with-filename")
                .action(ArgAction::SetTrue)
                .conflicts_with("no_filename")
                .help("Print file name with output lines"),
        )
        .arg(
            Arg::new("no_filename")
                .short('F')
                .long("no-filename")
                .action(ArgAction::SetTrue)
                .conflicts_with("with_filename")
                .help("Suppress the file name prefix on output"),
        )
        .get_matches_from(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_stdin() -> Result<(), String> {
        let args = ["fzgrep", "Query"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "Query");
        assert_eq!(request.targets(), &Vec::<String>::new());
        assert!(!request.formatting_options().line_number());
        assert!(!request.formatting_options().file_name());
        Ok(())
    }

    #[test]
    fn constructor_single_file() -> Result<(), String> {
        let args = ["fzgrep", "Query", "File"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "Query");
        assert_eq!(request.targets(), &vec![String::from("File")]);
        assert!(!request.formatting_options().line_number());
        assert!(!request.formatting_options().file_name());
        Ok(())
    }

    #[test]
    fn constructor_multiple_files() -> Result<(), String> {
        let args = ["fzgrep", "Query", "File1", "File2", "File3"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "Query");
        assert_eq!(
            request.targets(),
            &vec![
                String::from("File1"),
                String::from("File2"),
                String::from("File3")
            ]
        );
        assert!(!request.formatting_options().line_number());
        assert!(request.formatting_options().file_name());
        Ok(())
    }

    #[test]
    fn constructor_non_ascii_emoji() -> Result<(), String> {
        let args = ["fzgrep", "🐣🦀", "File1", "👨‍🔬.txt", "File3"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "🐣🦀");
        assert_eq!(
            request.targets(),
            &vec![
                String::from("File1"),
                String::from("👨‍🔬.txt"),
                String::from("File3")
            ]
        );
        assert!(!request.formatting_options().line_number());
        assert!(request.formatting_options().file_name());
        Ok(())
    }

    #[test]
    fn constructor_non_ascii_cyrillic() -> Result<(), String> {
        let args = ["fzgrep", "тест", "File1", "тест.txt", "File3"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "тест");
        assert_eq!(
            request.targets(),
            &vec![
                String::from("File1"),
                String::from("тест.txt"),
                String::from("File3")
            ]
        );
        assert!(!request.formatting_options().line_number());
        assert!(request.formatting_options().file_name());
        Ok(())
    }

    #[test]
    fn constructor_non_ascii_chinese() -> Result<(), String> {
        let args = ["fzgrep", "打电", "File1", "测试.txt", "File3"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "打电");
        assert_eq!(
            request.targets(),
            &vec![
                String::from("File1"),
                String::from("测试.txt"),
                String::from("File3")
            ]
        );
        assert!(!request.formatting_options().line_number());
        assert!(request.formatting_options().file_name());
        Ok(())
    }

    #[test]
    fn constructor_line_number_short() -> Result<(), String> {
        let args = ["fzgrep", "-n", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "query");
        assert_eq!(request.targets(), &vec![String::from("file")]);
        assert!(request.formatting_options().line_number());
        Ok(())
    }

    #[test]
    fn constructor_line_number_long() -> Result<(), String> {
        let args = ["fzgrep", "--line-number", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "query");
        assert_eq!(request.targets(), &vec![String::from("file")]);
        assert!(request.formatting_options().line_number());
        Ok(())
    }

    #[test]
    fn constructor_with_file_name_short() -> Result<(), String> {
        let args = ["fzgrep", "-f", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "query");
        assert_eq!(request.targets(), &vec![String::from("file")]);
        assert!(request.formatting_options().file_name());
        Ok(())
    }

    #[test]
    fn constructor_with_file_name_long() -> Result<(), String> {
        let args = ["fzgrep", "--with-filename", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "query");
        assert_eq!(request.targets(), &vec![String::from("file")]);
        assert!(request.formatting_options().file_name());
        Ok(())
    }

    #[test]
    fn constructor_no_file_name_short() -> Result<(), String> {
        let args = ["fzgrep", "-F", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "query");
        assert_eq!(request.targets(), &vec![String::from("file")]);
        assert!(!request.formatting_options().file_name());
        Ok(())
    }

    #[test]
    fn constructor_no_file_name_long() -> Result<(), String> {
        let args = ["fzgrep", "--no-filename", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "query");
        assert_eq!(request.targets(), &vec![String::from("file")]);
        assert!(!request.formatting_options().file_name());
        Ok(())
    }

    #[test]
    fn constructor_all_options_short() -> Result<(), String> {
        let args = ["fzgrep", "-nf", "query", "file"];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "query");
        assert_eq!(request.targets(), &vec![String::from("file")]);
        assert!(request.formatting_options().line_number());
        assert!(request.formatting_options().file_name());
        Ok(())
    }

    #[test]
    fn constructor_all_options_long() -> Result<(), String> {
        let args = [
            "fzgrep",
            "--line-number",
            "--with-filename",
            "query",
            "file",
        ];
        let request = Request::new(args.into_iter().map(String::from))?;
        assert_eq!(request.query(), "query");
        assert_eq!(request.targets(), &vec![String::from("file")]);
        assert!(request.formatting_options().line_number());
        assert!(request.formatting_options().file_name());
        Ok(())
    }

    #[test]
    fn query() {
        let request = Request {
            query: String::from("test"),
            targets: Vec::new(),
            formatting_options: FormattingOptions::default(),
        };
        assert_eq!(request.query(), "test");
    }

    #[test]
    fn targets() {
        let request = Request {
            query: String::from("test"),
            targets: vec![
                String::from("File1"),
                String::from("File2"),
                String::from("File3"),
            ],
            formatting_options: FormattingOptions::default(),
        };
        assert_eq!(
            request.targets(),
            &vec![
                String::from("File1"),
                String::from("File2"),
                String::from("File3")
            ]
        );
    }

    #[test]
    fn formatting_options() {
        let request = Request {
            query: String::from("test"),
            targets: Vec::new(),
            formatting_options: FormattingOptionsBuilder::new()
                .line_number(true)
                .file_name(true)
                .build(),
        };
        assert!(request.formatting_options().line_number());
        assert!(request.formatting_options().file_name());
    }
}
