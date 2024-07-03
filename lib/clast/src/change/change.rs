use markdown::mdast::Node;

use crate::markdown::NodeUtils;
use crate::node::TryFromNode;
use crate::{ChangelogNode, ChangelogNodeKind, ParseError, Position};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub position: Position,
    pub text: String,
}

impl Change {
    pub fn new(position: Position, text: String) -> Self {
        Self { position, text }
    }
}

impl ChangelogNode for Change {
    fn node_kind() -> ChangelogNodeKind {
        ChangelogNodeKind::Change
    }
}

impl TryFromNode for Change {
    fn try_from_node(node: &Node) -> Result<Self, ParseError> {
        node.validate_list_item()
            .map_err(|err| err.at_position(node.unwrap_position()))?;
        let position = node.unwrap_position();
        let text = node.children_text();
        Ok(Change::new(position, text))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from_nodes {
        use super::*;
        use crate::node::TryFromNodes;
        use test_utils::{
            fails_for_empty_nodes, fails_for_wrong_node, list_item_node, list_node,
            works_with_valid_node,
        };

        fails_for_empty_nodes!(Change);
        fails_for_wrong_node!(Change, list_node(()));
        works_with_valid_node!(
            Change,
            list_item_node("stuff"),
            |effective: Change, node: &Node| {
                assert_eq!(effective.position, node.unwrap_position());
                assert_eq!(effective.text, "stuff");
            }
        );
    }
}
