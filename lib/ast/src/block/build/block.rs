use crate::{BlockNode, ContainerNode, LeafNode};

use super::{
    builder::{AddResult, BuilderState, InitResult, NodeBuilder},
    container::ContainerNodeBuilder,
    leaf::LeafNodeBuilder,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockNodeBuilder {
    Container(ContainerNodeBuilder),
    Leaf(LeafNodeBuilder),
}

impl NodeBuilder for BlockNodeBuilder {
    type Node = BlockNode;

    fn init_from_line(current: location::Position, line: &str) -> InitResult<Self>
    where
        Self: Sized,
    {
        match ContainerNodeBuilder::init_from_line(current, line) {
            InitResult::Builder(builder) => {
                InitResult::Builder(BlockNodeBuilder::Container(builder))
            }
            InitResult::Incompatible => match LeafNodeBuilder::init_from_line(current, line) {
                InitResult::Builder(builder) => {
                    InitResult::Builder(BlockNodeBuilder::Leaf(builder))
                }
                InitResult::Incompatible => InitResult::Incompatible,
            },
        }
    }

    fn state(&self) -> BuilderState {
        match self {
            BlockNodeBuilder::Container(builder) => builder.state(),
            BlockNodeBuilder::Leaf(builder) => builder.state(),
        }
    }

    fn maybe_add_line(&mut self, current: location::Position, line: &str) -> AddResult {
        match self {
            BlockNodeBuilder::Container(builder) => builder.maybe_add_line(current, line),
            BlockNodeBuilder::Leaf(builder) => builder.maybe_add_line(current, line),
        }
    }
}
