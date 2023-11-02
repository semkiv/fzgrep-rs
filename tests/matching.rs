#[test]
fn ascii_query() -> Result<(), String> {
    let args = [
        "fzgrep",
        "contigous",
        "resources/tests/👨‍🔬.txt",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
        "resources/tests/测试.txt",
    ];
    let request = fzgrep::Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "contigous");
    assert_eq!(
        request.targets(),
        &vec![
            String::from("resources/tests/👨‍🔬.txt"),
            String::from("resources/tests/test.txt"),
            String::from("resources/tests/тест.txt"),
            String::from("resources/tests/测试.txt")
        ]
    );

    let results = fzgrep::find_matches(&request).unwrap();
    assert_eq!(results.len(), 12);

    assert_eq!(
        results[0].filename.clone().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[0].matching_line.line_number, 7);
    assert_eq!(
        results[0].matching_line.line_content,
        String::from("contiguous")
    );
    assert_eq!(results[0].matching_line.fuzzy_match.score(), 116);
    assert_eq!(
        results[0].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[1].filename.clone().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[1].matching_line.line_number, 3);
    assert_eq!(
        results[1].matching_line.line_content,
        String::from("contiguous")
    );
    assert_eq!(results[1].matching_line.fuzzy_match.score(), 116);
    assert_eq!(
        results[1].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[2].filename.clone().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[2].matching_line.line_number, 6);
    assert_eq!(
        results[2].matching_line.line_content,
        String::from("contiguous")
    );
    assert_eq!(results[2].matching_line.fuzzy_match.score(), 116);
    assert_eq!(
        results[2].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[3].filename.clone().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[3].matching_line.line_number, 4);
    assert_eq!(
        results[3].matching_line.line_content,
        String::from("contiguous")
    );
    assert_eq!(results[3].matching_line.fuzzy_match.score(), 116);
    assert_eq!(
        results[3].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[4].filename.clone().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[4].matching_line.line_number, 3);
    assert_eq!(
        results[4].matching_line.line_content,
        String::from("Contiguous")
    );
    assert_eq!(results[4].matching_line.fuzzy_match.score(), 115);
    assert_eq!(
        results[4].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[5].filename.clone().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[5].matching_line.line_number, 4);
    assert_eq!(
        results[5].matching_line.line_content,
        String::from("Contiguous")
    );
    assert_eq!(results[5].matching_line.fuzzy_match.score(), 115);
    assert_eq!(
        results[5].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[6].filename.clone().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[6].matching_line.line_number, 7);
    assert_eq!(
        results[6].matching_line.line_content,
        String::from("Contiguous")
    );
    assert_eq!(results[6].matching_line.fuzzy_match.score(), 115);
    assert_eq!(
        results[6].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[7].filename.clone().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[7].matching_line.line_number, 3);
    assert_eq!(
        results[7].matching_line.line_content,
        String::from("Contiguous")
    );
    assert_eq!(results[7].matching_line.fuzzy_match.score(), 115);
    assert_eq!(
        results[7].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[8].filename.clone().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[8].matching_line.line_number, 1);
    assert_eq!(results[8].matching_line.line_content, String::from("Randomly shuffled lines containing ASCII (upper- and lowercase), Cyrillic (upper- and lowercase), Chinese and emoji symbols"));
    assert_eq!(results[8].matching_line.fuzzy_match.score(), 56);
    assert_eq!(
        results[8].matching_line.fuzzy_match.positions(),
        &vec![24, 25, 26, 27, 31, 33, 54, 75, 116]
    );

    assert_eq!(
        results[9].filename.clone().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[9].matching_line.line_number, 1);
    assert_eq!(results[9].matching_line.line_content, String::from("Randomly shuffled lines containing ASCII (upper- and lowercase), Cyrillic (upper- and lowercase), Chinese and emoji symbols"));
    assert_eq!(results[9].matching_line.fuzzy_match.score(), 56);
    assert_eq!(
        results[9].matching_line.fuzzy_match.positions(),
        &vec![24, 25, 26, 27, 31, 33, 54, 75, 116]
    );

    assert_eq!(
        results[10].filename.clone().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[10].matching_line.line_number, 1);
    assert_eq!(results[10].matching_line.line_content, String::from("Randomly shuffled lines containing ASCII (upper- and lowercase), Cyrillic (upper- and lowercase), Chinese and emoji symbols"));
    assert_eq!(results[10].matching_line.fuzzy_match.score(), 56);
    assert_eq!(
        results[10].matching_line.fuzzy_match.positions(),
        &vec![24, 25, 26, 27, 31, 33, 54, 75, 116]
    );

    assert_eq!(
        results[11].filename.clone().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[11].matching_line.line_number, 1);
    assert_eq!(results[11].matching_line.line_content, String::from("Randomly shuffled lines containing ASCII (upper- and lowercase), Cyrillic (upper- and lowercase), Chinese and emoji symbols"));
    assert_eq!(results[11].matching_line.fuzzy_match.score(), 56);
    assert_eq!(
        results[11].matching_line.fuzzy_match.positions(),
        &vec![24, 25, 26, 27, 31, 33, 54, 75, 116]
    );

    Ok(())
}

