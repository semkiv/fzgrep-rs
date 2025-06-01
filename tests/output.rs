#![expect(clippy::non_ascii_literal, reason = "It's tests, who cares?")]
#![expect(
    clippy::tests_outside_test_module,
    reason = "These are integration tests"
)]
#![expect(clippy::too_many_lines, reason = "It's tests, who cares?")]

use fzgrep::cli;
use std::io::{self, IsTerminal as _};
use std::str;
use yansi::{Condition, Paint as _};

#[test]
fn default_single_file() {
    let cmd = ["fzgrep", "contigous", "resources/tests/test.txt"];
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n",
            "contig"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "ous"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal()))
        ),
        format!(
            "{}u{}\n",
            "Contig"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "ous"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal()))
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            "resources/tests/test.txt"
                .magenta()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "contig"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "ous"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal()))
        ),
        format!(
            "{}{}{}u{}\n",
            "resources/tests/тест.txt"
                .magenta()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "contig"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "ous"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal()))
        ),
        format!(
            "{}{}{}u{}\n",
            "resources/tests/test.txt"
                .magenta()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "Contig"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "ous"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal()))
        ),
        format!(
            "{}{}{}u{}\n",
            "resources/tests/тест.txt"
                .magenta()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "Contig"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "ous"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal()))
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            '2'.green(),
            ':'.cyan(),
            "contig".red().bold(),
            "ous".red().bold()
        ),
        format!(
            "{}{}{}u{}\n",
            '3'.green(),
            ':'.cyan(),
            "Contig".red().bold(),
            "ous".red().bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            '2'.green(),
            ':'.cyan(),
            "contig".red().bold(),
            "ous".red().bold()
        ),
        format!(
            "{}{}{}u{}\n",
            '3'.green(),
            ':'.cyan(),
            "Contig".red().bold(),
            "ous".red().bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            "resources/tests/test.txt".magenta(),
            ':'.cyan(),
            "contig".red().bold(),
            "ous".red().bold()
        ),
        format!(
            "{}{}{}u{}\n",
            "resources/tests/test.txt".magenta(),
            ':'.cyan(),
            "Contig".red().bold(),
            "ous".red().bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            "resources/tests/test.txt".magenta(),
            ':'.cyan(),
            "contig".red().bold(),
            "ous".red().bold()
        ),
        format!(
            "{}{}{}u{}\n",
            "resources/tests/test.txt".magenta(),
            ':'.cyan(),
            "Contig".red().bold(),
            "ous".red().bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!("{}u{}\n", "contig".red().bold(), "ous".red().bold()),
        format!("{}u{}\n", "Contig".red().bold(), "ous".red().bold()),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!("{}u{}\n", "contig".red().bold(), "ous".red().bold()),
        format!("{}u{}\n", "Contig".red().bold(), "ous".red().bold()),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!("{}u{}\n", "contig".red().bold(), "ous".red().bold()),
        format!("{}u{}\n", "contig".red().bold(), "ous".red().bold()),
        format!("{}u{}\n", "Contig".red().bold(), "ous".red().bold()),
        format!("{}u{}\n", "Contig".red().bold(), "ous".red().bold()),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "打电动\n\
            {}u{}\n",
            "contig".red().bold(),
            "ous".red().bold()
        ),
        format!(
            "打电动\n\
            contiguous\n\
            {}u{}\n",
            "Contig".red().bold(),
            "ous".red().bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "打电动\n\
            {}u{}\n",
            "contig".red().bold(),
            "ous".red().bold()
        ),
        format!(
            "打电动\n\
            contiguous\n\
            {}u{}\n",
            "Contig".red().bold(),
            "ous".red().bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n\
            Contiguous\n\
            Текст\n",
            "contig".red().bold(),
            "ous".red().bold()
        ),
        format!(
            "{}u{}\n\
            Текст\n\
            тестування\n",
            "Contig".red().bold(),
            "ous".red().bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n\
            Contiguous\n\
            Текст\n",
            "contig".red().bold(),
            "ous".red().bold()
        ),
        format!(
            "{}u{}\n\
            Текст\n\
            тестування\n",
            "Contig".red().bold(),
            "ous".red().bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!("{}u{}\n", "contig".blue(), "ous".blue()),
        format!("{}u{}\n", "Contig".blue(), "ous".blue()),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n",
            "contig"
                .blue()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "ous"
                .blue()
                .whenever(Condition::cached(io::stdout().is_terminal()))
        ),
        format!(
            "{}u{}\n",
            "Contig"
                .blue()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "ous"
                .blue()
                .whenever(Condition::cached(io::stdout().is_terminal()))
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = ["contiguous\n", "Contiguous\n"].concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}u{}\n",
            "contig".on_yellow().underline(),
            "ous".on_yellow().underline()
        ),
        format!(
            "{}u{}\n",
            "Contig".on_yellow().underline(),
            "ous".on_yellow().underline()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            '2'.new().italic().underline(),
            ':'.cyan(),
            "contig".red().bold(),
            "ous".red().bold()
        ),
        format!(
            "{}{}{}u{}\n",
            '3'.new().italic().underline(),
            ':'.cyan(),
            "Contig".red().bold(),
            "ous".red().bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}u{}\n",
            "resources/tests/test.txt".new().italic().dim(),
            ':'.cyan(),
            "contig".red().bold(),
            "ous".red().bold()
        ),
        format!(
            "{}{}{}u{}\n",
            "resources/tests/test.txt".new().italic().dim(),
            ':'.cyan(),
            "Contig".red().bold(),
            "ous".red().bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}{}{}u{}\n",
            "resources/tests/test.txt".magenta(),
            ':'.fixed(245).dim(),
            '2'.green(),
            ':'.fixed(245).dim(),
            "contig".red().bold(),
            "ous".red().bold()
        ),
        format!(
            "{}{}{}{}{}u{}\n",
            "resources/tests/test.txt".magenta(),
            ':'.fixed(245).dim(),
            '3'.green(),
            ':'.fixed(245).dim(),
            "Contig".red().bold(),
            "ous".red().bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}\n",
            "contig".red().bold(),
            'u'.rgb(192, 255, 238).dim(),
            "ous".red().bold()
        ),
        format!(
            "{}{}{}\n",
            "Contig".red().bold(),
            'u'.rgb(192, 255, 238).dim(),
            "ous".red().bold()
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}\n\
            {}u{}\n\
            {}\n\
            {}\n",
            "打电动".rgb(192, 255, 238).dim(),
            "contig".red().bold(),
            "ous".red().bold(),
            "Contiguous".rgb(192, 255, 238).dim(),
            "Текст".rgb(192, 255, 238).dim(),
        ),
        format!(
            "{}\n\
            {}u{}\n\
            {}\n\
            {}\n",
            "contiguous".rgb(192, 255, 238).dim(),
            "Contig".red().bold(),
            "ous".red().bold(),
            "Текст".rgb(192, 255, 238).dim(),
            "тестування".rgb(192, 255, 238).dim(),
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}{}打电动\n\
            {}{}{}{}{}u{}\n\
            {}{}{}{}Contiguous\n\
            {}{}{}{}Текст\n",
            "resources/tests/test.txt"
                .magenta()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            '1'.green()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "resources/tests/test.txt"
                .magenta()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            '2'.green()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "contig"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "ous"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "resources/tests/test.txt"
                .magenta()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            '3'.green()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "resources/tests/test.txt"
                .magenta()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            '4'.green()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
        ),
        format!(
            "{}{}{}{}contiguous\n\
            {}{}{}{}{}u{}\n\
            {}{}{}{}Текст\n\
            {}{}{}{}тестування\n",
            "resources/tests/test.txt"
                .magenta()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            '2'.green()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "resources/tests/test.txt"
                .magenta()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            '3'.green()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "Contig"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "ous"
                .red()
                .bold()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "resources/tests/test.txt"
                .magenta()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            '4'.green()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            "resources/tests/test.txt"
                .magenta()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            '5'.green()
                .whenever(Condition::cached(io::stdout().is_terminal())),
            ':'.cyan()
                .whenever(Condition::cached(io::stdout().is_terminal())),
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let expected = [
        format!(
            "{}{}{}{}{}\n\
            {}{}{}{}{}{}{}\n\
            {}{}{}{}{}\n\
            {}{}{}{}{}\n",
            "resources/tests/test.txt".rgb(192, 255, 238).italic(),
            ':'.magenta(),
            '1'.rgb(192, 255, 238).italic(),
            ':'.magenta(),
            "打电动".fixed(245).dim(),
            "resources/tests/test.txt".rgb(192, 255, 238).italic(),
            ':'.magenta(),
            '2'.rgb(192, 255, 238).italic(),
            ':'.magenta(),
            "contig".on_yellow(),
            'u'.fixed(245).dim(),
            "ous".on_yellow(),
            "resources/tests/test.txt".rgb(192, 255, 238).italic(),
            ':'.magenta(),
            '3'.rgb(192, 255, 238).italic(),
            ':'.magenta(),
            "Contiguous".fixed(245).dim(),
            "resources/tests/test.txt".rgb(192, 255, 238).italic(),
            ':'.magenta(),
            '4'.rgb(192, 255, 238).italic(),
            ':'.magenta(),
            "Текст".fixed(245).dim(),
        ),
        format!(
            "{}{}{}{}{}\n\
            {}{}{}{}{}{}{}\n\
            {}{}{}{}{}\n\
            {}{}{}{}{}\n",
            "resources/tests/test.txt".rgb(192, 255, 238).italic(),
            ':'.magenta(),
            '2'.rgb(192, 255, 238).italic(),
            ':'.magenta(),
            "contiguous".fixed(245).dim(),
            "resources/tests/test.txt".rgb(192, 255, 238).italic(),
            ':'.magenta(),
            '3'.rgb(192, 255, 238).italic(),
            ':'.magenta(),
            "Contig".on_yellow(),
            'u'.fixed(245).dim(),
            "ous".on_yellow(),
            "resources/tests/test.txt".rgb(192, 255, 238).italic(),
            ':'.magenta(),
            '4'.rgb(192, 255, 238).italic(),
            ':'.magenta(),
            "Текст".fixed(245).dim(),
            "resources/tests/test.txt".rgb(192, 255, 238).italic(),
            ':'.magenta(),
            '5'.rgb(192, 255, 238).italic(),
            ':'.magenta(),
            "тестування".fixed(245).dim(),
        ),
    ]
    .concat();
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), expected);
}

#[test]
fn no_matches_default_single_file() {
    let cmd = ["fzgrep", "nomatch", "resources/tests/test.txt"];
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
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
    let request = cli::make_request(cmd.into_iter().map(String::from));
    let mut buf = Vec::new();
    fzgrep::run(request, &mut buf).unwrap();
    assert_eq!(str::from_utf8(&buf).unwrap(), "");
}
