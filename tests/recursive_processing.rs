use fzgrep::Request;
use std::{error::Error, path::PathBuf};

#[test]
fn basic_usage() -> Result<(), Box<dyn Error>> {
    let args = ["fzgrep", "--recursive", "recursive", "resources/tests/"];
    let request = Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "recursive");
    assert_eq!(
        request.targets(),
        &Some(vec![PathBuf::from("resources/tests/")])
    );

    let mut results =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive())?
            .into_iter()
            .map(|x| x.location.file_name)
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

    Ok(())
}

#[test]
fn basic_usage_no_trailing_slash() -> Result<(), Box<dyn Error>> {
    let args = ["fzgrep", "--recursive", "recursive", "resources/tests"];
    let request = Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "recursive");
    assert_eq!(
        request.targets(),
        &Some(vec![PathBuf::from("resources/tests")])
    );

    let mut results =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive())?
            .into_iter()
            .map(|x| x.location.file_name)
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

    Ok(())
}

#[test]
fn only_files() -> Result<(), Box<dyn Error>> {
    let args = [
        "fzgrep",
        "--recursive",
        "recursive",
        "resources/tests/nested/test.txt",
        "resources/tests/nested/test2.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "recursive");
    assert_eq!(
        request.targets(),
        &Some(vec![
            PathBuf::from("resources/tests/nested/test.txt"),
            PathBuf::from("resources/tests/nested/test2.txt")
        ])
    );

    let mut results =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive())?
            .into_iter()
            .map(|x| x.location.file_name)
            .collect::<Vec<_>>();
    results.sort();
    assert_eq!(
        results,
        [
            "resources/tests/nested/test.txt",
            "resources/tests/nested/test2.txt",
        ]
    );

    Ok(())
}

#[test]
fn files_and_dirs_mixed() -> Result<(), Box<dyn Error>> {
    let args = [
        "fzgrep",
        "--recursive",
        "recursive",
        "resources/tests/nested/more_nested/",
        "resources/tests/nested/test.txt",
        "resources/tests/nested/test2.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "recursive");
    assert_eq!(
        request.targets(),
        &Some(vec![
            PathBuf::from("resources/tests/nested/more_nested/"),
            PathBuf::from("resources/tests/nested/test.txt"),
            PathBuf::from("resources/tests/nested/test2.txt")
        ])
    );

    let mut results =
        fzgrep::find_matches(request.query(), request.targets(), request.recursive())?
            .into_iter()
            .map(|x| x.location.file_name)
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

    Ok(())
}
