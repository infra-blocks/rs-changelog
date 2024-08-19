use crate::{LeafNode, ParagraphNode};

use super::builder::{AddResult, BuilderState, InitResult, NodeBuilder};

#[derive(Debug, Clone, PartialEq, Eq)]
enum LeafNodeBuilder<T>
where
    T: NodeBuilder<Node = ParagraphNode>,
{
    ThematicBreak,
    AtxHeading,
    SetextHeading,
    IndentedCode,
    FencedCode,
    Html,
    LinkReferenceDefinition,
    Paragraph(T),
    BlankLine,
}

impl<T> NodeBuilder for LeafNodeBuilder<T>
where
    T: NodeBuilder<Node = ParagraphNode>,
{
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
