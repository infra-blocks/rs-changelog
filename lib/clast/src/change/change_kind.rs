use std::str::FromStr;

use markdown::mdast::Node;

use crate::markdown::NodeUtils;
use crate::node::TryFromNode;
use crate::{ChangelogNode, ChangelogNodeKind, ParseError};

// TODO: include position?
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ChangeKind {
    Added,
    Changed,
    Deprecated,
    Removed,
    Fixed,
    Security,
}

impl ChangelogNode for ChangeKind {
    fn node_kind() -> ChangelogNodeKind {
        ChangelogNodeKind::ChangeKind
    }
}

impl FromStr for ChangeKind {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Added" => Ok(ChangeKind::Added),
            "Changed" => Ok(ChangeKind::Changed),
            "Deprecated" => Ok(ChangeKind::Deprecated),
            "Removed" => Ok(ChangeKind::Removed),
            "Fixed" => Ok(ChangeKind::Fixed),
            "Security" => Ok(ChangeKind::Security),
            _ => Err(ParseError::invalid_text(s, "unknown change kind")),
        }
    }
}

impl TryFromNode for ChangeKind {
    fn try_from_node(node: &Node) -> Result<Self, ParseError> {
        node.validate_heading_with_depth(3)
            .map_err(|err| err.at_position(node.unwrap_position()))?;
        let text = node.children_text();
        let change_kind = text
            .parse()
            .map_err(|err: ParseError| err.at_position(node.unwrap_position()))?;
        Ok(change_kind)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from_str {
        use super::*;

        #[test]
        fn works_for_added() {
            assert_eq!(ChangeKind::Added, "Added".parse().unwrap());
        }

        #[test]
        fn works_for_changed() {
            assert_eq!(ChangeKind::Changed, "Changed".parse().unwrap());
        }

        #[test]
        fn works_for_deprecated() {
            assert_eq!(ChangeKind::Deprecated, "Deprecated".parse().unwrap());
        }

        #[test]
        fn works_for_removed() {
            assert_eq!(ChangeKind::Removed, "Removed".parse().unwrap());
        }

        #[test]
        fn works_for_fixed() {
            assert_eq!(ChangeKind::Fixed, "Fixed".parse().unwrap());
        }

        #[test]
        fn works_for_security() {
            assert_eq!(ChangeKind::Security, "Security".parse().unwrap());
        }

        #[test]
        fn fails_for_unknown() {
            let error = ChangeKind::from_str("Unknown").unwrap_err();
            assert!(error.is_invalid_text_error());
        }
    }

    mod try_from_nodes {
        use super::*;
        use crate::node::TryFromNodes;
        use test_utils::{
            fails_for_empty_nodes, fails_for_invalid_heading_depth, fails_for_invalid_text,
            fails_for_wrong_node, heading_node, list_node, works_with_valid_node,
        };

        fails_for_empty_nodes!(ChangeKind);
        fails_for_wrong_node!(ChangeKind, list_node(()));
        fails_for_invalid_heading_depth!(ChangeKind, 2);
        fails_for_invalid_text!(ChangeKind, heading_node((3, "Transmogrified")));
        works_with_valid_node!(
            ChangeKind,
            heading_node((3, "Added")),
            |change_kind: ChangeKind, _| {
                assert_eq!(change_kind, ChangeKind::Added);
            }
        );
    }
}
