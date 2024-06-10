use std::convert::TryFrom;

use crate::changelog::error::ChangelogParseError;
use crate::changelog::markdown::from_slice::TryFromSlice;
use crate::changelog::markdown::{
    check_heading_with_depth, missing_node_error, ChangelogNode, MarkdownError,
};
use markdown::mdast::{Heading, Node};

const NODE_TYPE: ChangelogNode = ChangelogNode::Title;

#[derive(Debug)]
pub struct Title {
    pub heading: Heading,
}

impl TryFromSlice for Title {
    fn try_from_slice(
        slice: &[Node],
    ) -> Result<(&[Node], Self), ChangelogParseError<MarkdownError>> {
        let node = slice.first().ok_or_else(|| missing_node_error(NODE_TYPE))?;
        let title = node.try_into()?;
        Ok((&slice[1..], title))
    }
}

impl TryFrom<&Node> for Title {
    type Error = ChangelogParseError<MarkdownError>;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        Ok(Title {
            heading: check_heading_with_depth(NODE_TYPE, node, 1)?,
        })
    }
}