#[test]
fn emoji_query() -> Result<(), String> {
    let args = [
        "fzgrep",
        "🐣🦀",
        "resources/tests/test.txt",
        "resources/tests/👨‍🔬.txt",
        "resources/tests/тест.txt",
        "resources/tests/测试.txt",
    ];
    let request = fzgrep::Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "🐣🦀");
    assert_eq!(
        request.targets(),
        &vec![
            String::from("resources/tests/test.txt"),
            String::from("resources/tests/👨‍🔬.txt"),
            String::from("resources/tests/тест.txt"),
            String::from("resources/tests/测试.txt")
        ]
    );

    let results = fzgrep::find_matches(&request).unwrap();
    assert_eq!(results.len(), 4);

    assert_eq!(
        results[0].filename.clone().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[0].matching_line.line_number, 7);
    assert_eq!(
        results[0].matching_line.line_content,
        String::from("🐲🐣🐼🦀🦞🦠")
    );
    assert_eq!(results[0].matching_line.fuzzy_match.score(), 4);
    assert_eq!(
        results[0].matching_line.fuzzy_match.positions(),
        &vec![1, 3]
    );

    assert_eq!(
        results[1].filename.clone().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[1].matching_line.line_number, 6);
    assert_eq!(
        results[1].matching_line.line_content,
        String::from("🐲🐣🐼🦀🦞🦠")
    );
    assert_eq!(results[1].matching_line.fuzzy_match.score(), 4);
    assert_eq!(
        results[1].matching_line.fuzzy_match.positions(),
        &vec![1, 3]
    );

    assert_eq!(
        results[2].filename.clone().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[2].matching_line.line_number, 2);
    assert_eq!(
        results[2].matching_line.line_content,
        String::from("🐲🐣🐼🦀🦞🦠")
    );
    assert_eq!(results[2].matching_line.fuzzy_match.score(), 4);
    assert_eq!(
        results[2].matching_line.fuzzy_match.positions(),
        &vec![1, 3]
    );

    assert_eq!(
        results[3].filename.clone().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[3].matching_line.line_number, 5);
    assert_eq!(
        results[3].matching_line.line_content,
        String::from("🐲🐣🐼🦀🦞🦠")
    );
    assert_eq!(results[3].matching_line.fuzzy_match.score(), 4);
    assert_eq!(
        results[3].matching_line.fuzzy_match.positions(),
        &vec![1, 3]
    );

    Ok(())
}

