use colored::Colorize;

#[test]
fn default() {
    let args = ["fzgrep", "contigous", "resources/tests/test.txt"];
    let config = fzgrep::Config::new(args.into_iter().map(String::from)).unwrap();
    let matches = fzgrep::find_matches(&config).unwrap();
    let formatted = fzgrep::format_results(matches, config.formatting_options());
    let expected = vec![
        format!(
            "resources/tests/test.txt: {}{}{}{}{}{}u{}{}{} (score 116)",
            "c".blue(),
            "o".blue(),
            "n".blue(),
            "t".blue(),
            "i".blue(),
            "g".blue(),
            "o".blue(),
            "u".blue(),
            "s".blue()
        ),
        format!(
            "resources/tests/test.txt: {}{}{}{}{}{}u{}{}{} (score 115)",
            "C".blue(),
            "o".blue(),
            "n".blue(),
            "t".blue(),
            "i".blue(),
            "g".blue(),
            "o".blue(),
            "u".blue(),
            "s".blue()
        ),
        format!(
            "resources/tests/test.txt: Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols (score 56)",
            "c".blue(),
            "o".blue(),
            "n".blue(),
            "t".blue(),
            "i".blue(),
            "g".blue(),
            "o".blue(),
            "u".blue(),
            "s".blue()
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn line_number_short() {
    let args = ["fzgrep", "-n", "contigous", "resources/tests/test.txt"];
    let config = fzgrep::Config::new(args.into_iter().map(String::from)).unwrap();
    let matches = fzgrep::find_matches(&config).unwrap();
    let formatted = fzgrep::format_results(matches, config.formatting_options());
    let expected = vec![
        format!(
            "resources/tests/test.txt:3: {}{}{}{}{}{}u{}{}{} (score 116)",
            "c".blue(),
            "o".blue(),
            "n".blue(),
            "t".blue(),
            "i".blue(),
            "g".blue(),
            "o".blue(),
            "u".blue(),
            "s".blue()
        ),
        format!(
            "resources/tests/test.txt:4: {}{}{}{}{}{}u{}{}{} (score 115)",
            "C".blue(),
            "o".blue(),
            "n".blue(),
            "t".blue(),
            "i".blue(),
            "g".blue(),
            "o".blue(),
            "u".blue(),
            "s".blue()
        ),
        format!(
            "resources/tests/test.txt:1: Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols (score 56)",
            "c".blue(),
            "o".blue(),
            "n".blue(),
            "t".blue(),
            "i".blue(),
            "g".blue(),
            "o".blue(),
            "u".blue(),
            "s".blue()
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn line_number_long() {
    let args = ["fzgrep", "--line-number", "contigous", "resources/tests/test.txt"];
    let config = fzgrep::Config::new(args.into_iter().map(String::from)).unwrap();
    let matches = fzgrep::find_matches(&config).unwrap();
    let formatted = fzgrep::format_results(matches, config.formatting_options());
    let expected = vec![
        format!(
            "resources/tests/test.txt:3: {}{}{}{}{}{}u{}{}{} (score 116)",
            "c".blue(),
            "o".blue(),
            "n".blue(),
            "t".blue(),
            "i".blue(),
            "g".blue(),
            "o".blue(),
            "u".blue(),
            "s".blue()
        ),
        format!(
            "resources/tests/test.txt:4: {}{}{}{}{}{}u{}{}{} (score 115)",
            "C".blue(),
            "o".blue(),
            "n".blue(),
            "t".blue(),
            "i".blue(),
            "g".blue(),
            "o".blue(),
            "u".blue(),
            "s".blue()
        ),
        format!(
            "resources/tests/test.txt:1: Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols (score 56)",
            "c".blue(),
            "o".blue(),
            "n".blue(),
            "t".blue(),
            "i".blue(),
            "g".blue(),
            "o".blue(),
            "u".blue(),
            "s".blue()
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}
