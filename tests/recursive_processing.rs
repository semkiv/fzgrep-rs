use fzgrep::Request;
use std::{path::PathBuf, error::Error};

#[test]
fn basic_usage() -> Result<(), Box<dyn Error>> {
    let args = ["fzgrep", "--recursive", "test", "resources/tests/"];
    let request = Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "test");
    assert_eq!(
        request.targets(),
        &Some(vec![PathBuf::from("resources/tests/")])
    );

    let results = fzgrep::find_matches(request.query(), request.targets())?;
    assert_eq!(results.len(), 12);

    Ok(())
}

#[test]
fn basic_usage_no_trailing_slash() -> Result<(), Box<dyn Error>> {
    let args = ["fzgrep", "--recursive", "test", "resources/tests"];
    let request = Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "test");
    assert_eq!(
        request.targets(),
        &Some(vec![PathBuf::from("resources/tests")])
    );

    let results = fzgrep::find_matches(request.query(), request.targets())?;
    assert_eq!(results.len(), 12);

    Ok(())
}

#[test]
fn only_files() -> Result<(), Box<dyn Error>> {
    let args = [
        "fzgrep",
        "--recursive",
        "test",
        "resources/tests/nested/test1.txt",
        "resources/tests/nested/test2.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "test");
    assert_eq!(
        request.targets(),
        &Some(vec![
            PathBuf::from("resources/tests/nested/test1.txt"),
            PathBuf::from("resources/tests/nested/test1.txt")
        ])
    );

    let results = fzgrep::find_matches(request.query(), request.targets())?;
    assert_eq!(results.len(), 12);

    Ok(())
}

#[test]
fn files_and_dirs_mixed() -> Result<(), Box<dyn Error>> {
    let args = [
        "fzgrep",
        "--recursive",
        "test",
        "resources/tests/nested/nested/",
        "resources/tests/nested/test1.txt",
        "resources/tests/nested/test2.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "test");
    assert_eq!(
        request.targets(),
        &Some(vec![
            PathBuf::from("resources/tests/nested/nested/"),
            PathBuf::from("resources/tests/nested/test1.txt"),
            PathBuf::from("resources/tests/nested/test1.txt")
        ])
    );

    let results = fzgrep::find_matches(request.query(), request.targets())?;
    assert_eq!(results.len(), 12);

    Ok(())
}
