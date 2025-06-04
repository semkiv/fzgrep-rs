mod context;

use context::Context;

use crate::match_properties::MatchProperties as CompleteMatchProperties;
use crate::match_properties::location::Location;

use crate::details::basic::MatchProperties as BasicMatchProperties;

use vscode_fuzzy_score_rs::FuzzyMatch;

/// Represents match properties that may or may not have fully accumulated trailing (i.e. "after") context yet.
///
pub enum MatchProperties {
    /// Match properties whose trailing (i.e. "after") context has already been fully accumulated.
    ///
    /// # Fields
    ///   * a ready-to-use instance of [`crate::match_properties::MatchProperties`]
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
        ///
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
        basic_properties: BasicMatchProperties,
        before_context: Option<Vec<String>>,
        after_context_size: usize,
    ) -> Self {
        let BasicMatchProperties {
            matching_line,
            fuzzy_match,
            location,
        } = basic_properties;
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
                panic!("An instance of 'MatchProperties' updated after completion")
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

#[cfg(test)]
mod tests {
    #![expect(clippy::min_ident_chars, reason = "It's tests")]
    #![expect(clippy::too_many_lines, reason = "It's tests")]
    #![expect(clippy::unreachable, reason = "It's tests")]

    use super::*;
    use crate::match_properties::context::Context as CompleteContext;
    use context::after_context::AfterContext;

    #[test]
    fn constructor() {
        let line = String::from("test");
        let fm = vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap();
        let loc = Location {
            source_name: None,
            line_number: Some(42),
        };
        let basic_props = BasicMatchProperties {
            matching_line: line.clone(),
            fuzzy_match: fm.clone(),
            location: loc.clone(),
        };
        let before_ctx = Some(vec![String::from("before")]);
        let cap = 42;
        let props = MatchProperties::new(basic_props, before_ctx.clone(), cap);
        match &props {
            MatchProperties::Ready(_) => unreachable!(),
            MatchProperties::Pending {
                matching_line,
                fuzzy_match,
                location,
                context,
            } => {
                assert_eq!(matching_line, &line);
                assert_eq!(fuzzy_match, &fm);
                assert_eq!(location, &loc);
                match context {
                    Context::Ready(_) => unreachable!(),
                    Context::Pending {
                        before_context,
                        after_context,
                    } => {
                        assert_eq!(before_context, &before_ctx);
                        match after_context {
                            AfterContext::Ready(_) => unreachable!(),
                            AfterContext::Pending { collected, missing } => {
                                assert!(collected.is_empty());
                                assert_eq!(collected.capacity(), cap);
                                assert_eq!(missing, &cap);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn constructor_zero_capacity() {
        let line = String::from("test");
        let fm = vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap();
        let loc = Location {
            source_name: None,
            line_number: Some(42),
        };
        let basic_props = BasicMatchProperties {
            matching_line: line.clone(),
            fuzzy_match: fm.clone(),
            location: loc.clone(),
        };
        let before_ctx = Some(vec![String::from("before")]);
        let props = MatchProperties::new(basic_props, before_ctx.clone(), 0);
        match props {
            MatchProperties::Ready(props) => {
                assert_eq!(
                    props,
                    CompleteMatchProperties {
                        matching_line: line,
                        fuzzy_match: fm,
                        location: loc,
                        context: CompleteContext {
                            before: before_ctx,
                            after: None
                        }
                    }
                );
            }
            MatchProperties::Pending { .. } => unreachable!(),
        }
    }

    #[test]
    fn update() {
        let line = String::from("test");
        let fm = vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap();
        let loc = Location {
            source_name: None,
            line_number: Some(42),
        };
        let before_ctx = Some(vec![String::from("before1"), String::from("before2")]);
        let cap = 3;
        let props = MatchProperties::Pending {
            matching_line: line.clone(),
            fuzzy_match: fm.clone(),
            location: loc.clone(),
            context: Context::Pending {
                before_context: before_ctx.clone(),
                after_context: AfterContext::Pending {
                    collected: Vec::new(),
                    missing: cap,
                },
            },
        };

        let props = props.update(String::from("after1"));
        match &props {
            MatchProperties::Ready(_) => unreachable!(),
            MatchProperties::Pending {
                matching_line,
                fuzzy_match,
                location,
                context,
            } => {
                assert_eq!(matching_line, &line);
                assert_eq!(fuzzy_match, &fm);
                assert_eq!(location, &loc);
                match context {
                    Context::Ready(_) => unreachable!(),
                    Context::Pending {
                        before_context,
                        after_context,
                    } => {
                        assert_eq!(before_context, &before_ctx);
                        match after_context {
                            AfterContext::Ready(_) => unreachable!(),
                            AfterContext::Pending { collected, missing } => {
                                assert_eq!(collected, &vec!["after1"]);
                                assert_eq!(missing, &2);
                            }
                        }
                    }
                }
            }
        }

        let props = props.update(String::from("after2"));
        match &props {
            MatchProperties::Ready(_) => unreachable!(),
            MatchProperties::Pending {
                matching_line,
                fuzzy_match,
                location,
                context,
            } => {
                assert_eq!(matching_line, &line);
                assert_eq!(fuzzy_match, &fm);
                assert_eq!(location, &loc);
                match context {
                    Context::Ready(_) => unreachable!(),
                    Context::Pending {
                        before_context,
                        after_context,
                    } => {
                        assert_eq!(before_context, &before_ctx);
                        match after_context {
                            AfterContext::Ready(_) => unreachable!(),
                            AfterContext::Pending { collected, missing } => {
                                assert_eq!(collected, &vec!["after1", "after2"]);
                                assert_eq!(missing, &1);
                            }
                        }
                    }
                }
            }
        }

        let props = props.update(String::from("after3"));
        match &props {
            MatchProperties::Ready(props) => {
                assert_eq!(
                    props,
                    &CompleteMatchProperties {
                        matching_line: line,
                        fuzzy_match: fm,
                        location: loc,
                        context: CompleteContext {
                            before: before_ctx,
                            after: Some(vec![
                                String::from("after1"),
                                String::from("after2"),
                                String::from("after3")
                            ]),
                        }
                    }
                );
            }
            MatchProperties::Pending { .. } => unreachable!(),
        }
    }

    #[test]
    #[should_panic(expected = "An instance of 'MatchProperties' updated after completion")]
    fn update_completed() {
        let props = MatchProperties::Ready(CompleteMatchProperties {
            matching_line: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
            location: Location {
                source_name: None,
                line_number: Some(42),
            },
            context: CompleteContext {
                before: Some(vec![String::from("before")]),
                after: Some(vec![String::from("after")]),
            },
        });
        props.update(String::from("more after"));
    }

    #[test]
    fn complete() {
        let line = String::from("test");
        let fm = vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap();
        let loc = Location {
            source_name: None,
            line_number: Some(42),
        };
        let before_ctx = Some(vec![String::from("before")]);
        let cap = 3;

        let props = MatchProperties::Pending {
            matching_line: line.clone(),
            fuzzy_match: fm.clone(),
            location: loc.clone(),
            context: Context::Pending {
                before_context: before_ctx.clone(),
                after_context: AfterContext::Pending {
                    collected: vec![String::from("after1"), String::from("after2")],
                    missing: cap,
                },
            },
        };

        let props = props.complete();
        assert_eq!(
            props,
            CompleteMatchProperties {
                matching_line: line,
                fuzzy_match: fm,
                location: loc,
                context: CompleteContext {
                    before: before_ctx,
                    after: Some(vec![String::from("after1"), String::from("after2")])
                }
            }
        );
    }

    #[test]
    #[should_panic(
        expected = "Attempted to complete an already completed instance of 'MatchProperties'"
    )]
    fn complete_completed() {
        let props = MatchProperties::Ready(CompleteMatchProperties {
            matching_line: String::from("test"),
            fuzzy_match: vscode_fuzzy_score_rs::fuzzy_match("test", "test").unwrap(),
            location: Location {
                source_name: None,
                line_number: Some(42),
            },
            context: CompleteContext {
                before: Some(vec![String::from("before")]),
                after: Some(vec![String::from("after")]),
            },
        });
        props.complete();
    }
}
