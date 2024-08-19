use std::fmt;

use super::BlockNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitResult<T: BlockNodeBuilder> {
    /// This variant is returned when the builder could unambiguously determine the matching node can only consume this line.
    /// In this case, no need to return a buffered builder.
    Finished(BlockNode),
    // This variant is returned when the builder could consume the first line and the corresponding node
    // could extend to the following lines.
    Building(T),
    // This variant is returned when the matching node cannot be built from the provided line.
    /// The line wasn't consumed, so it should be used with [crate::block::factory::BuilderFactory::find_and_initialize_builder].
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddResult {
    /// This variant repesents a line that was successfully consumed by the builder.
    /// The builder should be fed the folling line in this situation.
    Ok,
    /// This variant is returned when the builder cannot consume the line.
    /// The user should call [BlockNodeBuilder::finish] to get the buffered node.
    /// The line wasn't consumed, so it should be used with [crate::block::factory::BuilderFactory::find_and_initialize_builder].
    Incompatible,
}

pub trait BlockNodeBuilder: fmt::Debug {
    fn init_from_line(current: location::Position, line: &str) -> InitResult<Self>
    where
        Self: Sized;
    fn maybe_add_line(&mut self, current: location::Position, line: &str) -> AddResult;
    fn finish(self) -> BlockNode;
}
