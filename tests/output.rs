use fzgrep::Request;
use yansi::{Color, Paint};

#[test]
fn default_single_file() {
    let args = ["fzgrep", "contigous", "resources/tests/test.txt"];
    let request = Request::new(args.into_iter().map(String::from));
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
    ]
    .join("\n");
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
    let request = Request::new(args.into_iter().map(String::from));
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
    ]
    .join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn line_number_short() {
    let args = ["fzgrep", "-n", "contigous", "resources/tests/test.txt"];
    let request = Request::new(args.into_iter().map(String::from));
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}u{}",
            Paint::green('2'),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}",
            Paint::green('3'),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .join("\n");
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
    let request = Request::new(args.into_iter().map(String::from));
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}u{}",
            Paint::green('2'),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}",
            Paint::green('3'),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn with_filename_short() {
    let args = ["fzgrep", "-f", "contigous", "resources/tests/test.txt"];
    let request = Request::new(args.into_iter().map(String::from));
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
    ]
    .join("\n");
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
    let request = Request::new(args.into_iter().map(String::from));
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
    ]
    .join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn no_filename_short() {
    let args = ["fzgrep", "-F", "contigous", "resources/tests/test.txt"];
    let request = Request::new(args.into_iter().map(String::from));
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
    ]
    .join("\n");
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
    let request = Request::new(args.into_iter().map(String::from));
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
    ]
    .join("\n");
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
    let request = Request::new(args.into_iter().map(String::from));
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
    ]
    .join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn formatting_color_always() {
    let args = [
        "fzgrep",
        "--color",
        "always",
        "--color-overrides",
        "ms=34",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from));
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!("{}u{}", Paint::blue("contig"), Paint::blue("ous")),
        format!("{}u{}", Paint::blue("Contig"), Paint::blue("ous")),
    ]
    .join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn formatting_color_auto() {
    let args = [
        "fzgrep",
        "--color",
        "auto",
        "--color-overrides",
        "ms=34",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from));
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!("{}u{}", Paint::blue("contig"), Paint::blue("ous")),
        format!("{}u{}", Paint::blue("Contig"), Paint::blue("ous")),
    ]
    .join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn formatting_color_never() {
    let args = [
        "fzgrep",
        "--color",
        "never",
        "--color-overrides",
        "ms=34",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from));
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = ["contiguous", "Contiguous"].join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn formatting_override_selected_match() {
    let args = [
        "fzgrep",
        "--color",
        "always",
        "--color-overrides",
        "ms=4;43",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from));
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}u{}",
            Paint::new("contig").underline().bg(Color::Yellow),
            Paint::new("ous").underline().bg(Color::Yellow)
        ),
        format!(
            "{}u{}",
            Paint::new("Contig").underline().bg(Color::Yellow),
            Paint::new("ous").underline().bg(Color::Yellow)
        ),
    ]
    .join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn formatting_override_line_number() {
    let args = [
        "fzgrep",
        "--line-number",
        "--color",
        "always",
        "--color-overrides",
        "ln=3;4",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from));
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}u{}",
            Paint::new('2').italic().underline(),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}",
            Paint::new('3').italic().underline(),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn formatting_override_file_name() {
    let args = [
        "fzgrep",
        "--with-filename",
        "--color",
        "always",
        "--color-overrides",
        "fn=3;2",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from));
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}u{}",
            Paint::new("resources/tests/test.txt").italic().dimmed(),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}",
            Paint::new("resources/tests/test.txt").italic().dimmed(),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn formatting_override_separator() {
    let args = [
        "fzgrep",
        "--line-number",
        "--with-filename",
        "--color",
        "always",
        "--color-overrides",
        "se=2;38;5;245",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from));
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::fixed(245, ':').dimmed(),
            Paint::green('2'),
            Paint::fixed(245, ':').dimmed(),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::fixed(245, ':').dimmed(),
            Paint::green('3'),
            Paint::fixed(245, ':').dimmed(),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn formatting_override_selected_line() {
    let args = [
        "fzgrep",
        "--color",
        "always",
        "--color-overrides",
        "sl=2;38;2;192;255;238",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from));
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}",
            Paint::red("contig").bold(),
            Paint::rgb(192, 255, 238, 'u').dimmed(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}",
            Paint::red("Contig").bold(),
            Paint::rgb(192, 255, 238, 'u').dimmed(),
            Paint::red("ous").bold()
        ),
    ]
    .join("\n");
    assert_eq!(formatted, expected);
}

#[test]
fn all_options_short() {
    let args = ["fzgrep", "-nf", "contigous", "resources/tests/test.txt"];
    let request = Request::new(args.into_iter().map(String::from));
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::green('2'),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}{}{}u{}",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::green('3'),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .join("\n");
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
        "ms=43:mc=43:ln=3;38;2;192;255;238:fn=3;38;2;192;255;238:sl=2;38;5;245:cx=2;38;5;245:se=35",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from));
    let matches =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive()).unwrap();
    let formatted = fzgrep::format_results(&matches, &request.output_options());
    let expected = [
        format!(
            "{}{}{}{}{}{}{}",
            Paint::rgb(192, 255, 238, "resources/tests/test.txt").italic(),
            Paint::magenta(':'),
            Paint::rgb(192, 255, 238, '2').italic(),
            Paint::magenta(':'),
            Paint::new("contig").bg(Color::Yellow),
            Paint::fixed(245, 'u').dimmed(),
            Paint::new("ous").bg(Color::Yellow)
        ),
        format!(
            "{}{}{}{}{}{}{}",
            Paint::rgb(192, 255, 238, "resources/tests/test.txt").italic(),
            Paint::magenta(':'),
            Paint::rgb(192, 255, 238, '3').italic(),
            Paint::magenta(':'),
            Paint::new("Contig").bg(Color::Yellow),
            Paint::fixed(245, 'u').dimmed(),
            Paint::new("ous").bg(Color::Yellow)
        ),
    ]
    .join("\n");
    assert_eq!(formatted, expected);
}
