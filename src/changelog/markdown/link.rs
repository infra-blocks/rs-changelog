use crate::changelog::error::ChangelogParseError;
use crate::changelog::markdown::from_slice::TryFromSlice;
use crate::changelog::markdown::{
    invalid_node_error, missing_node_error, ChangelogNode, MarkdownError, MarkdownNode,
};
use crate::changelog::position::Position;
use markdown::mdast::{Definition, Node};

const NODE_TYPE: ChangelogNode = ChangelogNode::Link;

#[derive(Debug)]
pub struct Link {
    // TODO: check which node comes here.
    pub definition: Definition,
}

impl TryFromSlice for Link {
    fn try_from_slice(
        slice: &[Node],
    ) -> Result<(&[Node], Self), ChangelogParseError<MarkdownError>> {
        let node = slice.first().ok_or_else(|| missing_node_error(NODE_TYPE))?;

        let Node::Definition(definition) = node else {
            return Err(invalid_node_error(
                node.position().map(Position::from),
                NODE_TYPE,
                MarkdownNode::Definition,
                node.clone(),
            ));
        };

        Ok((
            &slice[1..],
            Link {
                definition: definition.clone(),
            },
        ))
    }
}
