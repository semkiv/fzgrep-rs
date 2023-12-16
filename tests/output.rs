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
            "{}u{}",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::red("cont").bold(),
            Paint::red('i').bold(),
            Paint::red('g').bold(),
            Paint::red('o').bold(),
            Paint::red('u').bold(),
            Paint::red('s').bold()
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
            "{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}",
            Paint::magenta("resources/tests/тест.txt"),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}",
            Paint::magenta("resources/tests/тест.txt"),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("cont").bold(),
            Paint::red('i').bold(),
            Paint::red('g').bold(),
            Paint::red('o').bold(),
            Paint::red('u').bold(),
            Paint::red('s').bold()
        ),
        format!(
            "{}{}Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::magenta("resources/tests/тест.txt"),
            Paint::cyan(':'),
            Paint::red("cont").bold(),
            Paint::red('i').bold(),
            Paint::red('g').bold(),
            Paint::red('o').bold(),
            Paint::red('u').bold(),
            Paint::red('s').bold()
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
            "{}{}{}u{}",
            Paint::green('3'),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}",
            Paint::green('4'),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::green('1'),
            Paint::cyan(':'),
            Paint::red("cont").bold(),
            Paint::red('i').bold(),
            Paint::red('g').bold(),
            Paint::red('o').bold(),
            Paint::red('u').bold(),
            Paint::red('s').bold()
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
            "{}{}{}u{}",
            Paint::green('3'),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}",
            Paint::green('4'),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::green('1'),
            Paint::cyan(':'),
            Paint::red("cont").bold(),
            Paint::red('i').bold(),
            Paint::red('g').bold(),
            Paint::red('o').bold(),
            Paint::red('u').bold(),
            Paint::red('s').bold()
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
            "{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("cont").bold(),
            Paint::red('i').bold(),
            Paint::red('g').bold(),
            Paint::red('o').bold(),
            Paint::red('u').bold(),
            Paint::red('s').bold()
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
            "{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("cont").bold(),
            Paint::red('i').bold(),
            Paint::red('g').bold(),
            Paint::red('o').bold(),
            Paint::red('u').bold(),
            Paint::red('s').bold()
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
            "{}u{}",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::red("cont").bold(),
            Paint::red('i').bold(),
            Paint::red('g').bold(),
            Paint::red('o').bold(),
            Paint::red('u').bold(),
            Paint::red('s').bold()
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
            "{}u{}",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::red("cont").bold(),
            Paint::red('i').bold(),
            Paint::red('g').bold(),
            Paint::red('o').bold(),
            Paint::red('u').bold(),
            Paint::red('s').bold()
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
            "{}u{}",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::red("cont").bold(),
            Paint::red('i').bold(),
            Paint::red('g').bold(),
            Paint::red('o').bold(),
            Paint::red('u').bold(),
            Paint::red('s').bold()
        ),
        format!(
            "Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::red("cont").bold(),
            Paint::red('i').bold(),
            Paint::red('g').bold(),
            Paint::red('o').bold(),
            Paint::red('u').bold(),
            Paint::red('s').bold()
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
            "{}{}{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::green('3'),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::green('4'),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}{}Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::green('1'),
            Paint::cyan(':'),
            Paint::red("cont").bold(),
            Paint::red('i').bold(),
            Paint::red('g').bold(),
            Paint::red('o').bold(),
            Paint::red('u').bold(),
            Paint::red('s').bold()
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
        "--color",
        "always",
        "--color-overrides",
        "ms=01;33",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from)).unwrap();
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::green('3'),
            Paint::cyan(':'),
            Paint::yellow("contig").bold(),
            Paint::yellow("ous").bold()
        ),
        format!(
            "{}{}{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::green('4'),
            Paint::cyan(':'),
            Paint::yellow("Contig").bold(),
            Paint::yellow("ous").bold()
        ),
        format!(
            "{}{}{}{}Randomly shuffled lines {}ain{}n{} ASCII (upper- and l{}wercase), Cyrillic ({}pper- and lowercase), Chinese and emoji {}ymbols",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::green('1'),
            Paint::cyan(':'),
            Paint::yellow("cont").bold(),
            Paint::yellow('i').bold(),
            Paint::yellow('g').bold(),
            Paint::yellow('o').bold(),
            Paint::yellow('u').bold(),
            Paint::yellow('s').bold()
        ),
    ].join("\n");
    assert_eq!(formatted, expected);
}

todo!("Selected match color test");
todo!("Context match color test");
todo!("Line number color test");
todo!("File name color test");
todo!("Separator color test");
todo!("Selected line color test");
todo!("Context color test");
todo!("Plain formatting test");
