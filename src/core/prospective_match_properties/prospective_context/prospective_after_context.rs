// TODO: docs

pub enum ProspectiveAfterContext {
    Ready(Option<Vec<String>>),
    Pending {
        collected: Vec<String>,
        missing: usize,
    },
}

impl ProspectiveAfterContext {
    pub fn new(size: usize) -> Self {
        match size {
            0 => Self::Ready(None),
            1.. => Self::Pending {
                collected: Vec::with_capacity(size),
                missing: size,
            },
        }
    }

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

    pub fn complete(self) -> Option<Vec<String>> {
        match self {
            Self::Ready(context) => context,
            Self::Pending { collected, .. } => Some(collected),
        }
    }
}

// TODO: tests
