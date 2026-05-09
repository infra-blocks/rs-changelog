use crate::parse::{
    ast::Ast,
    releases::{Changes, ChangesParseError},
};
pub use heading::*;

/// The "Unreleased" section of a changelog.
///
/// Unlike the regular releases of a changelog, the unreleased
/// section does not have any version attributed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unreleased {
    /// The unreleased section heading.
    heading: UnreleasedHeading,
    /// The unreleased changes.
    changes: Changes,
}

impl Unreleased {
    pub fn new(heading: UnreleasedHeading, changes: Changes) -> Self {
        Self { heading, changes }
    }

    pub(crate) fn parse(ast: &mut Ast) -> Result<Self, UnreleasedParseError> {
        let heading = UnreleasedHeading::parse(ast)?;
        let changes = Changes::parse(ast)?;
        Ok(Self::new(heading, changes))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnreleasedParseError {
    InvalidHeading(UnreleasedHeadingParseError),
    InvalidChanges(ChangesParseError),
}

impl From<UnreleasedHeadingParseError> for UnreleasedParseError {
    fn from(value: UnreleasedHeadingParseError) -> Self {
        Self::InvalidHeading(value)
    }
}

impl From<ChangesParseError> for UnreleasedParseError {
    fn from(value: ChangesParseError) -> Self {
        Self::InvalidChanges(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use std::collections::VecDeque;

        use changelog_ast::AstIterator;

        use crate::parse::releases::{
            Changed,
            change_set::{Change, ChangeSet, ChangeSetParseError},
        };

        use super::*;

        // We won't be retesting the heading and the changes parsing here, as they are
        // already tested in their respective module. We're only going to test some
        // basic tests.
        #[test]
        fn should_error_for_invalid_heading() {
            let mut ast: VecDeque<_> = AstIterator::new(
                "## [Not Released]\n\n### Added\n- stuff\n\n[Not Released]: www.porque.pork.cunt",
            )
            .collect();
            let result = Unreleased::parse(&mut ast);
            assert_eq!(
                result,
                Err(UnreleasedParseError::InvalidHeading(
                    UnreleasedHeadingParseError::InvalidText(3..17)
                ))
            )
        }

        #[test]
        fn should_error_for_invalid_changes() {
            let mut ast: VecDeque<_> = AstIterator::new(
                "## [Unreleased]\n\n### Fuckulated\n- stuff\n\n[Unreleased]: www.porque.pork.cunt",
            )
            .collect();
            let result = Unreleased::parse(&mut ast);
            assert_eq!(
                result,
                Err(UnreleasedParseError::InvalidChanges(
                    ChangesParseError::InvalidChangeSet(ChangeSetParseError::InvalidHeader(17..32))
                ))
            )
        }

        #[test]
        fn should_succeed_for_valid_unreleased_block() {
            let mut ast: VecDeque<_> = AstIterator::new(
                "## [Unreleased]\n\n### Changed\n- stuff\n\n[Unreleased]: www.porque.pork.cunt",
            )
            .collect();
            let result = Unreleased::parse(&mut ast);
            assert_eq!(
                result,
                Ok(Unreleased::new(
                    UnreleasedHeading::new(0..16),
                    Changed::from(ChangeSet::new(17..29, vec![Change::new(29..38)])).into()
                ))
            );
        }
    }
}

mod heading {
    use std::ops::Range;

    use changelog_ast::{HeadingLevel, Node};

    use crate::parse::{ast::Ast, node_ext::NodeExt};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct UnreleasedHeading {
        pub(crate) range: Range<usize>,
    }

    impl UnreleasedHeading {
        pub fn new(range: Range<usize>) -> Self {
            Self { range }
        }

        pub(crate) fn parse(ast: &mut Ast) -> Result<Self, UnreleasedHeadingParseError> {
            let Some(first) = ast.front() else {
                return Err(UnreleasedHeadingParseError::Empty);
            };

            if !first.is_heading_of_level(HeadingLevel::H2) {
                return Err(UnreleasedHeadingParseError::InvalidNode(
                    first.range().clone(),
                ));
            }

            let result = match first {
                Node::Heading(heading) => {
                    let children = &heading.children;
                    // TODO: maybe collect the text at the heading level, then do a direct comparison instead
                    // of the nodes matching? Would save on code but would be every so slightly less performant.
                    // It would also allow to treat broken links later in the parsing flow.
                    if children.is_empty() {
                        Err(UnreleasedHeadingParseError::InvalidText(
                            children[0].range().start..children[children.len() - 1].range().end,
                        ))
                    } else if children.len() == 1 {
                        if !children[0].is_shortcut_link_with_id("Unreleased") {
                            Err(UnreleasedHeadingParseError::InvalidText(
                                children[0].range().clone(),
                            ))
                        } else {
                            Ok(UnreleasedHeading::new(first.range().clone()))
                        }
                    } else if children.len() >= 3
                        && children[0].is_text_equals("[")
                        && children[2].is_text_equals("]")
                    {
                        Err(UnreleasedHeadingParseError::BrokenLink(
                            children[0].range().start..children[2].range().end,
                        ))
                    } else {
                        Err(UnreleasedHeadingParseError::InvalidText(
                            children[0].range().start..children[children.len() - 1].range().end,
                        ))
                    }
                }
                _ => unreachable!(),
            };

            if result.is_ok() {
                ast.pop_front();
            }
            result
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum UnreleasedHeadingParseError {
        Empty,
        InvalidNode(Range<usize>),
        InvalidText(Range<usize>),
        BrokenLink(Range<usize>),
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod parse {
            use std::collections::VecDeque;

            use changelog_ast::AstIterator;

            use super::*;

            macro_rules! failure {
                ($markdown:expr, $error:expr) => {
                    let mut ast: VecDeque<_> = AstIterator::new($markdown).collect();
                    let result = UnreleasedHeading::parse(&mut ast);
                    assert_eq!(result, Err($error));
                };
            }

            #[test]
            fn should_error_for_empty_string() {
                failure!("", UnreleasedHeadingParseError::Empty);
            }

            #[test]
            fn should_error_for_invalid_node_kind() {
                failure!(
                    "not a heading bro",
                    UnreleasedHeadingParseError::InvalidNode(0..17)
                );
            }

            #[test]
            fn should_error_for_invalid_heading_level() {
                failure!(
                    "### [Unreleased]\n\n[Unreleased]: https://www.stfu.com/you-feel?answer=yes",
                    UnreleasedHeadingParseError::InvalidNode(0..17)
                );
            }

            #[test]
            fn should_error_for_invalid_heading_text() {
                failure!(
                    "## [Big Release]\n\n[Big Release]: https://www.stfu.com/you-feel?answer=yes",
                    UnreleasedHeadingParseError::InvalidText(3..16)
                );
            }

            #[test]
            fn should_error_for_broken_link() {
                failure!(
                    "## [Unreleased]\n### Added\n stuff",
                    UnreleasedHeadingParseError::BrokenLink(3..15)
                );
            }

            #[test]
            fn should_work_for_valid_markdown() {
                let mut ast: VecDeque<_> = AstIterator::new(
                    "## [Unreleased]\n\n[Unreleased]: https://www.stfu.com/you-feel?answer=yes",
                )
                .collect();
                let result = UnreleasedHeading::parse(&mut ast);
                assert_eq!(result, Ok(UnreleasedHeading::new(0..16)));
            }
        }
    }
}
