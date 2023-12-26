use std::path::PathBuf;
use vscode_fuzzy_score_rs::FuzzyMatch;

pub struct MatchingResult {
    pub matching_line: String,
    pub fuzzy_match: FuzzyMatch,
    pub file_name: Option<PathBuf>,
    pub line_number: Option<usize>,
    pub context: Context,
}

pub struct Context {
    pub before: Vec<String>,
    pub after: Vec<String>,
}
