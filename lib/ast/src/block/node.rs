use crate::InlineNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockNode {
    Container(ContainerNode),
    Leaf(LeafNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerNode {
    BlockQuote(BlockQuoteNode),
    List(ListNode),
    ListItem(ListItemNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockQuoteNode {
    location: location::Span,
    children: Vec<BlockNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    location: location::Span,
    children: Vec<BlockNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItemNode {
    location: location::Span,
    children: Vec<BlockNode>,
}

// TODO: delineate... one node each.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafNode {
    ThematicBreak,
    AtxHeading,
    SetextHeading,
    IndentedCode,
    FencedCode,
    Html,
    LinkReferenceDefinition,
    Paragraph(ParagraphNode),
    BlankLine,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphNode {
    location: location::Span,
    children: Vec<InlineNode>,
}
