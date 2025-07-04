#![cfg(any(target_os = "linux", target_os = "macos"))]

#[cfg(test)]
mod integration_tests {
    use fzgrep::cli;
    use fzgrep::request::targets::Targets;
    use fzgrep::request::targets::filter::Filter;

    use glob::Pattern;
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

        let request = cli::make_request(cmd.into_iter().map(String::from));

        assert_eq!(request.query, "recursive");
        assert_eq!(
            request.targets,
            Targets::RecursiveEntries {
                paths: vec![PathBuf::from("resources/tests/")],
                filter: None
            }
        );

        let results = fzgrep::collect_matches(request.into()).unwrap();
        let mut results = results
            .into_iter()
            .map(|x| x.location.source_name.unwrap())
            .collect::<Vec<_>>();
        results.sort();

        assert_eq!(
            results,
            [
                "resources/tests/ignore.json",
                "resources/tests/nested/ignore.json",
                "resources/tests/nested/ignore/ignore.json",
                "resources/tests/nested/ignore/test.json",
                "resources/tests/nested/more_nested/ignore.json",
                "resources/tests/nested/more_nested/test.json",
                "resources/tests/nested/more_nested/test.txt",
                "resources/tests/nested/test.json",
                "resources/tests/nested/test.txt",
                "resources/tests/nested/test2.txt",
                "resources/tests/test.json",
                "resources/tests/test.txt",
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

        let request = cli::make_request(cmd.into_iter().map(String::from));

        assert_eq!(request.query, "recursive");
        assert_eq!(
            request.targets,
            Targets::RecursiveEntries {
                paths: vec![PathBuf::from("resources/tests/")],
                filter: None
            }
        );

        let results = fzgrep::collect_matches(request.into()).unwrap();
        let mut results = results
            .into_iter()
            .map(|x| x.location.source_name.unwrap())
            .collect::<Vec<_>>();
        results.sort();

        assert_eq!(
            results,
            [
                "resources/tests/ignore.json",
                "resources/tests/nested/ignore.json",
                "resources/tests/nested/ignore/ignore.json",
                "resources/tests/nested/ignore/test.json",
                "resources/tests/nested/more_nested/ignore.json",
                "resources/tests/nested/more_nested/test.json",
                "resources/tests/nested/more_nested/test.txt",
                "resources/tests/nested/test.json",
                "resources/tests/nested/test.txt",
                "resources/tests/nested/test2.txt",
                "resources/tests/test.json",
                "resources/tests/test.txt",
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

        let request = cli::make_request(cmd.into_iter().map(String::from));

        assert_eq!(request.query, "recursive");
        assert_eq!(
            request.targets,
            Targets::RecursiveEntries {
                paths: vec![
                    PathBuf::from("resources/tests/nested/test.txt"),
                    PathBuf::from("resources/tests/nested/test2.txt")
                ],
                filter: None
            }
        );

        let results = fzgrep::collect_matches(request.into()).unwrap();
        let mut results = results
            .into_iter()
            .map(|x| x.location.source_name.unwrap())
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

        let request = cli::make_request(cmd.into_iter().map(String::from));

        assert_eq!(request.query, "recursive");
        assert_eq!(
            request.targets,
            Targets::RecursiveEntries {
                paths: vec![
                    PathBuf::from("resources/tests/nested/more_nested/"),
                    PathBuf::from("resources/tests/nested/test.txt"),
                    PathBuf::from("resources/tests/nested/test2.txt")
                ],
                filter: None
            }
        );

        let results = fzgrep::collect_matches(request.into()).unwrap();
        let mut results = results
            .into_iter()
            .map(|x| x.location.source_name.unwrap())
            .collect::<Vec<_>>();
        results.sort();

        assert_eq!(
            results,
            [
                "resources/tests/nested/more_nested/ignore.json",
                "resources/tests/nested/more_nested/test.json",
                "resources/tests/nested/more_nested/test.txt",
                "resources/tests/nested/test.txt",
                "resources/tests/nested/test2.txt",
            ]
        );
    }

    #[test]
    fn recursive_with_include_filters() {
        let cmd = [
            "fzgrep",
            "--with-filename",
            "--recursive",
            "--include",
            "**/more_nested/*.txt",
            "--include",
            "**/tests/*.txt",
            "recursive",
            "resources/tests/nested",
            "resources/tests/ignore.json",
            "resources/tests/test.json",
            "resources/tests/test.txt",
        ];
        let request = cli::make_request(cmd.into_iter().map(String::from));

        assert_eq!(request.query, "recursive");
        assert_eq!(
            request.targets,
            Targets::RecursiveEntries {
                paths: vec![
                    PathBuf::from("resources/tests/nested/"),
                    PathBuf::from("resources/tests/ignore.json"),
                    PathBuf::from("resources/tests/test.json"),
                    PathBuf::from("resources/tests/test.txt"),
                ],
                filter: Some(Filter::with_include(vec![
                    Pattern::new("**/more_nested/*.txt").unwrap(),
                    Pattern::new("**/tests/*.txt").unwrap(),
                ]))
            }
        );

        let results = fzgrep::collect_matches(request.into()).unwrap();
        let mut results = results
            .into_iter()
            .map(|x| x.location.source_name.unwrap())
            .collect::<Vec<_>>();
        results.sort();

        assert_eq!(
            results,
            [
                "resources/tests/nested/more_nested/test.txt",
                "resources/tests/test.txt",
            ]
        );
    }

    #[test]
    fn recursive_with_exclude_filters() {
        let cmd = [
            "fzgrep",
            "--with-filename",
            "--recursive",
            "--exclude",
            "**/ignore/**",
            "--exclude",
            "**/ignore.json",
            "--exclude",
            "**/*.txt",
            "recursive",
            "resources/tests/nested",
            "resources/tests/ignore.json",
            "resources/tests/test.json",
            "resources/tests/test.txt",
        ];

        let request = cli::make_request(cmd.into_iter().map(String::from));

        assert_eq!(request.query, "recursive");
        assert_eq!(
            request.targets,
            Targets::RecursiveEntries {
                paths: vec![
                    PathBuf::from("resources/tests/nested/"),
                    PathBuf::from("resources/tests/ignore.json"),
                    PathBuf::from("resources/tests/test.json"),
                    PathBuf::from("resources/tests/test.txt"),
                ],
                filter: Some(Filter::with_exclude(vec![
                    Pattern::new("**/ignore/**").unwrap(),
                    Pattern::new("**/ignore.json").unwrap(),
                    Pattern::new("**/*.txt").unwrap(),
                ]))
            }
        );

        let results = fzgrep::collect_matches(request.into()).unwrap();
        let mut results = results
            .into_iter()
            .map(|x| x.location.source_name.unwrap())
            .collect::<Vec<_>>();
        results.sort();

        assert_eq!(
            results,
            [
                "resources/tests/nested/more_nested/test.json",
                "resources/tests/nested/test.json",
                "resources/tests/test.json",
            ]
        );
    }

    #[test]
    fn recursive_with_include_and_exclude_filters() {
        let cmd = [
            "fzgrep",
            "--with-filename",
            "--recursive",
            "--include",
            "**/tests/nested/**/*.json",
            "--include",
            "**/tests/nested/**/*.txt",
            "--exclude",
            "**/ignore/**",
            "--exclude",
            "**/ignore.json",
            "recursive",
            "resources/tests/nested",
            "resources/tests/ignore.json",
            "resources/tests/test.json",
            "resources/tests/test.txt",
        ];

        let request = cli::make_request(cmd.into_iter().map(String::from));

        assert_eq!(request.query, "recursive");
        assert_eq!(
            request.targets,
            Targets::RecursiveEntries {
                paths: vec![
                    PathBuf::from("resources/tests/nested/"),
                    PathBuf::from("resources/tests/ignore.json"),
                    PathBuf::from("resources/tests/test.json"),
                    PathBuf::from("resources/tests/test.txt"),
                ],
                filter: Some(Filter::new(
                    vec![
                        Pattern::new("**/tests/nested/**/*.json").unwrap(),
                        Pattern::new("**/tests/nested/**/*.txt").unwrap(),
                    ],
                    vec![
                        Pattern::new("**/ignore/**").unwrap(),
                        Pattern::new("**/ignore.json").unwrap(),
                    ]
                )),
            }
        );

        let results = fzgrep::collect_matches(request.into()).unwrap();
        let mut results = results
            .into_iter()
            .map(|x| x.location.source_name.unwrap())
            .collect::<Vec<_>>();
        results.sort();

        assert_eq!(
            results,
            [
                "resources/tests/nested/more_nested/test.json",
                "resources/tests/nested/more_nested/test.txt",
                "resources/tests/nested/test.json",
                "resources/tests/nested/test.txt",
                "resources/tests/nested/test2.txt",
            ]
        );
    }
}
