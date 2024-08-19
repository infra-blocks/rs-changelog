// TODO: merge both returning variants to just return the initialized builder.
// Review the API so it goes: consume_line -> check_result -> consume_line -> etc...
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitResult<T: NodeBuilder> {
    // This variant is returned when the builder could consume the first line and the corresponding node
    // could extend to the following lines.
    Builder(T),
    // This variant is returned when the matching node cannot be built from the provided line.
    /// The line wasn't consumed, so it should be used with [crate::block::factory::BuilderFactory::find_and_initialize_builder].
    Incompatible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuilderState {
    /// This variant is returned when the node is still in the process of being built.
    InProgress,
    /// This variant is returned when the builder has determined no further input could be
    /// aggregated to the node.
    Finished,
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

pub trait NodeBuilder {
    type Node;

    fn init_from_line(current: location::Position, line: &str) -> InitResult<Self>
    where
        Self: Sized;
    fn state(&self) -> BuilderState;
    /// Should only be called if the builder has the state [BuilderState::InProgress].
    fn maybe_add_line(&mut self, current: location::Position, line: &str) -> AddResult;
    // /// Should only be called if the builder has the state [BuilderState::Finished].
    // fn build() -> Self::Node;
}
