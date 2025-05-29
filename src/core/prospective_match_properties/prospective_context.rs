mod prospective_after_context;

use prospective_after_context::ProspectiveAfterContext;

use crate::match_properties::context::Context;

/// Represents a context that may or may not have fully accumulated trailing (i.e. "after") context yet.
///
pub enum ProspectiveContext {
    /// A context that has already been fully accumulated (with an ready-to-use instance of [`Context`] inside).
    ///
    Ready(Context),

    /// A context whose trailing (i.e. "after") part that has not yet (or at all) been accumulated.
    ///
    Pending {
        /// Leading (i.e. "before") context (if any).
        ///
        before_context: Option<Vec<String>>,
        /// Trailing (i.e. "after") context. May or may not be complete.
        ///
        after_context: ProspectiveAfterContext,
    },
}

impl ProspectiveContext {
    /// Creates a [`ProspectiveContext`] with the requested "before" context and "after" context size.
    /// If and only if `after_context_size` is `0`, returns a [`ProspectiveContext::Ready`] instance
    /// with an empty trailing context.
    /// Otherwise returns a [`ProspectiveContext::Pending`] instance with an accordingly constructed
    /// instance of [`ProspectiveAfterContext`] as the "after" context.
    ///
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

    /// Updates the current instance of [`ProspectiveAfterContext`] by feeding `line`
    /// into the internal "after" context.
    /// If the after context becomes [`ProspectiveAfterContext::Ready`] after this,
    /// the current instance itself becomes [`ProspectiveContext::Ready`].
    ///
    /// # Panics
    ///
    /// Updating an instance of [`ProspectiveContext::Ready`] is considered a logic error
    /// and therefore causes a panic.
    ///
    pub fn update(self, line: String) -> Self {
        match self {
            #[expect(clippy::panic, reason = "It is a logic error")]
            Self::Ready(_) => {
                panic!("An instance of 'ProspectiveContext' updated after completion");
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

    /// "Completes" an instance of [`ProspectiveContext::Pending`] by completing
    /// the internal "after" context and returns any context collected at the time.
    ///
    /// # Panics
    ///
    /// Completing an instance of [`ProspectiveContext::Ready`] is considered a logic error
    /// and therefore causes a panic.
    ///
    pub fn complete(self) -> Context {
        match self {
            #[expect(clippy::panic, reason = "It is a logic error")]
            Self::Ready(_) => panic!(
                "Attempted to complete an already completed instance of 'ProspectiveContext'"
            ),
            Self::Pending {
                before_context,
                after_context,
            } => Context {
                before: before_context,
                after: Some(after_context.complete()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::unreachable, reason = "It's tests, who cares?")]

    use super::*;

    #[test]
    fn constructor() {
        let cap = 42;
        let ctx = ProspectiveContext::new(
            Some(vec![String::from("before1"), String::from("before2")]),
            cap,
        );
        match ctx {
            ProspectiveContext::Ready(_) => unreachable!(),
            ProspectiveContext::Pending {
                before_context,
                after_context,
            } => {
                assert_eq!(before_context.unwrap(), vec!["before1", "before2"]);
                match after_context {
                    ProspectiveAfterContext::Ready(_) => unreachable!(),
                    ProspectiveAfterContext::Pending { collected, missing } => {
                        assert!(collected.is_empty());
                        assert_eq!(collected.capacity(), cap);
                        assert_eq!(missing, cap);
                    }
                }
            }
        }
    }

    #[test]
    fn constructor_zero_capacity() {
        let ctx = ProspectiveContext::new(
            Some(vec![String::from("before1"), String::from("before2")]),
            0,
        );
        match ctx {
            ProspectiveContext::Ready(ctx) => {
                assert_eq!(ctx.before.unwrap(), vec!["before1", "before2"]);
                assert_eq!(ctx.after, None);
            }
            ProspectiveContext::Pending { .. } => unreachable!(),
        }
    }

    #[test]
    fn feed() {
        let ctx = ProspectiveContext::new(
            Some(vec![String::from("before1"), String::from("before2")]),
            3,
        );

        let ctx = ctx.update(String::from("line1"));
        match &ctx {
            ProspectiveContext::Ready(_) => unreachable!(),
            ProspectiveContext::Pending {
                before_context,
                after_context,
            } => {
                assert_eq!(
                    before_context.as_ref().unwrap(),
                    &vec!["before1", "before2"]
                );
                match after_context {
                    ProspectiveAfterContext::Ready(_) => unreachable!(),
                    ProspectiveAfterContext::Pending { collected, missing } => {
                        assert_eq!(collected, &vec!["after1"]);
                        assert_eq!(missing, &2);
                    }
                }
            }
        }

        let ctx = ctx.update(String::from("after2"));
        match &ctx {
            ProspectiveContext::Ready(_) => unreachable!(),
            ProspectiveContext::Pending {
                before_context,
                after_context,
            } => {
                assert_eq!(
                    before_context.as_ref().unwrap(),
                    &vec!["before1", "before2"]
                );
                match after_context {
                    ProspectiveAfterContext::Ready(_) => unreachable!(),
                    ProspectiveAfterContext::Pending { collected, missing } => {
                        assert_eq!(collected, &vec!["after1", "after2"]);
                        assert_eq!(missing, &1);
                    }
                }
            }
        }

        let ctx = ctx.update(String::from("after3"));
        match &ctx {
            ProspectiveContext::Ready(ctx) => {
                assert_eq!(ctx.before.as_ref().unwrap(), &vec!["before1", "before2"]);
                assert_eq!(
                    ctx.after.as_ref().unwrap(),
                    &vec!["after1", "after2", "after3"]
                );
            }
            ProspectiveContext::Pending { .. } => unreachable!(),
        }
    }

    #[test]
    #[should_panic(expected = "An instance of 'ProspectiveContext' fed after completion")]
    fn feed_empty() {
        let ctx = ProspectiveContext::Ready(Context {
            before: None,
            after: None,
        });
        ctx.update(String::from("line"));
    }

    #[test]
    #[should_panic(expected = "An instance of 'ProspectiveContext' fed after completion")]
    fn feed_completed() {
        let ctx = ProspectiveContext::Ready(Context {
            before: Some(vec![String::from("before1"), String::from("before2")]),
            after: Some(vec![String::from("after1"), String::from("after2")]),
        });
        ctx.update(String::from("after3"));
    }

    #[test]
    fn complete() {
        let ctx = ProspectiveContext::Pending {
            before_context: Some(vec![String::from("before1"), String::from("before2")]),
            after_context: ProspectiveAfterContext::Pending {
                collected: vec![String::from("after1"), String::from("after2")],
                missing: 1,
            },
        };

        let ctx = ctx.complete();
        assert_eq!(ctx.before.unwrap(), vec!["before1", "before2"]);
        assert_eq!(ctx.after.unwrap(), vec!["after1", "after2"]);
    }

    #[test]
    #[should_panic(
        expected = "Attempted to complete an already completed instance of 'ProspectiveContext'"
    )]
    fn complete_completed() {
        let ctx = ProspectiveContext::Ready(Context {
            before: Some(vec![String::from("before1"), String::from("before2")]),
            after: Some(vec![String::from("after1"), String::from("after2")]),
        });

        ctx.complete();
    }
}
