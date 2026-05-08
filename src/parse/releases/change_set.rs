use std::ops::Range;

pub use change::*;
pub use change_set_kind::*;

use changelog_ast::{HeadingLevel, Node};

use crate::parse::{node_ext::NodeExt, parser::Unparsed};

mod change {
    use std::ops::Range;

    use changelog_ast::{Item, Node};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Change {
        pub(crate) range: Range<usize>,
    }

    impl Change {
        pub fn new(range: Range<usize>) -> Self {
            Self { range }
        }
    }

    impl From<&Item<'_>> for Change {
        fn from(value: &Item<'_>) -> Self {
            Self::new(value.range.clone())
        }
    }

    impl TryFrom<&Node<'_>> for Change {
        type Error = Range<usize>;

        fn try_from(value: &Node<'_>) -> Result<Self, Self::Error> {
            match value {
                Node::Item(item) => Ok(item.into()),
                _ => Err(value.range().clone()),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod try_from {
            use changelog_ast::{CowStr, Text};

            use super::*;

            #[test]
            fn should_error_for_text() {
                let node = Node::Text(Text::new(0..12, CowStr::from("Please bro stfu")));
                assert_eq!(Change::try_from(&node), Err(0..12));
            }

            #[test]
            fn should_succeed_for_item() {
                let node = Node::Item(Item::new(
                    0..12,
                    vec![Node::Text(Text::new(
                        1..12,
                        CowStr::from("now you really gots to stfu gotta gotta gots to"),
                    ))],
                ));
                assert_eq!(Change::try_from(&node), Ok(Change::new(0..12)));
            }
        }
    }
}

mod change_set_kind {
    use changelog_ast::{Node, Text};

