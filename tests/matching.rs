use fzgrep::Request;
use std::{error::Error, path::PathBuf};

#[test]
fn ascii_query() -> Result<(), Box<dyn Error>> {
    let args = [
        "fzgrep",
        "contigous",
        "resources/tests/👨‍🔬.txt",
        "resources/tests/name with spaces.txt",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
        "resources/tests/测试.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "contigous");
    assert_eq!(
        request.targets(),
        &Some(vec![
            PathBuf::from("resources/tests/👨‍🔬.txt"),
            PathBuf::from("resources/tests/name with spaces.txt"),
            PathBuf::from("resources/tests/test.txt"),
            PathBuf::from("resources/tests/тест.txt"),
            PathBuf::from("resources/tests/测试.txt")
        ])
    );

    let results = fzgrep::find_matches(request.query(), request.targets())?;
    assert_eq!(results.len(), 12);

    assert_eq!(results[0].location.file_name, "resources/tests/👨‍🔬.txt");
    assert_eq!(results[0].location.line_number, 7);
    assert_eq!(results[0].content, String::from("contiguous"));
    assert_eq!(results[0].fuzzy_match.score(), 116);
    assert_eq!(
        results[0].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(results[1].location.file_name, "resources/tests/test.txt");
    assert_eq!(results[1].location.line_number, 3);
    assert_eq!(results[1].content, String::from("contiguous"));
    assert_eq!(results[1].fuzzy_match.score(), 116);
    assert_eq!(
        results[1].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(results[2].location.file_name, "resources/tests/тест.txt");
    assert_eq!(results[2].location.line_number, 6);
    assert_eq!(results[2].content, String::from("contiguous"));
    assert_eq!(results[2].fuzzy_match.score(), 116);
    assert_eq!(
        results[2].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(results[3].location.file_name, "resources/tests/测试.txt");
    assert_eq!(results[3].location.line_number, 4);
    assert_eq!(results[3].content, String::from("contiguous"));
    assert_eq!(results[3].fuzzy_match.score(), 116);
    assert_eq!(
        results[3].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(results[4].location.file_name, "resources/tests/👨‍🔬.txt");
    assert_eq!(results[4].location.line_number, 3);
    assert_eq!(results[4].content, String::from("Contiguous"));
    assert_eq!(results[4].fuzzy_match.score(), 115);
    assert_eq!(
        results[4].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(results[5].location.file_name, "resources/tests/test.txt");
    assert_eq!(results[5].location.line_number, 4);
    assert_eq!(results[5].content, String::from("Contiguous"));
    assert_eq!(results[5].fuzzy_match.score(), 115);
    assert_eq!(
        results[5].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(results[6].location.file_name, "resources/tests/тест.txt");
    assert_eq!(results[6].location.line_number, 7);
    assert_eq!(results[6].content, String::from("Contiguous"));
    assert_eq!(results[6].fuzzy_match.score(), 115);
    assert_eq!(
        results[6].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(results[7].location.file_name, "resources/tests/测试.txt");
    assert_eq!(results[7].location.line_number, 3);
    assert_eq!(results[7].content, String::from("Contiguous"));
    assert_eq!(results[7].fuzzy_match.score(), 115);
    assert_eq!(
        results[7].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(results[8].location.file_name, "resources/tests/👨‍🔬.txt");
    assert_eq!(results[8].location.line_number, 1);
    assert_eq!(results[8].content, String::from("Randomly shuffled lines containing ASCII (upper- and lowercase), Cyrillic (upper- and lowercase), Chinese and emoji symbols"));
    assert_eq!(results[8].fuzzy_match.score(), 56);
    assert_eq!(
        results[8].fuzzy_match.positions(),
        &vec![24, 25, 26, 27, 31, 33, 54, 75, 116]
    );

    assert_eq!(results[9].location.file_name, "resources/tests/test.txt");
    assert_eq!(results[9].location.line_number, 1);
    assert_eq!(results[9].content, String::from("Randomly shuffled lines containing ASCII (upper- and lowercase), Cyrillic (upper- and lowercase), Chinese and emoji symbols"));
    assert_eq!(results[9].fuzzy_match.score(), 56);
    assert_eq!(
        results[9].fuzzy_match.positions(),
        &vec![24, 25, 26, 27, 31, 33, 54, 75, 116]
    );

    assert_eq!(results[10].location.file_name, "resources/tests/тест.txt");
    assert_eq!(results[10].location.line_number, 1);
    assert_eq!(results[10].content, String::from("Randomly shuffled lines containing ASCII (upper- and lowercase), Cyrillic (upper- and lowercase), Chinese and emoji symbols"));
    assert_eq!(results[10].fuzzy_match.score(), 56);
    assert_eq!(
        results[10].fuzzy_match.positions(),
        &vec![24, 25, 26, 27, 31, 33, 54, 75, 116]
    );

    assert_eq!(results[11].location.file_name, "resources/tests/测试.txt");
    assert_eq!(results[11].location.line_number, 1);
    assert_eq!(results[11].content, String::from("Randomly shuffled lines containing ASCII (upper- and lowercase), Cyrillic (upper- and lowercase), Chinese and emoji symbols"));
    assert_eq!(results[11].fuzzy_match.score(), 56);
    assert_eq!(
        results[11].fuzzy_match.positions(),
        &vec![24, 25, 26, 27, 31, 33, 54, 75, 116]
    );

    Ok(())
}

#[test]
fn emoji_query() -> Result<(), Box<dyn Error>> {
    let args = [
        "fzgrep",
        "🐣🦀",
        "resources/tests/name with spaces.txt",
        "resources/tests/test.txt",
        "resources/tests/👨‍🔬.txt",
        "resources/tests/тест.txt",
        "resources/tests/测试.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "🐣🦀");
    assert_eq!(
        request.targets(),
        &Some(vec![
            PathBuf::from("resources/tests/name with spaces.txt"),
            PathBuf::from("resources/tests/test.txt"),
            PathBuf::from("resources/tests/👨‍🔬.txt"),
            PathBuf::from("resources/tests/тест.txt"),
            PathBuf::from("resources/tests/测试.txt")
        ])
    );

    let results = fzgrep::find_matches(request.query(), request.targets())?;
    assert_eq!(results.len(), 4);

    assert_eq!(results[0].location.file_name, "resources/tests/test.txt");
    assert_eq!(results[0].location.line_number, 7);
    assert_eq!(results[0].content, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[0].fuzzy_match.score(), 4);
    assert_eq!(results[0].fuzzy_match.positions(), &vec![1, 3]);

    assert_eq!(results[1].location.file_name, "resources/tests/👨‍🔬.txt");
    assert_eq!(results[1].location.line_number, 6);
    assert_eq!(results[1].content, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[1].fuzzy_match.score(), 4);
    assert_eq!(results[1].fuzzy_match.positions(), &vec![1, 3]);

    assert_eq!(results[2].location.file_name, "resources/tests/тест.txt");
    assert_eq!(results[2].location.line_number, 2);
    assert_eq!(results[2].content, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[2].fuzzy_match.score(), 4);
    assert_eq!(results[2].fuzzy_match.positions(), &vec![1, 3]);

    assert_eq!(results[3].location.file_name, "resources/tests/测试.txt");
    assert_eq!(results[3].location.line_number, 5);
    assert_eq!(results[3].content, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[3].fuzzy_match.score(), 4);
    assert_eq!(results[3].fuzzy_match.positions(), &vec![1, 3]);

    Ok(())
}

#[test]
fn cyrillic_query() -> Result<(), Box<dyn Error>> {
    let args = [
        "fzgrep",
        "тест",
        "resources/tests/name with spaces.txt",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
        "resources/tests/👨‍🔬.txt",
        "resources/tests/测试.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "тест");
    assert_eq!(
        request.targets(),
        &Some(vec![
            PathBuf::from("resources/tests/name with spaces.txt"),
            PathBuf::from("resources/tests/test.txt"),
            PathBuf::from("resources/tests/тест.txt"),
            PathBuf::from("resources/tests/👨‍🔬.txt"),
            PathBuf::from("resources/tests/测试.txt")
        ])
    );

    let results = fzgrep::find_matches(request.query(), request.targets())?;
    assert_eq!(results.len(), 8);

    assert_eq!(results[0].location.file_name, "resources/tests/test.txt");
    assert_eq!(results[0].location.line_number, 6);
    assert_eq!(results[0].content, String::from("тестування"));
    assert_eq!(results[0].fuzzy_match.score(), 46);
    assert_eq!(results[0].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(results[1].location.file_name, "resources/tests/тест.txt");
    assert_eq!(results[1].location.line_number, 5);
    assert_eq!(results[1].content, String::from("тестування"));
    assert_eq!(results[1].fuzzy_match.score(), 46);
    assert_eq!(results[1].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(results[2].location.file_name, "resources/tests/👨‍🔬.txt");
    assert_eq!(results[2].location.line_number, 5);
    assert_eq!(results[2].content, String::from("тестування"));
    assert_eq!(results[2].fuzzy_match.score(), 46);
    assert_eq!(results[2].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(results[3].location.file_name, "resources/tests/测试.txt");
    assert_eq!(results[3].location.line_number, 6);
    assert_eq!(results[3].content, String::from("тестування"));
    assert_eq!(results[3].fuzzy_match.score(), 46);
    assert_eq!(results[3].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(results[4].location.file_name, "resources/tests/test.txt");
    assert_eq!(results[4].location.line_number, 5);
    assert_eq!(results[4].content, String::from("Текст"));
    assert_eq!(results[4].fuzzy_match.score(), 25);
    assert_eq!(results[4].fuzzy_match.positions(), &vec![0, 1, 3, 4]);

    assert_eq!(results[5].location.file_name, "resources/tests/тест.txt");
    assert_eq!(results[5].location.line_number, 3);
    assert_eq!(results[5].content, String::from("Текст"));
    assert_eq!(results[5].fuzzy_match.score(), 25);
    assert_eq!(results[5].fuzzy_match.positions(), &vec![0, 1, 3, 4]);

    assert_eq!(results[6].location.file_name, "resources/tests/👨‍🔬.txt");
    assert_eq!(results[6].location.line_number, 2);
    assert_eq!(results[6].content, String::from("Текст"));
    assert_eq!(results[6].fuzzy_match.score(), 25);
    assert_eq!(results[6].fuzzy_match.positions(), &vec![0, 1, 3, 4]);

    assert_eq!(results[7].location.file_name, "resources/tests/测试.txt");
    assert_eq!(results[7].location.line_number, 7);
    assert_eq!(results[7].content, String::from("Текст"));
    assert_eq!(results[7].fuzzy_match.score(), 25);
    assert_eq!(results[7].fuzzy_match.positions(), &vec![0, 1, 3, 4]);

    Ok(())
}

#[test]
fn chinese_query() -> Result<(), Box<dyn Error>> {
    let args = [
        "fzgrep",
        "打电",
        "resources/tests/name with spaces.txt",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
        "resources/tests/测试.txt",
        "resources/tests/👨‍🔬.txt",
    ];
    let request = Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "打电");
    assert_eq!(
        request.targets(),
        &Some(vec![
            PathBuf::from("resources/tests/name with spaces.txt"),
            PathBuf::from("resources/tests/test.txt"),
            PathBuf::from("resources/tests/тест.txt"),
            PathBuf::from("resources/tests/测试.txt"),
            PathBuf::from("resources/tests/👨‍🔬.txt"),
        ])
    );

    let results = fzgrep::find_matches(request.query(), request.targets())?;
    assert_eq!(results.len(), 4);

    assert_eq!(results[0].location.file_name, "resources/tests/test.txt");
    assert_eq!(results[0].location.line_number, 2);
    assert_eq!(results[0].content, String::from("打电动"));
    assert_eq!(results[0].fuzzy_match.score(), 17);
    assert_eq!(results[0].fuzzy_match.positions(), &vec![0, 1]);

    assert_eq!(results[1].location.file_name, "resources/tests/тест.txt");
    assert_eq!(results[1].location.line_number, 4);
    assert_eq!(results[1].content, String::from("打电动"));
    assert_eq!(results[1].fuzzy_match.score(), 17);
    assert_eq!(results[1].fuzzy_match.positions(), &vec![0, 1]);

    assert_eq!(results[2].location.file_name, "resources/tests/测试.txt");
    assert_eq!(results[2].location.line_number, 2);
    assert_eq!(results[2].content, String::from("打电动"));
    assert_eq!(results[2].fuzzy_match.score(), 17);
    assert_eq!(results[2].fuzzy_match.positions(), &vec![0, 1]);

    assert_eq!(results[3].location.file_name, "resources/tests/👨‍🔬.txt");
    assert_eq!(results[3].location.line_number, 4);
    assert_eq!(results[3].content, String::from("打电动"));
    assert_eq!(results[3].fuzzy_match.score(), 17);
    assert_eq!(results[3].fuzzy_match.positions(), &vec![0, 1]);

    Ok(())
}
