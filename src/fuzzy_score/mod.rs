#[derive(Debug)]
pub struct FuzzyMatch {
    score: u32,
    matches: Vec<MatchedChar>,
}

impl FuzzyMatch {
    const PATH_SEPARATORS: [char; 2] = ['/', '\\'];
    const REGULAR_SEPARATORS: [char; 7] = ['_', '-', '.', ' ', '\'', '"', ':'];

    pub fn new(query: &str, target: &str) -> FuzzyMatch {
        if target.len() < query.len() {
            return FuzzyMatch {
                score: Score::NONE,
                matches: Vec::new(),
            };
        }

        let mut matches = vec![vec![0; target.len()]; query.len()];
        let mut scores = vec![vec![0; target.len()]; query.len()];

        let mut prev_char = '\0';

        for (query_index, query_char) in query.chars().enumerate() {
            for (target_index, target_char) in target.chars().enumerate() {
                let left_score = if target_index > 0 {
                    scores[query_index][target_index - 1]
                } else {
                    Score::NONE
                };
                let diag_score = if query_index > 0 && target_index > 0 {
                    scores[query_index - 1][target_index - 1]
                } else {
                    Score::NONE
                };
                let sequence_length = if query_index > 0 && target_index > 0 {
                    matches[query_index - 1][target_index - 1]
                } else {
                    Score::NONE
                };

                let context = Context {
                    query_char,
                    target_char,
                    prev_char,
                    word_start: target_index == 0,
                    sequence_length,
                };

                let score = if diag_score == Score::NONE && query_index > 0 {
                    Score::NONE
                } else {
                    FuzzyMatch::compute_char_score(context)
                };

                let higher_score = score != Score::NONE && diag_score + score >= left_score;
                matches[query_index][target_index] =
                    if higher_score { sequence_length + 1 } else { 0 };
                scores[query_index][target_index] = if higher_score {
                    diag_score + score
                } else {
                    left_score
                };

                prev_char = target_char;
            }
        }

        let mut ret = FuzzyMatch {
            score: scores[query.len() - 1][target.len() - 1],
            matches: target
                .chars()
                .map(|item| MatchedChar {
                    character: item,
                    is_match: false,
                })
                .collect(),
        };

        let mut query_index = (query.len() - 1) as i32;
        let mut target_index = (target.len() - 1) as i32;
        while query_index >= 0 && target_index >= 0 {
            if matches[query_index as usize][target_index as usize] == 0 {
                target_index -= 1;
            } else {
                ret.matches[target_index as usize].is_match = true;
                query_index -= 1;
                target_index -= 1;
            }
        }

        ret
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn matches(&self) -> &Vec<MatchedChar> {
        &self.matches
    }

    fn compute_char_score(context: Context) -> u32 {
        if !FuzzyMatch::consider_equal(context.query_char, context.target_char) {
            return Score::NONE;
        }

        let mut score =
            Score::REGULAR + Score::CONSECUTIVE_MATCH * context.sequence_length;

        score += FuzzyMatch::compute_exact_match_bonus(&context);
        score += FuzzyMatch::compute_word_start_bonus(&context);
        score += FuzzyMatch::compute_after_separator_bonus(&context);
        score += FuzzyMatch::compute_camel_case_bonus(&context);

        score
    }

    fn compute_exact_match_bonus(context: &Context) -> u32 {
        if context.query_char == context.target_char {
            Score::EXACT_MATCH
        } else {
            Score::NONE
        }
    }

    fn compute_word_start_bonus(context: &Context) -> u32 {
        if context.word_start {
            Score::WORD_START
        } else {
            Score::NONE
        }
    }

    fn compute_after_separator_bonus(context: &Context) -> u32 {
        if FuzzyMatch::PATH_SEPARATORS
            .iter()
            .any(|&c| c == context.prev_char)
        {
            Score::AFTER_PATH_SEPARATOR
        } else if FuzzyMatch::REGULAR_SEPARATORS
            .iter()
            .any(|&c| c == context.prev_char)
        {
            Score::AFTER_SEPARATOR
        } else {
            Score::NONE
        }
    }

    fn compute_camel_case_bonus(context: &Context) -> u32 {
        if !FuzzyMatch::is_separator(context.prev_char) && context.target_char.is_uppercase() {
            Score::CAMEL_CASE
        } else {
            Score::NONE
        }
    }

    fn consider_equal(query_char: char, target_char: char) -> bool {
        if query_char.to_lowercase().collect::<String>()
            == target_char.to_lowercase().collect::<String>()
        {
            return true;
        }

        if query_char == '/' || query_char == '\\' {
            return target_char == '/' || target_char == '\\';
        }

        false
    }

    fn is_separator(target_char: char) -> bool {
        FuzzyMatch::REGULAR_SEPARATORS
            .iter()
            .any(|&c| c == target_char)
            || FuzzyMatch::PATH_SEPARATORS
                .iter()
                .any(|&c| c == target_char)
    }
}

struct Score;

impl Score {
    const NONE: u32 = 0;
    const REGULAR: u32 = 1;
    const CONSECUTIVE_MATCH: u32 = 5;
    const EXACT_MATCH: u32 = 1;
    const WORD_START: u32 = 8;
    const AFTER_SEPARATOR: u32 = 4;
    const AFTER_PATH_SEPARATOR: u32 = 5;
    const CAMEL_CASE: u32 = 2;
}

struct Context {
    query_char: char,
    target_char: char,
    prev_char: char,
    word_start: bool,
    sequence_length: u32,
}

#[derive(Debug)]
pub struct MatchedChar {
    pub character: char,
    pub is_match: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn some_score() {
        let query = "word";
        let target = "Hello, world!";

        assert_ne!(FuzzyMatch::new(query, target).score(), Score::NONE);
    }

    #[test]
    fn no_score() {
        let query = "butterfly";
        let target = "Hello, world!";

        assert_eq!(FuzzyMatch::new(query, target).score(), Score::NONE);
    }
}
