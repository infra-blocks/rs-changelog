use crate::{BlockQuoteNode, ContainerNode};

use super::{
    block_quote::BlockQuoteNodeBuilder,
    builder::{AddResult, BuilderState, InitResult, NodeBuilder},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerNodeBuilder {
    BlockQuote(BlockQuoteNodeBuilder),
    List,
    ListItem,
}

impl NodeBuilder for ContainerNodeBuilder {
    type Node = ContainerNode;

    fn init_from_line(current: location::Position, line: &str) -> InitResult<Self>
    where
        Self: Sized,
    {
        match BlockQuoteNodeBuilder::init_from_line(current, line) {
            InitResult::Builder(builder) => {
                InitResult::Builder(ContainerNodeBuilder::BlockQuote(builder))
            }
            InitResult::Incompatible => todo!("implement me!"),
        }
    }

    fn state(&self) -> BuilderState {
        match self {
            ContainerNodeBuilder::BlockQuote(builder) => builder.state(),
            ContainerNodeBuilder::List => todo!("implement me!"),
            ContainerNodeBuilder::ListItem => todo!("implement me!"),
        }
    }

    fn maybe_add_line(&mut self, current: location::Position, line: &str) -> AddResult {
        match self {
            ContainerNodeBuilder::BlockQuote(builder) => builder.maybe_add_line(current, line),
            ContainerNodeBuilder::List => todo!("implement me!"),
            ContainerNodeBuilder::ListItem => todo!("implement me!"),
        }
    }
}
