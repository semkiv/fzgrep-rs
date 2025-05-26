mod prospective_context;

use prospective_context::ProspectiveContext;

use crate::match_properties::{MatchProperties, location::Location};

use vscode_fuzzy_score_rs::FuzzyMatch;

// TODO: doc, tests

pub enum ProspectiveMatchProperties {
    Ready(MatchProperties),
    Pending {
        matching_line: String,
        fuzzy_match: FuzzyMatch,
        location: Location,
        context: ProspectiveContext,
    },
}

impl ProspectiveMatchProperties {
    pub fn new(
        matching_line: String,
        fuzzy_match: FuzzyMatch,
        location: Location,
        before_context: Option<Vec<String>>,
        after_context_size: usize,
    ) -> Self {
        let context = ProspectiveContext::new(before_context, after_context_size);
        match context {
            ProspectiveContext::Ready(context) => Self::Ready(MatchProperties {
                matching_line,
                fuzzy_match,
                location,
                context,
            }),
            ProspectiveContext::Pending { .. } => Self::Pending {
                matching_line,
                fuzzy_match,
                location,
                context,
            },
        }
    }

    pub fn update(self, line: String) -> Self {
        match self {
            Self::Ready(_) => {
                unreachable!("An already complete ProspectiveMatchProperties should not be updated")
            }
            Self::Pending {
                matching_line,
                fuzzy_match,
                location,
                context,
            } => {
                let context = context.update(line);
                match context {
                    ProspectiveContext::Ready(context) => Self::Ready(MatchProperties {
                        matching_line,
                        fuzzy_match,
                        location,
                        context,
                    }),
                    ProspectiveContext::Pending { .. } => Self::Pending {
                        matching_line,
                        fuzzy_match,
                        location,
                        context,
                    },
                }
            }
        }
    }

    pub fn complete(self) -> MatchProperties {
        match self {
            Self::Ready(props) => props,
            Self::Pending {
                matching_line,
                fuzzy_match,
                location,
                context,
            } => MatchProperties {
                matching_line,
                fuzzy_match,
                location,
                context: context.complete(),
            },
        }
    }
}
