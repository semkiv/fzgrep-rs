pub mod after_context;

use after_context::AfterContext;

use crate::match_properties::context::Context as CompleteContext;

/// Represents a context that may or may not have fully accumulated trailing (i.e. "after") context yet.
///
pub enum Context {
    /// A context that has already been fully accumulated
    /// (with an ready-to-use instance of [`crate::match_properties::context::Context`] inside).
    ///
    Ready(CompleteContext),

    /// A context whose trailing (i.e. "after") part that has not yet (or at all) been accumulated.
    ///
    Pending {
        /// Leading (i.e. "before") context (if any).
        ///
        before_context: Option<Vec<String>>,
        /// Trailing (i.e. "after") context. May or may not be complete.
        ///
        after_context: AfterContext,
    },
}

impl Context {
    /// Creates a [`Context`] with the requested "before" context and "after" context size.
    /// If and only if `after_context_size` is `0`, returns a [`Context::Ready`] instance
    /// with an empty trailing context.
    /// Otherwise returns a [`Context::Pending`] instance with an accordingly constructed
    /// instance of [`AfterContext`] as the "after" context.
    ///
    pub fn new(before_context: Option<Vec<String>>, after_context_size: usize) -> Self {
        let after_context = AfterContext::new(after_context_size);
        match after_context {
            AfterContext::Ready(ctx) => Self::Ready(CompleteContext {
                before: before_context,
                after: ctx,
            }),
            AfterContext::Pending { .. } => Self::Pending {
                before_context,
                after_context,
            },
        }
    }

    /// Updates the current instance of [`AfterContext`] by feeding `line`
    /// into the internal "after" context.
    /// If the after context becomes [`AfterContext::Ready`] after this,
    /// the current instance itself becomes [`Context::Ready`].
    ///
    /// # Panics
    ///
    ///   * Updating an instance of [`Context::Ready`] is considered a logic error
    ///     and therefore causes a panic.
    ///
    pub fn update(self, line: String) -> Self {
        match self {
            #[expect(clippy::panic, reason = "It is a logic error")]
            Self::Ready(_) => {
                panic!("An instance of 'Context' updated after completion");
            }
            Self::Pending {
                before_context,
                after_context,
            } => {
                let after_context = after_context.feed(line);
                match after_context {
                    AfterContext::Ready(ctx) => Self::Ready(CompleteContext {
                        before: before_context,
                        after: ctx,
                    }),
                    AfterContext::Pending { .. } => Self::Pending {
                        before_context,
                        after_context,
                    },
                }
            }
        }
    }

    /// "Completes" an instance of [`Context::Pending`] by completing
    /// the internal "after" context and returns any context collected at the time.
    ///
    /// # Panics
    ///
    ///   * Completing an instance of [`Context::Ready`] is considered a logic error
    ///     and therefore causes a panic.
    ///
    pub fn complete(self) -> CompleteContext {
        match self {
            #[expect(clippy::panic, reason = "It is a logic error")]
            Self::Ready(_) => {
                panic!("Attempted to complete an already completed instance of 'Context'")
            }
            Self::Pending {
                before_context,
                after_context,
            } => CompleteContext {
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
        let ctx = Context::new(
            Some(vec![String::from("before1"), String::from("before2")]),
            cap,
        );
        match ctx {
            Context::Ready(_) => unreachable!(),
            Context::Pending {
                before_context,
                after_context,
            } => {
                assert_eq!(before_context.unwrap(), vec!["before1", "before2"]);
                match after_context {
                    AfterContext::Ready(_) => unreachable!(),
                    AfterContext::Pending { collected, missing } => {
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
        let ctx = Context::new(
            Some(vec![String::from("before1"), String::from("before2")]),
            0,
        );
        match ctx {
            Context::Ready(ctx) => {
                assert_eq!(ctx.before.unwrap(), vec!["before1", "before2"]);
                assert_eq!(ctx.after, None);
            }
            Context::Pending { .. } => unreachable!(),
        }
    }

    #[test]
    fn update() {
        let ctx = Context::new(
            Some(vec![String::from("before1"), String::from("before2")]),
            3,
        );

        let ctx = ctx.update(String::from("line1"));
        match &ctx {
            Context::Ready(_) => unreachable!(),
            Context::Pending {
                before_context,
                after_context,
            } => {
                assert_eq!(
                    before_context.as_ref().unwrap(),
                    &vec!["before1", "before2"]
                );
                match after_context {
                    AfterContext::Ready(_) => unreachable!(),
                    AfterContext::Pending { collected, missing } => {
                        assert_eq!(collected, &vec!["after1"]);
                        assert_eq!(missing, &2);
                    }
                }
            }
        }

        let ctx = ctx.update(String::from("after2"));
        match &ctx {
            Context::Ready(_) => unreachable!(),
            Context::Pending {
                before_context,
                after_context,
            } => {
                assert_eq!(
                    before_context.as_ref().unwrap(),
                    &vec!["before1", "before2"]
                );
                match after_context {
                    AfterContext::Ready(_) => unreachable!(),
                    AfterContext::Pending { collected, missing } => {
                        assert_eq!(collected, &vec!["after1", "after2"]);
                        assert_eq!(missing, &1);
                    }
                }
            }
        }

        let ctx = ctx.update(String::from("after3"));
        match &ctx {
            Context::Ready(ctx) => {
                assert_eq!(ctx.before.as_ref().unwrap(), &vec!["before1", "before2"]);
                assert_eq!(
                    ctx.after.as_ref().unwrap(),
                    &vec!["after1", "after2", "after3"]
                );
            }
            Context::Pending { .. } => unreachable!(),
        }
    }

    #[test]
    #[should_panic(expected = "An instance of 'Context' updated after completion")]
    fn update_completed() {
        let ctx = Context::Ready(CompleteContext {
            before: Some(vec![String::from("before")]),
            after: Some(vec![String::from("after")]),
        });
        ctx.update(String::from("more after"));
    }

    #[test]
    fn complete() {
        let ctx = Context::Pending {
            before_context: Some(vec![String::from("before1"), String::from("before2")]),
            after_context: AfterContext::Pending {
                collected: vec![String::from("after1"), String::from("after2")],
                missing: 1,
            },
        };

        let ctx = ctx.complete();
        assert_eq!(ctx.before.unwrap(), vec!["before1", "before2"]);
        assert_eq!(ctx.after.unwrap(), vec!["after1", "after2"]);
    }

    #[test]
    #[should_panic(expected = "Attempted to complete an already completed instance of 'Context'")]
    fn complete_completed() {
        let ctx = Context::Ready(CompleteContext {
            before: Some(vec![String::from("before1"), String::from("before2")]),
            after: Some(vec![String::from("after1"), String::from("after2")]),
        });

        ctx.complete();
    }
}
