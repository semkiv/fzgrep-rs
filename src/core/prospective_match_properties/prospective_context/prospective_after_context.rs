/// Represents a trailing (i.e. "after") context that may or may not have been fully accumulated yet.
///
pub enum ProspectiveAfterContext {
    /// A context that has already been fully accumulated (with the accumulated lines inside).
    ///
    Ready(Option<Vec<String>>),

    /// A context that has not yet (or at all) been accumulated.
    ///
    Pending {
        /// Lines collected so far.
        ///
        collected: Vec<String>,

        /// Number of still missing lines.
        missing: usize,
    },
}

impl ProspectiveAfterContext {
    /// Creates a [`ProspectiveAfterContext`] with the requested size.
    /// If and only if `size` is `0`, returns a [`ProspectiveAfterContext::Ready`] instance.
    /// Otherwise returns a [`ProspectiveAfterContext::Pending`] instance with no collected lines and
    /// the number missing lines equal to `size`.
    ///
    pub fn new(size: usize) -> Self {
        match size {
            0 => Self::Ready(None),
            1.. => Self::Pending {
                collected: Vec::with_capacity(size),
                missing: size,
            },
        }
    }

    /// Feeds a line into the current instance of [`ProspectiveAfterContext`].
    /// If the instance is [`ProspectiveAfterContext::Pending`] before being fed,
    /// it may or may not become [`ProspectiveAfterContext::Ready`] after.
    ///
    /// # Panics
    ///
    /// Feeding into an instance of [`ProspectiveAfterContext::Ready`] is considered a logic error
    /// and therefore causes a panic.
    ///
    pub fn feed(self, line: String) -> Self {
        match self {
            #[expect(clippy::panic, reason = "It is a logic error")]
            Self::Ready(_) => {
                panic!("An instance of 'ProspectiveAfterContext' fed after completion");
            }
            Self::Pending {
                mut collected,
                missing,
            } => {
                collected.push(line);
                #[expect(
                    clippy::expect_used,
                    reason = "The missing line count is expected to go down to zero by one at a time"
                )]
                missing
                    .checked_sub(1)
                    .expect("The missing lines count is negative");

                if missing == 0 {
                    return Self::Ready(Some(collected));
                }

                Self::Pending { collected, missing }
            }
        }
    }

    /// "Completes" an instance of [`ProspectiveAfterContext::Pending`]
    /// and returns any lines collected so far.
    ///
    /// # Panics
    ///
    /// Completing an instance of [`ProspectiveAfterContext::Ready`] is considered a logic error
    /// and therefore causes a panic.
    ///
    pub fn complete(self) -> Vec<String> {
        match self {
            #[expect(clippy::panic, reason = "It is a logic error")]
            Self::Ready(_) => panic!(
                "Attempted to complete an already completed instance of 'ProspectiveAfterContext'"
            ),
            Self::Pending { collected, .. } => collected,
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
        let ctx = ProspectiveAfterContext::new(cap);
        match ctx {
            ProspectiveAfterContext::Ready(_) => unreachable!(),
            ProspectiveAfterContext::Pending { collected, missing } => {
                assert!(collected.is_empty());
                assert_eq!(collected.capacity(), cap);
                assert_eq!(missing, cap);
            }
        }
    }

    #[test]
    fn constructor_zero_capacity() {
        let ctx = ProspectiveAfterContext::new(0);
        match ctx {
            ProspectiveAfterContext::Ready(ctx) => {
                assert!(ctx.is_none());
            }
            ProspectiveAfterContext::Pending { .. } => unreachable!(),
        }
    }

    #[test]
    fn feed() {
        let ctx = ProspectiveAfterContext::new(3);

        let ctx = ctx.feed(String::from("line1"));
        match &ctx {
            ProspectiveAfterContext::Ready(_) => unreachable!(),
            ProspectiveAfterContext::Pending { collected, missing } => {
                assert_eq!(collected, &vec!["line1"]);
                assert_eq!(missing, &2);
            }
        }

        let ctx = ctx.feed(String::from("line2"));
        match &ctx {
            ProspectiveAfterContext::Ready(_) => unreachable!(),
            ProspectiveAfterContext::Pending { collected, missing } => {
                assert_eq!(collected, &vec!["line1", "line2"]);
                assert_eq!(missing, &1);
            }
        }

        let ctx = ctx.feed(String::from("line3"));
        match &ctx {
            ProspectiveAfterContext::Ready(ctx) => {
                assert_eq!(ctx.as_ref().unwrap(), &vec!["line1", "line2", "line3"]);
            }
            ProspectiveAfterContext::Pending { .. } => unreachable!(),
        }
    }

    #[test]
    #[should_panic(expected = "An instance of 'ProspectiveAfterContext' fed after completion")]
    fn feed_empty() {
        let ctx = ProspectiveAfterContext::Ready(None);
        ctx.feed(String::from("line"));
    }

    #[test]
    #[should_panic(expected = "An instance of 'ProspectiveAfterContext' fed after completion")]
    fn feed_completed() {
        let ctx = ProspectiveAfterContext::Ready(Some(vec![
            String::from("line1"),
            String::from("line2"),
        ]));
        ctx.feed(String::from("line3"));
    }

    #[test]
    fn complete() {
        let ctx = ProspectiveAfterContext::Pending {
            collected: vec![String::from("line1"), String::from("line2")],
            missing: 1,
        };

        assert_eq!(ctx.complete(), vec!["line1", "line2"]);
    }

    #[test]
    #[should_panic(
        expected = "Attempted to complete an already completed instance of 'ProspectiveAfterContext'"
    )]
    fn complete_completed() {
        let ctx = ProspectiveAfterContext::Ready(Some(vec![
            String::from("line1"),
            String::from("line2"),
        ]));

        ctx.complete();
    }
}
