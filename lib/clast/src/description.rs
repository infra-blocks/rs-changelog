use markdown::mdast::Node;

use crate::markdown::NodeUtils;
use crate::node::TryFromNode;
use crate::{ChangelogNode, ChangelogNodeKind};
use crate::{ParseError, Position};

#[derive(Debug)]
pub struct Description {
    pub text: String,
    pub position: Position,
}

impl Description {
    pub fn new(position: Position, text: String) -> Self {
        Self { position, text }
    }
}

impl ChangelogNode for Description {
    fn node_kind() -> ChangelogNodeKind {
        ChangelogNodeKind::Description
    }
}

impl TryFromNode for Description {
    fn try_from_node(node: &Node) -> Result<Self, ParseError> {
        node.validate_paragraph()?;
        let position = node.unwrap_position();
        let text = node.children_text();
        Ok(Description::new(position, text))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from_nodes {
        use super::*;
        use test_utils::{
            fails_for_empty_nodes, fails_for_wrong_node, list_node, paragraph_node,
            works_with_valid_node,
        };

        fails_for_empty_nodes!(Description);
        fails_for_wrong_node!(Description, list_node(()));
        works_with_valid_node!(
            Description,
            paragraph_node("The Description"),
            |effective: Description, node: &Node| {
                assert_eq!(effective.text, "The Description");
                assert_eq!(effective.position, node.unwrap_position().into());
            }
        );
    }
}
