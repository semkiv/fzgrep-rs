mod context;

use context::Context;

use vscode_fuzzy_score_rs::FuzzyMatch;

use crate::match_properties::MatchProperties as CompleteMatchProperties;
use crate::match_properties::location::Location;

// TODO: tests

/// Represents match properties that may or may not have fully accumulated trailing (i.e. "after") context yet.
///
pub enum MatchProperties {
    /// Match properties whose trailing (i.e. "after") context has already been fully accumulated
    /// (with an ready-to-use instance of [`crate::match_properties::MatchProperties`] inside).
    ///
    Ready(CompleteMatchProperties),

    /// Match properties whose trailing (i.e. "after") context has not yet (or at all) been accumulated.
    /// The fields mostly repeat those of [`crate::match_properties::MatchProperties`], except for the context.
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
        context: Context,
    },
}

impl MatchProperties {
    /// Creates a [`MatchProperties`] with the requested properties and "after" context size.
    /// If and only if `after_context_size` is `0`, returns a [`MatchProperties::Ready`] instance
    /// with an empty trailing context.
    /// Otherwise returns a [`MatchProperties::Pending`] instance with an accordingly constructed
    /// instance of [`Context`] as the context.
    ///
    pub fn new(
        matching_line: String,
        fuzzy_match: FuzzyMatch,
        location: Location,
        before_context: Option<Vec<String>>,
        after_context_size: usize,
    ) -> Self {
        let context = Context::new(before_context, after_context_size);
        match context {
            Context::Ready(context) => Self::Ready(CompleteMatchProperties {
                matching_line,
                fuzzy_match,
                location,
                context,
            }),
            Context::Pending { .. } => Self::Pending {
                matching_line,
                fuzzy_match,
                location,
                context,
            },
        }
    }

    /// Updates the current instance of [`MatchProperties`] by accordingly updating
    /// the internal context.
    /// If the context becomes [`Context::Ready`] after this,
    /// the current instance itself becomes [`MatchProperties::Ready`].
    ///
    /// # Panics
    ///
    ///   * Updating an instance of [`MatchProperties::Ready`] is considered a logic error
    ///     and therefore causes a panic.
    ///
    pub fn update(self, line: String) -> Self {
        match self {
            #[expect(clippy::panic, reason = "It is a logic error")]
            Self::Ready(_) => {
                panic!("A instance of 'MatchProperties' updated after completeion")
            }
            Self::Pending {
                matching_line,
                fuzzy_match,
                location,
                context,
            } => {
                let context = context.update(line);
                match context {
                    Context::Ready(context) => Self::Ready(CompleteMatchProperties {
                        matching_line,
                        fuzzy_match,
                        location,
                        context,
                    }),
                    Context::Pending { .. } => Self::Pending {
                        matching_line,
                        fuzzy_match,
                        location,
                        context,
                    },
                }
            }
        }
    }

    /// "Completes" an instance of [`MatchProperties::Pending`] by completing
    /// the internal context and returns the properties with whatever context collected at the time.
    ///
    /// # Panics
    ///
    ///   * Completing an instance of [`MatchProperties::Ready`] is considered a logic error
    ///     and therefore causes a panic.
    ///
    pub fn complete(self) -> CompleteMatchProperties {
        match self {
            #[expect(clippy::panic, reason = "It is a logic error")]
            Self::Ready(_) => {
                panic!("Attempted to complete an already completed instance of 'MatchProperties'")
            }
            Self::Pending {
                matching_line,
                fuzzy_match,
                location,
                context,
            } => CompleteMatchProperties {
                matching_line,
                fuzzy_match,
                location,
                context: context.complete(),
            },
        }
    }
}
