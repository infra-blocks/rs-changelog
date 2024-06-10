use crate::changelog::error::ChangelogParseError;
use crate::changelog::markdown::from_slice::TryFromSlice;
use crate::changelog::markdown::{
    check_heading_with_depth, invalid_node_error, missing_node_error, ChangelogNode, MarkdownError,
    MarkdownNode,
};
use crate::changelog::position::Position;
use markdown::mdast::{Heading, ListItem, Node};

const NODE_TYPE: ChangelogNode = ChangelogNode::Change;

#[derive(Debug)]
pub struct Change {
    pub heading: Heading,
    pub items: Vec<Item>,
}

impl TryFromSlice for Change {
    fn try_from_slice(
        slice: &[Node],
    ) -> Result<(&[Node], Self), ChangelogParseError<MarkdownError>> {
        // TODO: make a function for that.
        let node = slice.first().ok_or_else(|| missing_node_error(NODE_TYPE))?;
        let heading = check_heading_with_depth(NODE_TYPE, node, 3)?;

        let (_, items) = match slice.get(1) {
            // TODO: enforce ordered is false
            Some(Node::List(list)) => Vec::try_from_slice(&list.children),
            Some(node) => Err(invalid_node_error(
                node.position().map(Position::from),
                NODE_TYPE,
                MarkdownNode::List,
                node.clone(),
            )),
            None => Err(missing_node_error(NODE_TYPE)),
        }?;
        // We took 2 nodes from the slice if we made it here.
        Ok((&slice[2..], Change { heading, items }))
    }
}

#[derive(Debug)]
pub struct Item {
    pub list_item: ListItem,
}

impl TryFromSlice for Item {
    fn try_from_slice(
        slice: &[Node],
    ) -> Result<(&[Node], Self), ChangelogParseError<MarkdownError>> {
        let node = slice.first().ok_or_else(|| missing_node_error(NODE_TYPE))?;

        // TODO: enforce checked is false.
        let Node::ListItem(list_item) = node else {
            return Err(invalid_node_error(
                node.position().map(Position::from),
                NODE_TYPE,
                MarkdownNode::Heading,
                node.clone(),
            ));
        };

        Ok((
            &slice[1..],
            Item {
                list_item: list_item.clone(),
            },
        ))
    }
}