#[test]
fn cyrillic_query() -> Result<(), String> {
    let args = [
        "fzgrep",
        "тест",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
        "resources/tests/👨‍🔬.txt",
        "resources/tests/测试.txt",
    ];
    let request = fzgrep::Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "тест");
    assert_eq!(
        request.targets(),
        &vec![
            String::from("resources/tests/test.txt"),
            String::from("resources/tests/тест.txt"),
            String::from("resources/tests/👨‍🔬.txt"),
            String::from("resources/tests/测试.txt")
        ]
    );

    let results = fzgrep::find_matches(&request).unwrap();
    assert_eq!(results.len(), 8);

    assert_eq!(
        results[0].filename.clone().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[0].matching_line.line_number, 6);
    assert_eq!(
        results[0].matching_line.line_content,
        String::from("тестування")
    );
    assert_eq!(results[0].matching_line.fuzzy_match.score(), 46);
    assert_eq!(
        results[0].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 2, 3]
    );

    assert_eq!(
        results[1].filename.clone().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[1].matching_line.line_number, 5);
    assert_eq!(
        results[1].matching_line.line_content,
        String::from("тестування")
    );
    assert_eq!(results[1].matching_line.fuzzy_match.score(), 46);
    assert_eq!(
        results[1].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 2, 3]
    );

    assert_eq!(
        results[2].filename.clone().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[2].matching_line.line_number, 5);
    assert_eq!(
        results[2].matching_line.line_content,
        String::from("тестування")
    );
    assert_eq!(results[2].matching_line.fuzzy_match.score(), 46);
    assert_eq!(
        results[2].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 2, 3]
    );

    assert_eq!(
        results[3].filename.clone().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[3].matching_line.line_number, 6);
    assert_eq!(
        results[3].matching_line.line_content,
        String::from("тестування")
    );
    assert_eq!(results[3].matching_line.fuzzy_match.score(), 46);
    assert_eq!(
        results[3].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 2, 3]
    );

    assert_eq!(
        results[4].filename.clone().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[4].matching_line.line_number, 5);
    assert_eq!(results[4].matching_line.line_content, String::from("Текст"));
    assert_eq!(results[4].matching_line.fuzzy_match.score(), 25);
    assert_eq!(
        results[4].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 3, 4]
    );

    assert_eq!(
        results[5].filename.clone().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[5].matching_line.line_number, 3);
    assert_eq!(results[5].matching_line.line_content, String::from("Текст"));
    assert_eq!(results[5].matching_line.fuzzy_match.score(), 25);
    assert_eq!(
        results[5].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 3, 4]
    );

    assert_eq!(
        results[6].filename.clone().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[6].matching_line.line_number, 2);
    assert_eq!(results[6].matching_line.line_content, String::from("Текст"));
    assert_eq!(results[6].matching_line.fuzzy_match.score(), 25);
    assert_eq!(
        results[6].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 3, 4]
    );

    assert_eq!(
        results[7].filename.clone().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[7].matching_line.line_number, 7);
    assert_eq!(results[7].matching_line.line_content, String::from("Текст"));
    assert_eq!(results[7].matching_line.fuzzy_match.score(), 25);
    assert_eq!(
        results[7].matching_line.fuzzy_match.positions(),
        &vec![0, 1, 3, 4]
    );

    Ok(())
}

#[test]
fn chinese_query() -> Result<(), String> {
    let args = [
        "fzgrep",
        "打电",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
        "resources/tests/测试.txt",
        "resources/tests/👨‍🔬.txt",
    ];
    let request = fzgrep::Request::new(args.into_iter().map(String::from))?;
    assert_eq!(request.query(), "打电");
    assert_eq!(
        request.targets(),
        &vec![
            String::from("resources/tests/test.txt"),
            String::from("resources/tests/тест.txt"),
            String::from("resources/tests/测试.txt"),
            String::from("resources/tests/👨‍🔬.txt"),
        ]
    );

    let results = fzgrep::find_matches(&request).unwrap();
    assert_eq!(results.len(), 4);

    assert_eq!(
        results[0].filename.clone().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[0].matching_line.line_number, 2);
    assert_eq!(
        results[0].matching_line.line_content,
        String::from("打电动")
    );
    assert_eq!(results[0].matching_line.fuzzy_match.score(), 17);
    assert_eq!(
        results[0].matching_line.fuzzy_match.positions(),
        &vec![0, 1]
    );

    assert_eq!(
        results[1].filename.clone().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[1].matching_line.line_number, 4);
    assert_eq!(
        results[1].matching_line.line_content,
        String::from("打电动")
    );
    assert_eq!(results[1].matching_line.fuzzy_match.score(), 17);
    assert_eq!(
        results[1].matching_line.fuzzy_match.positions(),
        &vec![0, 1]
    );

    assert_eq!(
        results[2].filename.clone().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[2].matching_line.line_number, 2);
    assert_eq!(
        results[2].matching_line.line_content,
        String::from("打电动")
    );
    assert_eq!(results[2].matching_line.fuzzy_match.score(), 17);
    assert_eq!(
        results[2].matching_line.fuzzy_match.positions(),
        &vec![0, 1]
    );

    assert_eq!(
        results[3].filename.clone().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[3].matching_line.line_number, 4);
    assert_eq!(
        results[3].matching_line.line_content,
        String::from("打电动")
    );
    assert_eq!(results[3].matching_line.fuzzy_match.score(), 17);
    assert_eq!(
        results[3].matching_line.fuzzy_match.positions(),
        &vec![0, 1]
    );

    Ok(())
}
