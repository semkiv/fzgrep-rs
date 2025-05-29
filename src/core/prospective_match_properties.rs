mod prospective_context;

use prospective_context::ProspectiveContext;

use vscode_fuzzy_score_rs::FuzzyMatch;

use crate::match_properties::MatchProperties;
use crate::match_properties::location::Location;

// TODO: tests

/// Represents match properties that may or may not have fully accumulated trailing (i.e. "after") context yet.
///
pub enum ProspectiveMatchProperties {
    /// Match properties whose trailing (i.e. "after") context has already been fully accumulated
    /// (with an ready-to-use instance of [`MatchProperties`] inside).
    ///
    Ready(MatchProperties),

    /// Match properties whose trailing (i.e. "after") context has not yet (or at all) been accumulated.
    /// The fields mostly repeat those of [`MatchProperties`], except for the context.
    ///
    Pending {
        /// The line that contains the match.
        ///
        matching_line: String,

        /// The properties of the match.
        ///
        fuzzy_match: FuzzyMatch,

        /// The location of the match.
        ///
        location: Location,

        /// The context surrounding the match, the trailing (i.e. "after") part of which
        /// may or may not be complete at the moment.
        context: ProspectiveContext,
    },
}

impl ProspectiveMatchProperties {
    /// Creates a [`ProspectiveMatchProperties`] with the requested properties and "after" context size.
    /// If and only if `after_context_size` is `0`, returns a [`ProspectiveMatchProperties::Ready`] instance
    /// with an empty trailing context.
    /// Otherwise returns a [`ProspectiveMatchProperties::Pending`] instance with an accordingly constructed
    /// instance of [`ProspectiveContext`] as the context.
    ///
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

    /// Updates the current instance of [`ProspectiveMatchProperties`] by accordingly updating
    /// the internal context.
    /// If the context becomes [`ProspectiveContext::Ready`] after this,
    /// the current instance itself becomes [`ProspectiveMatchProperties::Ready`].
    ///
    /// # Panics
    ///
    ///   * Updating an instance of [`ProspectiveMatchProperties::Ready`] is considered a logic error
    ///     and therefore causes a panic.
    ///
    pub fn update(self, line: String) -> Self {
        match self {
            #[expect(clippy::panic, reason = "It is a logic error")]
            Self::Ready(_) => {
                panic!("A instance of 'ProspectiveMatchProperties' updated after completeion")
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

    /// "Completes" an instance of [`ProspectiveMatchProperties::Pending`] by completing
    /// the internal context and returns the properties with whatever context collected at the time.
    ///
    /// # Panics
    ///
    ///   * Completing an instance of [`ProspectiveMatchProperties::Ready`] is considered a logic error
    ///     and therefore causes a panic.
    ///
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
