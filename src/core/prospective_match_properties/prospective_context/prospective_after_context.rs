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
            Self::Ready(_) => {
                unreachable!("An already completed ProspectiveAfterContext should not be fed");
            }
            Self::Pending {
                mut collected,
                mut missing,
            } => {
                collected.push(line);
                missing -= 1;

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
