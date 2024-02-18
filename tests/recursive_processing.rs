use fzgrep::{cli::args, Targets};
use std::path::PathBuf;

#[test]
fn basic_usage() {
    let cmd = [
        "fzgrep",
        "--with-filename",
        "--recursive",
        "recursive",
        "resources/tests/",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    assert_eq!(request.query, "recursive");
    assert_eq!(
        request.targets,
        Targets::RecursiveEntries(vec![PathBuf::from("resources/tests/")])
    );

    let mut results =
        fzgrep::collect_all_matches(&request.query, &request.targets, &request.match_options)
            .unwrap()
            .into_iter()
            .map(|x| x.file_name.unwrap())
            .collect::<Vec<_>>();
    results.sort();
    assert_eq!(
        results,
        [
            "resources/tests/nested/more_nested/test.txt",
            "resources/tests/nested/test.txt",
            "resources/tests/nested/test2.txt",
        ]
    );
}

#[test]
fn basic_usage_no_trailing_slash() {
    let cmd = [
        "fzgrep",
        "--with-filename",
        "--recursive",
        "recursive",
        "resources/tests",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    assert_eq!(request.query, "recursive");
    assert_eq!(
        request.targets,
        Targets::RecursiveEntries(vec![PathBuf::from("resources/tests/")])
    );

    let mut results =
        fzgrep::collect_all_matches(&request.query, &request.targets, &request.match_options)
            .unwrap()
            .into_iter()
            .map(|x| x.file_name.unwrap())
            .collect::<Vec<_>>();
    results.sort();
    assert_eq!(
        results,
        [
            "resources/tests/nested/more_nested/test.txt",
            "resources/tests/nested/test.txt",
            "resources/tests/nested/test2.txt",
        ]
    );
}

#[test]
fn only_files() {
    let cmd = [
        "fzgrep",
        "--with-filename",
        "--recursive",
        "recursive",
        "resources/tests/nested/test.txt",
        "resources/tests/nested/test2.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    assert_eq!(request.query, "recursive");
    assert_eq!(
        request.targets,
        Targets::RecursiveEntries(vec![
            PathBuf::from("resources/tests/nested/test.txt"),
            PathBuf::from("resources/tests/nested/test2.txt")
        ])
    );

    let mut results =
        fzgrep::collect_all_matches(&request.query, &request.targets, &request.match_options)
            .unwrap()
            .into_iter()
            .map(|x| x.file_name.unwrap())
            .collect::<Vec<_>>();
    results.sort();
    assert_eq!(
        results,
        [
            "resources/tests/nested/test.txt",
            "resources/tests/nested/test2.txt",
        ]
    );
}

#[test]
fn files_and_dirs_mixed() {
    let cmd = [
        "fzgrep",
        "--with-filename",
        "--recursive",
        "recursive",
        "resources/tests/nested/more_nested/",
        "resources/tests/nested/test.txt",
        "resources/tests/nested/test2.txt",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    assert_eq!(request.query, "recursive");
    assert_eq!(
        request.targets,
        Targets::RecursiveEntries(vec![
            PathBuf::from("resources/tests/nested/more_nested/"),
            PathBuf::from("resources/tests/nested/test.txt"),
            PathBuf::from("resources/tests/nested/test2.txt")
        ])
    );

    let mut results =
        fzgrep::collect_all_matches(&request.query, &request.targets, &request.match_options)
            .unwrap()
            .into_iter()
            .map(|x| x.file_name.unwrap())
            .collect::<Vec<_>>();
    results.sort();
    assert_eq!(
        results,
        [
            "resources/tests/nested/more_nested/test.txt",
            "resources/tests/nested/test.txt",
            "resources/tests/nested/test2.txt",
        ]
    );
}
