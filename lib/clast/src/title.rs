use markdown::mdast::Node;

use crate::markdown::NodeUtils;
use crate::node::TryFromNode;
use crate::{ChangelogNode, ChangelogNodeKind, ParseError, Position};

#[derive(Debug)]
pub struct Title {
    pub text: String,
    pub position: Position,
}

impl Title {
    pub fn new(position: Position, text: String) -> Self {
        Self { position, text }
    }
}

impl ChangelogNode for Title {
    fn node_kind() -> ChangelogNodeKind {
        ChangelogNodeKind::Title
    }
}

impl TryFromNode for Title {
    fn try_from_node(node: &Node) -> Result<Self, ParseError> {
        let position = node.unwrap_position();
        node.validate_heading_with_depth(1)
            .map_err(|err| err.at_position(position))?;
        let text = node.children_text();
        Ok(Title::new(position, text))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from_nodes {
        use test_utils::{
            fails_for_invalid_heading_depth, fails_for_wrong_node, heading_node, paragraph_node,
            works_with_valid_node,
        };

        use super::*;

        fails_for_wrong_node!(Title, paragraph_node("boom"));
        fails_for_invalid_heading_depth!(Title, 2);
        works_with_valid_node!(
            Title,
            heading_node((1, "The Title")),
            |effective: Title, node: &Node| {
                assert_eq!(effective.text, "The Title");
                assert_eq!(effective.position, node.unwrap_position());
            }
        );
    }
}
