use super::BlockNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildResult {
    // When a block is successfully built
    Success(BlockNode),
    // When a block is being built but waiting for more input.
    Building,
    // When the input does not correspond to the block type.
    // Example: trying to build a block quote with the line "- This is a list" will return this response.
    Incompatible,
}

pub trait BlockNodeBuilder {
    fn consume_line(&mut self, current: location::Position, line: &str) -> BuildResult;
}
