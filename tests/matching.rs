#![expect(clippy::cognitive_complexity, reason = "It's tests, who cares?")]
#![expect(clippy::indexing_slicing, reason = "It's tests, who cares?")]
#![expect(clippy::non_ascii_literal, reason = "It's tests, who cares?")]
#![expect(
    clippy::tests_outside_test_module,
    reason = "These are integration tests"
)]
#![expect(clippy::too_many_lines, reason = "It's tests, who cares?")]

use fzgrep::cli;

#[test]
fn ascii_query() {
    let cmd = [
        "fzgrep",
        "--with-filename",
        "--line-number",
        "contigous",
        "resources/tests/👨‍🔬.txt",
        "resources/tests/name with spaces.txt",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
        "resources/tests/测试.txt",
    ];

    let request = cli::make_request(cmd.into_iter().map(String::from));
    let results = fzgrep::collect_matches(request.into()).unwrap();

    assert_eq!(results.len(), 10);

    assert_eq!(
        results[0].location.source_name.as_ref().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[0].location.line_number.unwrap(), 6);
    assert_eq!(results[0].matching_line, String::from("contiguous"));
    assert_eq!(results[0].fuzzy_match.score(), 116);
    assert_eq!(
        results[0].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[1].location.source_name.as_ref().unwrap(),
        "resources/tests/name with spaces.txt"
    );
    assert_eq!(results[1].location.line_number.unwrap(), 5);
    assert_eq!(results[1].matching_line, String::from("contiguous"));
    assert_eq!(results[1].fuzzy_match.score(), 116);
    assert_eq!(
        results[1].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[2].location.source_name.as_ref().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[2].location.line_number.unwrap(), 2);
    assert_eq!(results[2].matching_line, String::from("contiguous"));
    assert_eq!(results[2].fuzzy_match.score(), 116);
    assert_eq!(
        results[2].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[3].location.source_name.as_ref().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[3].location.line_number.unwrap(), 5);
    assert_eq!(results[3].matching_line, String::from("contiguous"));
    assert_eq!(results[3].fuzzy_match.score(), 116);
    assert_eq!(
        results[3].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[4].location.source_name.as_ref().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[4].location.line_number.unwrap(), 3);
    assert_eq!(results[4].matching_line, String::from("contiguous"));
    assert_eq!(results[4].fuzzy_match.score(), 116);
    assert_eq!(
        results[4].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[5].location.source_name.as_ref().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[5].location.line_number.unwrap(), 2);
    assert_eq!(results[5].matching_line, String::from("Contiguous"));
    assert_eq!(results[5].fuzzy_match.score(), 115);
    assert_eq!(
        results[5].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[6].location.source_name.as_ref().unwrap(),
        "resources/tests/name with spaces.txt"
    );
    assert_eq!(results[6].location.line_number.unwrap(), 3);
    assert_eq!(results[6].matching_line, String::from("Contiguous"));
    assert_eq!(results[6].fuzzy_match.score(), 115);
    assert_eq!(
        results[6].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[7].location.source_name.as_ref().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[7].location.line_number.unwrap(), 3);
    assert_eq!(results[7].matching_line, String::from("Contiguous"));
    assert_eq!(results[7].fuzzy_match.score(), 115);
    assert_eq!(
        results[7].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[8].location.source_name.as_ref().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[8].location.line_number.unwrap(), 6);
    assert_eq!(results[8].matching_line, String::from("Contiguous"));
    assert_eq!(results[8].fuzzy_match.score(), 115);
    assert_eq!(
        results[8].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[9].location.source_name.as_ref().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[9].location.line_number.unwrap(), 2);
    assert_eq!(results[9].matching_line, String::from("Contiguous"));
    assert_eq!(results[9].fuzzy_match.score(), 115);
    assert_eq!(
        results[9].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );
}

