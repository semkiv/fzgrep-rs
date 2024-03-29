use atty::Stream;
use fzgrep::cli::args;
use std::str;
use yansi::{Color, Paint};

#[test]
fn default_single_file() {
    let cmd = ["fzgrep", "contigous", "resources/tests/test.txt"];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n",
            if atty::is(Stream::Stdout) {
                Paint::red("contig").bold().to_string()
            } else {
                String::from("contig")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("ous").bold().to_string()
            } else {
                String::from("ous")
            }
        ),
        format!(
            "{}u{}\n",
            if atty::is(Stream::Stdout) {
                Paint::red("Contig").bold().to_string()
            } else {
                String::from("Contig")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("ous").bold().to_string()
            } else {
                String::from("ous")
            }
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn default_multiple_files() {
    let cmd = [
        "fzgrep",
        "contigous",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            if atty::is(Stream::Stdout) {
                Paint::magenta("resources/tests/test.txt").to_string()
            } else {
                String::from("resources/tests/test.txt")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("contig").bold().to_string()
            } else {
                String::from("contig")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("ous").bold().to_string()
            } else {
                String::from("ous")
            }
        ),
        format!(
            "{}{}{}u{}\n",
            if atty::is(Stream::Stdout) {
                Paint::magenta("resources/tests/тест.txt").to_string()
            } else {
                String::from("resources/tests/тест.txt")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("contig").bold().to_string()
            } else {
                String::from("contig")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("ous").bold().to_string()
            } else {
                String::from("ous")
            }
        ),
        format!(
            "{}{}{}u{}\n",
            if atty::is(Stream::Stdout) {
                Paint::magenta("resources/tests/test.txt").to_string()
            } else {
                String::from("resources/tests/test.txt")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("Contig").bold().to_string()
            } else {
                String::from("Contig")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("ous").bold().to_string()
            } else {
                String::from("ous")
            }
        ),
        format!(
            "{}{}{}u{}\n",
            if atty::is(Stream::Stdout) {
                Paint::magenta("resources/tests/тест.txt").to_string()
            } else {
                String::from("resources/tests/тест.txt")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("Contig").bold().to_string()
            } else {
                String::from("Contig")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("ous").bold().to_string()
            } else {
                String::from("ous")
            }
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn line_number_short() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "-n",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            Paint::green('2'),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}\n",
            Paint::green('3'),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn line_number_long() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "--line-number",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            Paint::green('2'),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}\n",
            Paint::green('3'),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn with_filename_short() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "-f",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}\n",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn with_filename_long() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "--with-filename",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}\n",
            Paint::magenta("resources/tests/test.txt"),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn no_filename_short() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "-F",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}\n",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn no_filename_long() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "--no-filename",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}\n",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn no_filename_multiple_files() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "--no-filename",
        "contigous",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}\n",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}\n",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}\n",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn before_context_short() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "-B",
        "2",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "打电动\n\
            {}u{}\n",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "打电动\n\
            contiguous\n\
            {}u{}\n",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn before_context_long() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "--before-context",
        "2",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "打电动\n\
            {}u{}\n",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "打电动\n\
            contiguous\n\
            {}u{}\n",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn after_context_short() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "-A",
        "2",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n\
            Contiguous\n\
            Текст\n",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}\n\
            Текст\n\
            тестування\n",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn after_context_long() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "--after-context",
        "2",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n\
            Contiguous\n\
            Текст\n",
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}u{}\n\
            Текст\n\
            тестування\n",
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn formatting_color_always() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "--color-overrides",
        "ms=34",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!("{}u{}\n", Paint::blue("contig"), Paint::blue("ous")),
        format!("{}u{}\n", Paint::blue("Contig"), Paint::blue("ous")),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn formatting_color_auto() {
    let cmd = [
        "fzgrep",
        "--color",
        "auto",
        "--color-overrides",
        "ms=34",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n",
            if atty::is(Stream::Stdout) {
                Paint::blue("contig").to_string()
            } else {
                String::from("contig")
            },
            if atty::is(Stream::Stdout) {
                Paint::blue("ous").to_string()
            } else {
                String::from("ous")
            }
        ),
        format!(
            "{}u{}\n",
            if atty::is(Stream::Stdout) {
                Paint::blue("Contig").to_string()
            } else {
                String::from("Contig")
            },
            if atty::is(Stream::Stdout) {
                Paint::blue("ous").to_string()
            } else {
                String::from("ous")
            }
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn formatting_color_never() {
    let cmd = [
        "fzgrep",
        "--color",
        "never",
        "--color-overrides",
        "ms=34",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = ["contiguous\n", "Contiguous\n"].concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn formatting_override_selected_match() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "--color-overrides",
        "ms=4;43",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n",
            Paint::new("contig").underline().bg(Color::Yellow),
            Paint::new("ous").underline().bg(Color::Yellow)
        ),
        format!(
            "{}u{}\n",
            Paint::new("Contig").underline().bg(Color::Yellow),
            Paint::new("ous").underline().bg(Color::Yellow)
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn formatting_override_line_number() {
    let cmd = [
        "fzgrep",
        "--line-number",
        "--color",
        "always",
        "--color-overrides",
        "ln=3;4",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            Paint::new('2').italic().underline(),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}\n",
            Paint::new('3').italic().underline(),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn formatting_override_file_name() {
    let cmd = [
        "fzgrep",
        "--with-filename",
        "--color",
        "always",
        "--color-overrides",
        "fn=3;2",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            Paint::new("resources/tests/test.txt").italic().dimmed(),
            Paint::cyan(':'),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}u{}\n",
            Paint::new("resources/tests/test.txt").italic().dimmed(),
            Paint::cyan(':'),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn formatting_override_separator() {
    let cmd = [
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
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}{}{}u{}\n",
            Paint::magenta("resources/tests/test.txt"),
            Paint::fixed(245, ':').dimmed(),
            Paint::green('2'),
            Paint::fixed(245, ':').dimmed(),
            Paint::red("contig").bold(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}{}{}u{}\n",
            Paint::magenta("resources/tests/test.txt"),
            Paint::fixed(245, ':').dimmed(),
            Paint::green('3'),
            Paint::fixed(245, ':').dimmed(),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn formatting_override_selected_line() {
    let cmd = [
        "fzgrep",
        "--color",
        "always",
        "--color-overrides",
        "sl=2;38;2;192;255;238",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}\n",
            Paint::red("contig").bold(),
            Paint::rgb(192, 255, 238, 'u').dimmed(),
            Paint::red("ous").bold()
        ),
        format!(
            "{}{}{}\n",
            Paint::red("Contig").bold(),
            Paint::rgb(192, 255, 238, 'u').dimmed(),
            Paint::red("ous").bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn formatting_override_context() {
    let cmd = [
        "fzgrep",
        "--before-context",
        "1",
        "--after-context",
        "2",
        "--color",
        "always",
        "--color-overrides",
        "cx=2;38;2;192;255;238",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}\n\
            {}u{}\n\
            {}\n\
            {}\n",
            Paint::rgb(192, 255, 238, "打电动").dimmed(),
            Paint::red("contig").bold(),
            Paint::red("ous").bold(),
            Paint::rgb(192, 255, 238, "Contiguous").dimmed(),
            Paint::rgb(192, 255, 238, "Текст").dimmed(),
        ),
        format!(
            "{}\n\
            {}u{}\n\
            {}\n\
            {}\n",
            Paint::rgb(192, 255, 238, "contiguous").dimmed(),
            Paint::red("Contig").bold(),
            Paint::red("ous").bold(),
            Paint::rgb(192, 255, 238, "Текст").dimmed(),
            Paint::rgb(192, 255, 238, "тестування").dimmed(),
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn all_options_short() {
    let cmd = [
        "fzgrep",
        "-nf",
        "-B",
        "1",
        "-A",
        "2",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}{}打电动\n\
            {}{}{}{}{}u{}\n\
            {}{}{}{}Contiguous\n\
            {}{}{}{}Текст\n",
            if atty::is(Stream::Stdout) {
                Paint::magenta("resources/tests/test.txt").to_string()
            } else {
                String::from("resources/tests/test.txt")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::green('1').to_string()
            } else {
                String::from("1")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::magenta("resources/tests/test.txt").to_string()
            } else {
                String::from("resources/tests/test.txt")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::green('2').to_string()
            } else {
                String::from("2")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("contig").bold().to_string()
            } else {
                String::from("contig")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("ous").bold().to_string()
            } else {
                String::from("ous")
            },
            if atty::is(Stream::Stdout) {
                Paint::magenta("resources/tests/test.txt").to_string()
            } else {
                String::from("resources/tests/test.txt")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::green('3').to_string()
            } else {
                String::from("3")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::magenta("resources/tests/test.txt").to_string()
            } else {
                String::from("resources/tests/test.txt")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::green('4').to_string()
            } else {
                String::from("4")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            }
        ),
        format!(
            "{}{}{}{}contiguous\n\
            {}{}{}{}{}u{}\n\
            {}{}{}{}Текст\n\
            {}{}{}{}тестування\n",
            if atty::is(Stream::Stdout) {
                Paint::magenta("resources/tests/test.txt").to_string()
            } else {
                String::from("resources/tests/test.txt")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::green('2').to_string()
            } else {
                String::from("2")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::magenta("resources/tests/test.txt").to_string()
            } else {
                String::from("resources/tests/test.txt")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::green('3').to_string()
            } else {
                String::from("3")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("Contig").bold().to_string()
            } else {
                String::from("Contig")
            },
            if atty::is(Stream::Stdout) {
                Paint::red("ous").bold().to_string()
            } else {
                String::from("ous")
            },
            if atty::is(Stream::Stdout) {
                Paint::magenta("resources/tests/test.txt").to_string()
            } else {
                String::from("resources/tests/test.txt")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::green('4').to_string()
            } else {
                String::from("4")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::magenta("resources/tests/test.txt").to_string()
            } else {
                String::from("resources/tests/test.txt")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
            if atty::is(Stream::Stdout) {
                Paint::green('5').to_string()
            } else {
                String::from("5")
            },
            if atty::is(Stream::Stdout) {
                Paint::cyan(':').to_string()
            } else {
                String::from(":")
            },
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn all_options_long() {
    let cmd = [
        "fzgrep",
        "--line-number",
        "--with-filename",
        "--before-context",
        "1",
        "--after-context",
        "2",
        "--color",
        "always",
        "--color-overrides",
        "ms=43:ln=3;38;2;192;255;238:fn=3;38;2;192;255;238:sl=2;38;5;245:cx=2;38;5;245:se=35",
        "contigous",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}{}{}\n\
            {}{}{}{}{}{}{}\n\
            {}{}{}{}{}\n\
            {}{}{}{}{}\n",
            Paint::rgb(192, 255, 238, "resources/tests/test.txt").italic(),
            Paint::magenta(':'),
            Paint::rgb(192, 255, 238, '1').italic(),
            Paint::magenta(':'),
            Paint::fixed(245, "打电动").dimmed(),
            Paint::rgb(192, 255, 238, "resources/tests/test.txt").italic(),
            Paint::magenta(':'),
            Paint::rgb(192, 255, 238, '2').italic(),
            Paint::magenta(':'),
            Paint::new("contig").bg(Color::Yellow),
            Paint::fixed(245, 'u').dimmed(),
            Paint::new("ous").bg(Color::Yellow),
            Paint::rgb(192, 255, 238, "resources/tests/test.txt").italic(),
            Paint::magenta(':'),
            Paint::rgb(192, 255, 238, '3').italic(),
            Paint::magenta(':'),
            Paint::fixed(245, "Contiguous").dimmed(),
            Paint::rgb(192, 255, 238, "resources/tests/test.txt").italic(),
            Paint::magenta(':'),
            Paint::rgb(192, 255, 238, '4').italic(),
            Paint::magenta(':'),
            Paint::fixed(245, "Текст").dimmed(),
        ),
        format!(
            "{}{}{}{}{}\n\
            {}{}{}{}{}{}{}\n\
            {}{}{}{}{}\n\
            {}{}{}{}{}\n",
            Paint::rgb(192, 255, 238, "resources/tests/test.txt").italic(),
            Paint::magenta(':'),
            Paint::rgb(192, 255, 238, '2').italic(),
            Paint::magenta(':'),
            Paint::fixed(245, "contiguous").dimmed(),
            Paint::rgb(192, 255, 238, "resources/tests/test.txt").italic(),
            Paint::magenta(':'),
            Paint::rgb(192, 255, 238, '3').italic(),
            Paint::magenta(':'),
            Paint::new("Contig").bg(Color::Yellow),
            Paint::fixed(245, 'u').dimmed(),
            Paint::new("ous").bg(Color::Yellow),
            Paint::rgb(192, 255, 238, "resources/tests/test.txt").italic(),
            Paint::magenta(':'),
            Paint::rgb(192, 255, 238, '4').italic(),
            Paint::magenta(':'),
            Paint::fixed(245, "Текст").dimmed(),
            Paint::rgb(192, 255, 238, "resources/tests/test.txt").italic(),
            Paint::magenta(':'),
            Paint::rgb(192, 255, 238, '5').italic(),
            Paint::magenta(':'),
            Paint::fixed(245, "тестування").dimmed(),
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn no_matches_default_single_file() {
    let cmd = ["fzgrep", "nomatch", "resources/tests/test.txt"];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), "");
}

#[test]
fn no_matches_all_options_long() {
    let cmd = [
        "fzgrep",
        "--line-number",
        "--with-filename",
        "--color",
        "always",
        "--color-overrides",
        "ms=43:ln=3;38;2;192;255;238:fn=3;38;2;192;255;238:sl=2;38;5;245:cx=2;38;5;245:se=35",
        "nomatch",
        "resources/tests/test.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    let mut buf = Vec::new();
    fzgrep::run(&request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), "");
}
