use crate::changelog::error::ChangelogParseError;
use crate::changelog::markdown::from_slice::TryFromSlice;
use crate::changelog::markdown::{
    check_heading_with_depth, missing_node_error, Change, ChangelogNode, MarkdownError,
};
use markdown::mdast::{Heading, Node};

const NODE_TYPE: ChangelogNode = ChangelogNode::Release;

#[derive(Debug)]
pub struct Release {
    pub heading: Heading,
    pub changes: Vec<Change>,
}

impl TryFromSlice for Release {
    fn try_from_slice(
        slice: &[Node],
    ) -> Result<(&[Node], Self), ChangelogParseError<MarkdownError>> {
        let node = slice.get(0).ok_or_else(|| missing_node_error(NODE_TYPE))?;
        let heading = check_heading_with_depth(NODE_TYPE, node, 2)?;
        let (remaining, changes) = Vec::try_from_slice(&slice[1..])?;
        Ok((remaining, Release { heading, changes }))
    }
}
