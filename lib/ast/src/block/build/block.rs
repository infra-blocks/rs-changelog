use crate::{BlockNode, ContainerNode, LeafNode};

use super::builder::{AddResult, BuilderState, InitResult, NodeBuilder};

#[derive(Debug, Clone, PartialEq, Eq)]
enum BlockNodeBuilder<T, U>
where
    T: NodeBuilder<Node = ContainerNode>,
    U: NodeBuilder<Node = LeafNode>,
{
    Container(T),
    Leaf(U),
}

impl<T, U> NodeBuilder for BlockNodeBuilder<T, U>
where
    T: NodeBuilder<Node = ContainerNode>,
    U: NodeBuilder<Node = LeafNode>,
{
    type Node = BlockNode;

    fn init_from_line(current: location::Position, line: &str) -> InitResult<Self>
    where
        Self: Sized,
    {
        match T::init_from_line(current, line) {
            InitResult::Builder(builder) => {
                InitResult::Builder(BlockNodeBuilder::Container(builder))
            }
            InitResult::Incompatible => match U::init_from_line(current, line) {
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
