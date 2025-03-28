#![expect(clippy::cognitive_complexity, reason = "It's tests, who cares?")]
#![expect(clippy::indexing_slicing, reason = "It's tests, who cares?")]

use fzgrep::{MatchCollectionStrategy, cli::args};

#[test]
fn top_five() {
    let cmd = [
        "fzgrep",
        "--with-filename",
        "--line-number",
        "--top",
        "5",
        "--recursive",
        "test",
        "resources/tests/top_matches/",
    ];
    let request = args::make_request(cmd.into_iter().map(String::from));
    assert_eq!(request.strategy, MatchCollectionStrategy::CollectTop(5));

    let results =
        fzgrep::collect_top_matches(&request.query, &request.targets, &request.match_options, 5)
            .unwrap();
    assert_eq!(results.len(), 5);

    assert_eq!(
        results[0].file_name.as_ref().unwrap(),
        "resources/tests/top_matches/1.txt"
    );
    assert_eq!(results[0].line_number.unwrap(), 1);
    assert_eq!(results[0].matching_line, String::from("test task"));
    assert_eq!(results[0].fuzzy_match.score(), 46);
    assert_eq!(results[0].fuzzy_match.positions(), &vec![0, 1, 2, 3,]);

    assert_eq!(
        results[1].file_name.as_ref().unwrap(),
        "resources/tests/top_matches/1.txt"
    );
    assert_eq!(results[1].line_number.unwrap(), 5);
    assert_eq!(results[1].matching_line, String::from("tests"));
    assert_eq!(results[1].fuzzy_match.score(), 46);
    assert_eq!(results[1].fuzzy_match.positions(), &vec![0, 1, 2, 3,]);

    assert_eq!(
        results[2].file_name.as_ref().unwrap(),
        "resources/tests/top_matches/2.txt"
    );
    assert_eq!(results[2].line_number.unwrap(), 4);
    assert_eq!(results[2].matching_line, String::from("test"));
    assert_eq!(results[2].fuzzy_match.score(), 46);
    assert_eq!(results[2].fuzzy_match.positions(), &vec![0, 1, 2, 3,]);

    assert_eq!(
        results[3].file_name.as_ref().unwrap(),
        "resources/tests/top_matches/1.txt"
    );
    assert_eq!(results[3].line_number.unwrap(), 3);
    assert_eq!(results[3].matching_line, String::from("Test"));
    assert_eq!(results[3].fuzzy_match.score(), 45);
    assert_eq!(results[3].fuzzy_match.positions(), &vec![0, 1, 2, 3,]);

    assert_eq!(
        results[4].file_name.as_ref().unwrap(),
        "resources/tests/top_matches/2.txt"
    );
    assert_eq!(results[4].line_number.unwrap(), 5);
    assert_eq!(results[4].matching_line, String::from("Test task"));
    assert_eq!(results[4].fuzzy_match.score(), 45);
    assert_eq!(results[4].fuzzy_match.positions(), &vec![0, 1, 2, 3,]);
}

#[test]
fn stability() {
    let top = {
        let cmd = [
            "fzgrep",
            "--with-filename",
            "--line-number",
            "--top",
            "5",
            "--recursive",
            "test",
            "resources/tests/top_matches/",
        ];
        let request = args::make_request(cmd.into_iter().map(String::from));
        assert_eq!(request.strategy, MatchCollectionStrategy::CollectTop(5));

        fzgrep::collect_top_matches(&request.query, &request.targets, &request.match_options, 5)
            .unwrap()
    };

    let all = {
        let cmd = [
            "fzgrep",
            "--with-filename",
            "--line-number",
            "--recursive",
            "test",
            "resources/tests/top_matches/",
        ];
        let request = args::make_request(cmd.into_iter().map(String::from));
        assert_eq!(request.strategy, MatchCollectionStrategy::CollectAll);

        fzgrep::collect_top_matches(&request.query, &request.targets, &request.match_options, 5)
            .unwrap()
    };

    assert_eq!(top, all.into_iter().take(5).collect::<Vec<_>>());
}
