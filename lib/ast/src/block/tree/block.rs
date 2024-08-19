use crate::Segment;

use super::{ingestion::Consume, LeafNode};

// BIG TODO: don't make "nodes" consume direcly. Use parsers with the Parser trait.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockNode<'a> {
    /* Container(ContainerNode<'a>), */
    Leaf(LeafNode<'a>),
}

impl<'a> BlockNode<'a> {
    pub fn unwrap_leaf(self) -> LeafNode<'a> {
        match self {
            BlockNode::Leaf(leaf_node) => leaf_node,
        }
    }
}

impl<'a> From<Segment<'a>> for BlockNode<'a> {
    fn from(segment: Segment<'a>) -> Self {
        if let Ok(leaf_node) = LeafNode::try_from(segment) {
            return BlockNode::Leaf(leaf_node);
        }
        panic!("unexpected invalid markdown: {:?}", segment.text())
    }
}

impl<'a> Consume<'a> for BlockNode<'a> {
    fn consume(&mut self, segment: Segment<'a>) -> super::ingestion::NodeState<'a> {
        match self {
            BlockNode::Leaf(leaf_node) => leaf_node.consume(segment),
        }
    }
}