#[test]
fn emoji_query() {
    let cmd = [
        "fzgrep",
        "--with-filename",
        "--line-number",
        "🐣🦀",
        "resources/tests/name with spaces.txt",
        "resources/tests/test.txt",
        "resources/tests/👨‍🔬.txt",
        "resources/tests/тест.txt",
        "resources/tests/测试.txt",
    ];

    let request = cli::make_request(cmd.into_iter().map(String::from));
    let results = fzgrep::collect_matches(request.into()).unwrap();

    assert_eq!(results.len(), 5);

    assert_eq!(
        results[0].location.source_name.as_ref().unwrap(),
        "resources/tests/name with spaces.txt"
    );
    assert_eq!(results[0].location.line_number.unwrap(), 1);
    assert_eq!(results[0].matching_line, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[0].fuzzy_match.score(), 4);
    assert_eq!(results[0].fuzzy_match.positions(), &vec![1, 3]);

    assert_eq!(
        results[1].location.source_name.as_ref().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[1].location.line_number.unwrap(), 6);
    assert_eq!(results[1].matching_line, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[1].fuzzy_match.score(), 4);
    assert_eq!(results[1].fuzzy_match.positions(), &vec![1, 3]);

    assert_eq!(
        results[2].location.source_name.as_ref().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[2].location.line_number.unwrap(), 5);
    assert_eq!(results[2].matching_line, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[2].fuzzy_match.score(), 4);
    assert_eq!(results[2].fuzzy_match.positions(), &vec![1, 3]);

    assert_eq!(
        results[3].location.source_name.as_ref().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[3].location.line_number.unwrap(), 1);
    assert_eq!(results[3].matching_line, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[3].fuzzy_match.score(), 4);
    assert_eq!(results[3].fuzzy_match.positions(), &vec![1, 3]);

    assert_eq!(
        results[4].location.source_name.as_ref().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[4].location.line_number.unwrap(), 4);
    assert_eq!(results[4].matching_line, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[4].fuzzy_match.score(), 4);
    assert_eq!(results[4].fuzzy_match.positions(), &vec![1, 3]);
}