    /// An enum regrouping the allowed kinds of change sets.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ChangeSetKind {
        /// Added items in a change set.
        Added,
        /// Changed items in a change set.
        Changed,
        /// Deprecated items in a change set.
        Deprecated,
        /// Removed items in a change set.
        Removed,
        /// Fixed items in a change set.
        Fixed,
        /// Security items in a change set.
        Security,
    }

    impl TryFrom<&str> for ChangeSetKind {
        type Error = ();

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "Added" => Ok(Self::Added),
                "Changed" => Ok(Self::Changed),
                "Deprecated" => Ok(Self::Deprecated),
                "Removed" => Ok(Self::Removed),
                "Fixed" => Ok(Self::Fixed),
                "Security" => Ok(Self::Security),
                _ => Err(()),
            }
        }
    }

    impl TryFrom<&Text<'_>> for ChangeSetKind {
        type Error = ();

        fn try_from(value: &Text<'_>) -> Result<Self, Self::Error> {
            value.text.as_ref().try_into()
        }
    }

    impl TryFrom<&Node<'_>> for ChangeSetKind {
        type Error = ();

        fn try_from(value: &Node<'_>) -> Result<Self, Self::Error> {
            match value {
                Node::Text(text) => text.try_into(),
                _ => Err(()),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod try_from {
            use changelog_ast::{Code, CowStr, Node};

            use super::*;

            #[test]
            fn should_error_for_non_text_node() {
                let node = Node::Code(Code::new(0..12, CowStr::from("Added")));
                assert_eq!(ChangeSetKind::try_from(&node), Err(()));
            }

            #[test]
            fn should_error_for_unknown_text_content() {
                let node = Node::Text(Text::new(0..12, CowStr::from("Fuckulated")));
                assert_eq!(ChangeSetKind::try_from(&node), Err(()));
            }

            #[test]
            fn should_work_with_added_text_node() {
                let node = Node::Text(Text::new(0..12, CowStr::from("Added")));
                assert_eq!(ChangeSetKind::try_from(&node), Ok(ChangeSetKind::Added));
            }

            #[test]
            fn should_work_with_changed_text_node() {
                let node = Node::Text(Text::new(0..12, CowStr::from("Changed")));
                assert_eq!(ChangeSetKind::try_from(&node), Ok(ChangeSetKind::Changed));
            }

            #[test]
            fn should_work_with_deprecated_text_node() {
                let node = Node::Text(Text::new(0..12, CowStr::from("Deprecated")));
                assert_eq!(
                    ChangeSetKind::try_from(&node),
                    Ok(ChangeSetKind::Deprecated)
                );
            }

            #[test]
            fn should_work_with_removed_text_node() {
                let node = Node::Text(Text::new(0..12, CowStr::from("Removed")));
                assert_eq!(ChangeSetKind::try_from(&node), Ok(ChangeSetKind::Removed));
            }

            #[test]
            fn should_work_with_fixed_text_node() {
                let node = Node::Text(Text::new(0..12, CowStr::from("Fixed")));
                assert_eq!(ChangeSetKind::try_from(&node), Ok(ChangeSetKind::Fixed));
            }

            #[test]
            fn should_work_with_security_text_node() {
                let node = Node::Text(Text::new(0..12, CowStr::from("Security")));
                assert_eq!(ChangeSetKind::try_from(&node), Ok(ChangeSetKind::Security));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeSetParseError {
    /// This error happens when the parsing process runs out of nodes to inspect, but
    /// is still expecting some to produce a coherent result.
    Empty,
    /// When the expecting heading node is improper. For example, when it
    /// is not a markdown heading, or when the level is not the expected one.
    InvalidHeading(Range<usize>),
    /// When the content of the header does not match our expectations.
    ///
    /// This happens when it doesn't match one of the known strings identifying
    /// the change set kind. See [ChangeSetKind].
    InvalidHeader(Range<usize>),
    InvalidItem(Range<usize>),
    InvalidChangesList(Range<usize>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSet {
    heading: Range<usize>,
    items: Vec<Change>,
}

impl ChangeSet {
    pub fn new(heading: Range<usize>, items: Vec<Change>) -> Self {
        Self { heading, items }
    }

    /// Returns the range covering the whole change set.
    pub fn range(&self) -> Range<usize> {
        let start = self.heading.start;
        let end = self
            .items
            .last()
            .map(|c| c.range.end)
            .unwrap_or(self.heading.end);
        Range { start, end }
    }

    pub(crate) fn parse(ast: &mut Unparsed) -> Result<(ChangeSetKind, Self), ChangeSetParseError> {
        // The first node should be a heading of level 3, and its inner text should equal
        // "Added" verbatim.
        let Some(first) = ast.front() else {
            return Err(ChangeSetParseError::Empty);
        };

        // Checking the heading level validity.
        if !first.is_heading_of_level(HeadingLevel::H3) {
            return Err(ChangeSetParseError::InvalidHeading(first.range().clone()));
        }

        let kind = match first {
            Node::Heading(h) if h.children.len() == 1 => {
                match ChangeSetKind::try_from(&h.children[0]) {
                    Ok(kind) => kind,
                    Err(_) => return Err(ChangeSetParseError::InvalidHeader(h.range.clone())),
                }
            }
            _ => return Err(ChangeSetParseError::InvalidHeading(first.range().clone())),
        };

        // Now that we know the heading is good, we're looking into the next node,
        // which is expected to be a list.
        let Some(second) = ast.get(1) else {
            return Err(ChangeSetParseError::Empty);
        };

        if !second.is_list() {
            return Err(ChangeSetParseError::InvalidChangesList(
                second.range().clone(),
            ));
        }

        let items = second
            .children()
            .map(Change::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(ChangeSetParseError::InvalidItem)?;

        // If we make it here, we correctly parsed everything we needed. We took one heading node, and one list node from the AST.
        let heading_range = ast.pop_front().unwrap().unwrap_heading().range;
        // Just double checking our assumptions are correct still.
        ast.pop_front().unwrap().unwrap_list();
        Ok((kind, Self::new(heading_range, items)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::VecDeque;

    use changelog_ast::AstIterator;

    #[test]
    fn should_work_for_a_valid_added_change_set() {
        // TODO: make that fucking data struct already!!!!
        let mut ast: VecDeque<_> =
            AstIterator::new("### Added\n\n- Some sheeet\n- Big sheet").collect();
        assert_eq!(
            ChangeSet::parse(&mut ast),
            Ok((
                ChangeSetKind::Added,
                ChangeSet::new(0..10, vec![Change::new(11..25), Change::new(25..36)])
            ))
        );
        assert!(ast.is_empty());
    }
}
