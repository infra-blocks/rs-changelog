use crate::{LeafNode, ParagraphNode};

use super::{
    builder::{AddResult, BuilderState, InitResult, NodeBuilder},
    paragraph::ParagraphNodeBuilder,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafNodeBuilder {
    ThematicBreak,
    AtxHeading,
    SetextHeading,
    IndentedCode,
    FencedCode,
    Html,
    LinkReferenceDefinition,
    Paragraph(ParagraphNodeBuilder),
    BlankLine,
}

impl NodeBuilder for LeafNodeBuilder {
    type Node = LeafNode;

    fn init_from_line(current: location::Position, line: &str) -> InitResult<Self>
    where
        Self: Sized,
    {
        todo!("implement me!")
    }

    fn state(&self) -> BuilderState {
        todo!("implement me!")
    }

    fn maybe_add_line(&mut self, current: location::Position, line: &str) -> AddResult {
        todo!("implement me!")
    }
}
