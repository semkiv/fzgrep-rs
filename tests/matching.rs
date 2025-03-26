// It's tests, who cares?
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::too_many_lines)]

use fzgrep::{Targets, cli::args};
use std::path::PathBuf;

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
    let request = args::make_request(cmd.into_iter().map(String::from));
    assert_eq!(request.query, "contigous");
    assert_eq!(
        request.targets,
        Targets::Files(vec![
            PathBuf::from("resources/tests/👨‍🔬.txt"),
            PathBuf::from("resources/tests/name with spaces.txt"),
            PathBuf::from("resources/tests/test.txt"),
            PathBuf::from("resources/tests/тест.txt"),
            PathBuf::from("resources/tests/测试.txt")
        ])
    );

    let results =
        fzgrep::collect_all_matches(&request.query, &request.targets, &request.match_options)
            .unwrap();
    assert_eq!(results.len(), 10);

    assert_eq!(
        results[0].file_name.as_ref().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[0].line_number.unwrap(), 6);
    assert_eq!(results[0].matching_line, String::from("contiguous"));
    assert_eq!(results[0].fuzzy_match.score(), 116);
    assert_eq!(
        results[0].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[1].file_name.as_ref().unwrap(),
        "resources/tests/name with spaces.txt"
    );
    assert_eq!(results[1].line_number.unwrap(), 5);
    assert_eq!(results[1].matching_line, String::from("contiguous"));
    assert_eq!(results[1].fuzzy_match.score(), 116);
    assert_eq!(
        results[1].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[2].file_name.as_ref().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[2].line_number.unwrap(), 2);
    assert_eq!(results[2].matching_line, String::from("contiguous"));
    assert_eq!(results[2].fuzzy_match.score(), 116);
    assert_eq!(
        results[2].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[3].file_name.as_ref().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[3].line_number.unwrap(), 5);
    assert_eq!(results[3].matching_line, String::from("contiguous"));
    assert_eq!(results[3].fuzzy_match.score(), 116);
    assert_eq!(
        results[3].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[4].file_name.as_ref().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[4].line_number.unwrap(), 3);
    assert_eq!(results[4].matching_line, String::from("contiguous"));
    assert_eq!(results[4].fuzzy_match.score(), 116);
    assert_eq!(
        results[4].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[5].file_name.as_ref().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[5].line_number.unwrap(), 2);
    assert_eq!(results[5].matching_line, String::from("Contiguous"));
    assert_eq!(results[5].fuzzy_match.score(), 115);
    assert_eq!(
        results[5].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[6].file_name.as_ref().unwrap(),
        "resources/tests/name with spaces.txt"
    );
    assert_eq!(results[6].line_number.unwrap(), 3);
    assert_eq!(results[6].matching_line, String::from("Contiguous"));
    assert_eq!(results[6].fuzzy_match.score(), 115);
    assert_eq!(
        results[6].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[7].file_name.as_ref().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[7].line_number.unwrap(), 3);
    assert_eq!(results[7].matching_line, String::from("Contiguous"));
    assert_eq!(results[7].fuzzy_match.score(), 115);
    assert_eq!(
        results[7].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[8].file_name.as_ref().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[8].line_number.unwrap(), 6);
    assert_eq!(results[8].matching_line, String::from("Contiguous"));
    assert_eq!(results[8].fuzzy_match.score(), 115);
    assert_eq!(
        results[8].fuzzy_match.positions(),
        &vec![0, 1, 2, 3, 4, 5, 7, 8, 9]
    );

    assert_eq!(
        results[9].file_name.as_ref().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[9].line_number.unwrap(), 2);
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
    let request = args::make_request(cmd.into_iter().map(String::from));
    assert_eq!(request.query, "🐣🦀");
    assert_eq!(
        request.targets,
        Targets::Files(vec![
            PathBuf::from("resources/tests/name with spaces.txt"),
            PathBuf::from("resources/tests/test.txt"),
            PathBuf::from("resources/tests/👨‍🔬.txt"),
            PathBuf::from("resources/tests/тест.txt"),
            PathBuf::from("resources/tests/测试.txt")
        ])
    );

    let results =
        fzgrep::collect_all_matches(&request.query, &request.targets, &request.match_options)
            .unwrap();
    assert_eq!(results.len(), 5);

    assert_eq!(
        results[0].file_name.as_ref().unwrap(),
        "resources/tests/name with spaces.txt"
    );
    assert_eq!(results[0].line_number.unwrap(), 1);
    assert_eq!(results[0].matching_line, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[0].fuzzy_match.score(), 4);
    assert_eq!(results[0].fuzzy_match.positions(), &vec![1, 3]);

    assert_eq!(
        results[1].file_name.as_ref().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[1].line_number.unwrap(), 6);
    assert_eq!(results[1].matching_line, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[1].fuzzy_match.score(), 4);
    assert_eq!(results[1].fuzzy_match.positions(), &vec![1, 3]);

    assert_eq!(
        results[2].file_name.as_ref().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[2].line_number.unwrap(), 5);
    assert_eq!(results[2].matching_line, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[2].fuzzy_match.score(), 4);
    assert_eq!(results[2].fuzzy_match.positions(), &vec![1, 3]);

    assert_eq!(
        results[3].file_name.as_ref().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[3].line_number.unwrap(), 1);
    assert_eq!(results[3].matching_line, String::from("🐲🐣🐼🦀🦞🦠"));
    assert_eq!(results[3].fuzzy_match.score(), 4);
    assert_eq!(results[3].fuzzy_match.positions(), &vec![1, 3]);

    assert_eq!(
        results[4].file_name.as_ref().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[4].line_number.unwrap(), 4);
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
    let request = args::make_request(cmd.into_iter().map(String::from));
    assert_eq!(request.query, "тест");
    assert_eq!(
        request.targets,
        Targets::Files(vec![
            PathBuf::from("resources/tests/name with spaces.txt"),
            PathBuf::from("resources/tests/test.txt"),
            PathBuf::from("resources/tests/тест.txt"),
            PathBuf::from("resources/tests/👨‍🔬.txt"),
            PathBuf::from("resources/tests/测试.txt")
        ])
    );

    let results =
        fzgrep::collect_all_matches(&request.query, &request.targets, &request.match_options)
            .unwrap();
    assert_eq!(results.len(), 10);

    assert_eq!(
        results[0].file_name.as_ref().unwrap(),
        "resources/tests/name with spaces.txt"
    );
    assert_eq!(results[0].line_number.unwrap(), 2);
    assert_eq!(results[0].matching_line, String::from("тестування"));
    assert_eq!(results[0].fuzzy_match.score(), 46);
    assert_eq!(results[0].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(
        results[1].file_name.as_ref().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[1].line_number.unwrap(), 5);
    assert_eq!(results[1].matching_line, String::from("тестування"));
    assert_eq!(results[1].fuzzy_match.score(), 46);
    assert_eq!(results[1].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(
        results[2].file_name.as_ref().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[2].line_number.unwrap(), 4);
    assert_eq!(results[2].matching_line, String::from("тестування"));
    assert_eq!(results[2].fuzzy_match.score(), 46);
    assert_eq!(results[2].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(
        results[3].file_name.as_ref().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[3].line_number.unwrap(), 4);
    assert_eq!(results[3].matching_line, String::from("тестування"));
    assert_eq!(results[3].fuzzy_match.score(), 46);
    assert_eq!(results[3].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(
        results[4].file_name.as_ref().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[4].line_number.unwrap(), 5);
    assert_eq!(results[4].matching_line, String::from("тестування"));
    assert_eq!(results[4].fuzzy_match.score(), 46);
    assert_eq!(results[4].fuzzy_match.positions(), &vec![0, 1, 2, 3]);

    assert_eq!(
        results[5].file_name.as_ref().unwrap(),
        "resources/tests/name with spaces.txt"
    );
    assert_eq!(results[5].line_number.unwrap(), 4);
    assert_eq!(results[5].matching_line, String::from("Текст"));
    assert_eq!(results[5].fuzzy_match.score(), 25);
    assert_eq!(results[5].fuzzy_match.positions(), &vec![0, 1, 3, 4]);

    assert_eq!(
        results[6].file_name.as_ref().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[6].line_number.unwrap(), 4);
    assert_eq!(results[6].matching_line, String::from("Текст"));
    assert_eq!(results[6].fuzzy_match.score(), 25);
    assert_eq!(results[6].fuzzy_match.positions(), &vec![0, 1, 3, 4]);

    assert_eq!(
        results[7].file_name.as_ref().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[7].line_number.unwrap(), 2);
    assert_eq!(results[7].matching_line, String::from("Текст"));
    assert_eq!(results[7].fuzzy_match.score(), 25);
    assert_eq!(results[7].fuzzy_match.positions(), &vec![0, 1, 3, 4]);

    assert_eq!(
        results[8].file_name.as_ref().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[8].line_number.unwrap(), 1);
    assert_eq!(results[8].matching_line, String::from("Текст"));
    assert_eq!(results[8].fuzzy_match.score(), 25);
    assert_eq!(results[8].fuzzy_match.positions(), &vec![0, 1, 3, 4]);

    assert_eq!(
        results[9].file_name.as_ref().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[9].line_number.unwrap(), 6);
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
    let request = args::make_request(cmd.into_iter().map(String::from));
    assert_eq!(request.query, "打电");
    assert_eq!(
        request.targets,
        Targets::Files(vec![
            PathBuf::from("resources/tests/name with spaces.txt"),
            PathBuf::from("resources/tests/test.txt"),
            PathBuf::from("resources/tests/тест.txt"),
            PathBuf::from("resources/tests/测试.txt"),
            PathBuf::from("resources/tests/👨‍🔬.txt"),
        ])
    );

    let results =
        fzgrep::collect_all_matches(&request.query, &request.targets, &request.match_options)
            .unwrap();
    assert_eq!(results.len(), 5);

    assert_eq!(
        results[0].file_name.as_ref().unwrap(),
        "resources/tests/name with spaces.txt"
    );
    assert_eq!(results[0].line_number.unwrap(), 6);
    assert_eq!(results[0].matching_line, String::from("打电动"));
    assert_eq!(results[0].fuzzy_match.score(), 17);
    assert_eq!(results[0].fuzzy_match.positions(), &vec![0, 1]);

    assert_eq!(
        results[1].file_name.as_ref().unwrap(),
        "resources/tests/test.txt"
    );
    assert_eq!(results[1].line_number.unwrap(), 1);
    assert_eq!(results[1].matching_line, String::from("打电动"));
    assert_eq!(results[1].fuzzy_match.score(), 17);
    assert_eq!(results[1].fuzzy_match.positions(), &vec![0, 1]);

    assert_eq!(
        results[2].file_name.as_ref().unwrap(),
        "resources/tests/тест.txt"
    );
    assert_eq!(results[2].line_number.unwrap(), 3);
    assert_eq!(results[2].matching_line, String::from("打电动"));
    assert_eq!(results[2].fuzzy_match.score(), 17);
    assert_eq!(results[2].fuzzy_match.positions(), &vec![0, 1]);

    assert_eq!(
        results[3].file_name.as_ref().unwrap(),
        "resources/tests/测试.txt"
    );
    assert_eq!(results[3].line_number.unwrap(), 1);
    assert_eq!(results[3].matching_line, String::from("打电动"));
    assert_eq!(results[3].fuzzy_match.score(), 17);
    assert_eq!(results[3].fuzzy_match.positions(), &vec![0, 1]);

    assert_eq!(
        results[4].file_name.as_ref().unwrap(),
        "resources/tests/👨‍🔬.txt"
    );
    assert_eq!(results[4].line_number.unwrap(), 3);
    assert_eq!(results[4].matching_line, String::from("打电动"));
    assert_eq!(results[4].fuzzy_match.score(), 17);
    assert_eq!(results[4].fuzzy_match.positions(), &vec![0, 1]);
}
