mod prospective_after_context;

use prospective_after_context::ProspectiveAfterContext;

use crate::match_properties::context::Context;

// TODO: docs, tests
pub enum ProspectiveContext {
    Ready(Context),
    Pending {
        before_context: Option<Vec<String>>,
        after_context: ProspectiveAfterContext,
    },
}

impl ProspectiveContext {
    pub fn new(before_context: Option<Vec<String>>, after_context_size: usize) -> Self {
        let after_context = ProspectiveAfterContext::new(after_context_size);
        match after_context {
            ProspectiveAfterContext::Ready(ctx) => Self::Ready(Context {
                before: before_context,
                after: ctx,
            }),
            ProspectiveAfterContext::Pending { .. } => Self::Pending {
                before_context,
                after_context,
            },
        }
    }

    pub fn update(self, line: String) -> Self {
        match self {
            Self::Ready(_) => {
                unreachable!("An already completed ProspectiveContext should not be updated");
            }
            Self::Pending {
                before_context,
                after_context,
            } => {
                let after_context = after_context.feed(line);
                match after_context {
                    ProspectiveAfterContext::Ready(ctx) => Self::Ready(Context {
                        before: before_context,
                        after: ctx,
                    }),
                    ProspectiveAfterContext::Pending { .. } => Self::Pending {
                        before_context,
                        after_context,
                    },
                }
            }
        }
    }

    pub fn complete(self) -> Context {
        match self {
            Self::Ready(context) => context,
            Self::Pending {
                before_context,
                after_context,
            } => Context {
                before: before_context,
                after: after_context.complete(),
            },
        }
    }
}