#[test]
fn cyrillic_query() {
    let cmd = [
        "fzgrep",
        "--with-filename",
        "--line-number",
        "тест",
        "resources/tests/name with spaces.txt",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
        "resources/tests/👨‍🔬.txt",
        "resources/tests/测试.txt",
    ];

    let request = cli::make_request(cmd.into_iter().map(String::from));
    let results = fzgrep::collect_matches(request.into()).unwrap();

    assert_eq!(results.len(), 10);

    assert_eq!(
        results[0].location.source_name.as_ref().unwrap(),
        "resources/tests/name with spaces.txt"
    );
    assert_eq!(results[0].location.line_number.unwrap(), 2);
    assert_eq!(results[0].matching_line, String::from("тестування"));
    assert_eq!(results[0].fuzzy_match.score(), 46);
    assert_eq!(results[0].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(
        results[1].location.source_name.as_ref().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[1].location.line_number.unwrap(), 5);
    assert_eq!(results[1].matching_line, String::from("тестування"));
    assert_eq!(results[1].fuzzy_match.score(), 46);
    assert_eq!(results[1].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(
        results[2].location.source_name.as_ref().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[2].location.line_number.unwrap(), 4);
    assert_eq!(results[2].matching_line, String::from("тестування"));
    assert_eq!(results[2].fuzzy_match.score(), 46);
    assert_eq!(results[2].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(
        results[3].location.source_name.as_ref().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[3].location.line_number.unwrap(), 4);
    assert_eq!(results[3].matching_line, String::from("тестування"));
    assert_eq!(results[3].fuzzy_match.score(), 46);
    assert_eq!(results[3].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(
        results[4].location.source_name.as_ref().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[4].location.line_number.unwrap(), 5);
    assert_eq!(results[4].matching_line, String::from("тестування"));
    assert_eq!(results[4].fuzzy_match.score(), 46);
    assert_eq!(results[4].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(
        results[5].location.source_name.as_ref().unwrap(),
        "resources/tests/name with spaces.txt"
    );
    assert_eq!(results[5].location.line_number.unwrap(), 4);
    assert_eq!(results[5].matching_line, String::from("Текст"));
    assert_eq!(results[5].fuzzy_match.score(), 25);
    assert_eq!(results[5].fuzzy_match.positions(), &vec![0, 1, 3, 4]);

    assert_eq!(
        results[6].location.source_name.as_ref().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[6].location.line_number.unwrap(), 4);
    assert_eq!(results[6].matching_line, String::from("Текст"));
    assert_eq!(results[6].fuzzy_match.score(), 25);
    assert_eq!(results[6].fuzzy_match.positions(), &vec![0, 1, 3, 4]);

    assert_eq!(
        results[7].location.source_name.as_ref().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[7].location.line_number.unwrap(), 2);
    assert_eq!(results[7].matching_line, String::from("Текст"));
    assert_eq!(results[7].fuzzy_match.score(), 25);
    assert_eq!(results[7].fuzzy_match.positions(), &vec![0, 1, 3, 4]);

    assert_eq!(
        results[8].location.source_name.as_ref().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[8].location.line_number.unwrap(), 1);
    assert_eq!(results[8].matching_line, String::from("Текст"));
    assert_eq!(results[8].fuzzy_match.score(), 25);
    assert_eq!(results[8].fuzzy_match.positions(), &vec![0, 1, 3, 4]);

    assert_eq!(
        results[9].location.source_name.as_ref().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[9].location.line_number.unwrap(), 6);
    assert_eq!(results[9].matching_line, String::from("Текст"));
    assert_eq!(results[9].fuzzy_match.score(), 25);
    assert_eq!(results[9].fuzzy_match.positions(), &vec![0, 1, 3, 4]);
}

#[test]
fn chinese_query() {
    let cmd = [
        "fzgrep",
        "--with-filename",
        "--line-number",
        "打电",
        "resources/tests/name with spaces.txt",
        "resources/tests/test.txt",
        "resources/tests/тест.txt",
        "resources/tests/测试.txt",
        "resources/tests/👨‍🔬.txt",
    ];

    let request = cli::make_request(cmd.into_iter().map(String::from));
    let results = fzgrep::collect_matches(request.into()).unwrap();

    assert_eq!(results.len(), 5);

    assert_eq!(
        results[0].location.source_name.as_ref().unwrap(),
        "resources/tests/name with spaces.txt"
    );
    assert_eq!(results[0].location.line_number.unwrap(), 6);
    assert_eq!(results[0].matching_line, String::from("打电动"));
    assert_eq!(results[0].fuzzy_match.score(), 17);
    assert_eq!(results[0].fuzzy_match.positions(), &vec![0, 1]);

    assert_eq!(
        results[1].location.source_name.as_ref().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[1].location.line_number.unwrap(), 1);
    assert_eq!(results[1].matching_line, String::from("打电动"));
    assert_eq!(results[1].fuzzy_match.score(), 17);
    assert_eq!(results[1].fuzzy_match.positions(), &vec![0, 1]);

    assert_eq!(
        results[2].location.source_name.as_ref().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[2].location.line_number.unwrap(), 3);
    assert_eq!(results[2].matching_line, String::from("打电动"));
    assert_eq!(results[2].fuzzy_match.score(), 17);
    assert_eq!(results[2].fuzzy_match.positions(), &vec![0, 1]);

    assert_eq!(
        results[3].location.source_name.as_ref().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[3].location.line_number.unwrap(), 1);
    assert_eq!(results[3].matching_line, String::from("打电动"));
    assert_eq!(results[3].fuzzy_match.score(), 17);
    assert_eq!(results[3].fuzzy_match.positions(), &vec![0, 1]);

    assert_eq!(
        results[4].location.source_name.as_ref().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[4].location.line_number.unwrap(), 3);
    assert_eq!(results[4].matching_line, String::from("打电动"));
    assert_eq!(results[4].fuzzy_match.score(), 17);
    assert_eq!(results[4].fuzzy_match.positions(), &vec![0, 1]);
}
