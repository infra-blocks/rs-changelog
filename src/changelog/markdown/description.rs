use std::convert::TryFrom;

use crate::changelog::error::ChangelogParseError;
use crate::changelog::markdown::from_slice::TryFromSlice;
use crate::changelog::markdown::{
    invalid_node_error, missing_node_error, ChangelogNode, MarkdownError, MarkdownNode,
};
use crate::changelog::position::Position;
use markdown::mdast::{Node, Paragraph};

const NODE_TYPE: ChangelogNode = ChangelogNode::Description;

#[derive(Debug)]
pub struct Description {
    pub paragraph: Paragraph,
}

impl TryFromSlice for Description {
    fn try_from_slice(
        slice: &[Node],
    ) -> Result<(&[Node], Self), ChangelogParseError<MarkdownError>> {
        let node = slice.get(0).ok_or_else(|| missing_node_error(NODE_TYPE))?;

        let description = node.try_into()?;

        Ok((&slice[1..], description))
    }
}

impl TryFrom<&Node> for Description {
    type Error = ChangelogParseError<MarkdownError>;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        let Node::Paragraph(paragraph) = node else {
            return Err(invalid_node_error(
                node.position().map(Position::from),
                NODE_TYPE,
                MarkdownNode::Paragraph,
                node.clone(),
            ));
        };
        Ok(Description {
            paragraph: paragraph.clone(),
        })
    }
}
