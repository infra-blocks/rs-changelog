use std::ops::Range;

pub use change::*;
use change_set_kind::*;

use changelog_ast::{HeadingLevel, Node};

use crate::parse::{ast::Ast, node_ext::NodeExt};

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
        /// Fixed items in a change set.
        Fixed,
        /// Removed items in a change set.
        Removed,
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
    /// the change set kind.
    InvalidHeader(Range<usize>),
    InvalidItem(Range<usize>),
    InvalidChangesList(Range<usize>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeSet {
    Added(Added),
    Changed(Changed),
    Deprecated(Deprecated),
    Fixed(Fixed),
    Removed(Removed),
    Security(Security),
}

impl ChangeSet {
    pub fn range(&self) -> Range<usize> {
        match self {
            ChangeSet::Added(inner) => inner.range(),
            ChangeSet::Changed(inner) => inner.range(),
            ChangeSet::Deprecated(inner) => inner.range(),
            ChangeSet::Fixed(inner) => inner.range(),
            ChangeSet::Removed(inner) => inner.range(),
            ChangeSet::Security(inner) => inner.range(),
        }
    }

    pub fn is_added(&self) -> bool {
        matches!(self, ChangeSet::Added(_))
    }

    pub fn is_changed(&self) -> bool {
        matches!(self, ChangeSet::Changed(_))
    }

    pub fn is_deprecated(&self) -> bool {
        matches!(self, ChangeSet::Deprecated(_))
    }

    pub fn is_fixed(&self) -> bool {
        matches!(self, ChangeSet::Fixed(_))
    }

    pub fn is_removed(&self) -> bool {
        matches!(self, ChangeSet::Removed(_))
    }

    pub fn is_security(&self) -> bool {
        matches!(self, ChangeSet::Security(_))
    }

    pub(crate) fn parse(ast: &mut Ast) -> Result<Self, ChangeSetParseError> {
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
        Ok((kind, heading_range, items).into())
    }

    pub(crate) fn is_same_kind(&self, other: &ChangeSet) -> bool {
        self.is_added() && other.is_added()
            || self.is_changed() && other.is_changed()
            || self.is_deprecated() && other.is_deprecated()
            || self.is_fixed() && other.is_fixed()
            || self.is_removed() && other.is_removed()
            || self.is_security() && other.is_security()
    }
}

impl From<(ChangeSetKind, Range<usize>, Vec<Change>)> for ChangeSet {
    fn from(value: (ChangeSetKind, Range<usize>, Vec<Change>)) -> Self {
        match value.0 {
            ChangeSetKind::Added => Self::Added(Added::new(value.1, value.2)),
            ChangeSetKind::Changed => Self::Changed(Changed::new(value.1, value.2)),
            ChangeSetKind::Deprecated => Self::Deprecated(Deprecated::new(value.1, value.2)),
            ChangeSetKind::Fixed => Self::Fixed(Fixed::new(value.1, value.2)),
            ChangeSetKind::Removed => Self::Removed(Removed::new(value.1, value.2)),
            ChangeSetKind::Security => Self::Security(Security::new(value.1, value.2)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_work_for_a_valid_added_change_set() {
        let mut ast = Ast::from("### Added\n\n- Some sheeet\n- Big sheet");
        assert_eq!(
            ChangeSet::parse(&mut ast),
            Ok(ChangeSet::Added(Added::new(
                0..10,
                vec![Change::new(11..25), Change::new(25..36)]
            )))
        );
        assert!(ast.is_empty());
    }

    #[test]
    fn should_work_for_a_valid_changed_change_set() {
        let mut ast = Ast::from("### Changed\n\n- Some sheeet\n- Big sheet");
        assert_eq!(
            ChangeSet::parse(&mut ast),
            Ok(ChangeSet::Changed(Changed::new(
                0..12,
                vec![Change::new(13..27), Change::new(27..38)]
            )))
        );
        assert!(ast.is_empty());
    }

    #[test]
    fn should_work_for_a_valid_deprecated_change_set() {
        let mut ast = Ast::from("### Deprecated\n\n- Some sheeet\n- Big sheet");
        assert_eq!(
            ChangeSet::parse(&mut ast),
            Ok(ChangeSet::Deprecated(Deprecated::new(
                0..15,
                vec![Change::new(16..30), Change::new(30..41)]
            )))
        );
        assert!(ast.is_empty());
    }

    #[test]
    fn should_work_for_a_valid_fixed_change_set() {
        let mut ast = Ast::from("### Fixed\n\n- Some sheeet\n- Big sheet");
        assert_eq!(
            ChangeSet::parse(&mut ast),
            Ok(ChangeSet::Fixed(Fixed::new(
                0..10,
                vec![Change::new(11..25), Change::new(25..36)]
            )))
        );
        assert!(ast.is_empty());
    }

    #[test]
    fn should_work_for_a_valid_removed_change_set() {
        let mut ast = Ast::from("### Removed\n\n- Some sheeet\n- Big sheet");
        assert_eq!(
            ChangeSet::parse(&mut ast),
            Ok(ChangeSet::Removed(Removed::new(
                0..12,
                vec![Change::new(13..27), Change::new(27..38)]
            )))
        );
        assert!(ast.is_empty());
    }

    #[test]
    fn should_work_for_a_valid_security_change_set() {
        let mut ast = Ast::from("### Security\n\n- Some sheeet\n- Big sheet");
        assert_eq!(
            ChangeSet::parse(&mut ast),
            Ok(ChangeSet::Security(Security::new(
                0..13,
                vec![Change::new(14..28), Change::new(28..39)]
            )))
        );
        assert!(ast.is_empty());
    }
}

macro_rules! ChangeSetVariant {
    ($name:ident) => {
        #[derive(Debug, Default, Clone, PartialEq, Eq)]
        pub struct $name {
            heading: Range<usize>,
            items: Vec<Change>,
        }

        impl $name {
            pub(crate) fn new(heading: Range<usize>, items: Vec<Change>) -> Self {
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
                start..end
            }
        }

        impl From<$name> for ChangeSet {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }
    };
}

ChangeSetVariant!(Added);
ChangeSetVariant!(Changed);
ChangeSetVariant!(Deprecated);
ChangeSetVariant!(Fixed);
ChangeSetVariant!(Removed);
ChangeSetVariant!(Security);
