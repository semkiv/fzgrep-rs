use fzgrep::Request;
use yansi::Paint;

#[test]
fn default_single_file() {
    let args = ["fzgrep", "contigous", "resources/tests/test.txt"];
    let request = Request::new(args.into_iter().map(String::from)).unwrap();
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn default_multiple_files() {
    let args = [
        "fzgrep",
        "contigous",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from)).unwrap();
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "resources/tests/test.txt:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/тест.txt:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/test.txt:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/тест.txt:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/test.txt:Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/тест.txt:Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn line_number_short() {
    let args = ["fzgrep", "-n", "contigous", "resources/tests/test.txt"];
    let request = Request::new(args.into_iter().map(String::from)).unwrap();
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "3:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "4:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "1:Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn line_number_long() {
    let args = [
        "fzgrep",
        "--line-number",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from)).unwrap();
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "3:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "4:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "1:Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn with_filename_short() {
    let args = ["fzgrep", "-f", "contigous", "resources/tests/test.txt"];
    let request = Request::new(args.into_iter().map(String::from)).unwrap();
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "resources/tests/test.txt:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/test.txt:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/test.txt:Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn with_filename_long() {
    let args = [
        "fzgrep",
        "--with-filename",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from)).unwrap();
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "resources/tests/test.txt:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/test.txt:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/test.txt:Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn no_filename_short() {
    let args = ["fzgrep", "-F", "contigous", "resources/tests/test.txt"];
    let request = Request::new(args.into_iter().map(String::from)).unwrap();
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn no_filename_long() {
    let args = [
        "fzgrep",
        "--no-filename",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from)).unwrap();
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn no_filename_multiple_files() {
    let args = [
        "fzgrep",
        "--no-filename",
        "contigous",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from)).unwrap();
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn all_options_short() {
    let args = ["fzgrep", "-nf", "contigous", "resources/tests/test.txt"];
    let request = Request::new(args.into_iter().map(String::from)).unwrap();
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "resources/tests/test.txt:3:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/test.txt:4:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/test.txt:1:Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn all_options_long() {
    let args = [
        "fzgrep",
        "--line-number",
        "--with-filename",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from)).unwrap();
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "resources/tests/test.txt:3:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/test.txt:4:{}{}{}{}{}{}u{}{}{}",
            Paint::blue('C'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
        format!(
            "resources/tests/test.txt:1:Randomly shuffled lines {}{}{}{}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::blue('c'),
            Paint::blue('o'),
            Paint::blue('n'),
            Paint::blue('t'),
            Paint::blue('i'),
            Paint::blue('g'),
            Paint::blue('o'),
            Paint::blue('u'),
            Paint::blue('s')
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}
