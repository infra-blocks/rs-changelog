mod block_quote;
mod builder;
mod factory;
mod paragraph;

use crate::InlineNode;

/// Implementation of a Markdown Block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Container(Container),
    Leaf(Leaf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockNode {
    Container(ContainerNode),
    Leaf(LeafNode),
}

impl BlockNode {
    pub fn paragraph(location: location::Span, children: Vec<InlineNode>) -> Self {
        BlockNode::Leaf(LeafNode::paragraph(location, children))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Container {
    BlockQuote,
    List,
    ListItem,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerNode {
    kind: Container,
    location: location::Span,
    children: Vec<BlockNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Leaf {
    ThematicBreak,
    AtxHeading,
    SetextHeading,
    IndentedCode,
    FencedCode,
    Html,
    LinkReferenceDefinition,
    Paragraph,
    BlankLine,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafNode {
    kind: Leaf,
    location: location::Span,
    children: Vec<InlineNode>,
}

impl LeafNode {
    pub fn new(kind: Leaf, location: location::Span, children: Vec<InlineNode>) -> Self {
        LeafNode {
            kind,
            location,
            children,
        }
    }

    pub fn paragraph(location: location::Span, children: Vec<InlineNode>) -> Self {
        LeafNode::new(Leaf::Paragraph, location, children)
    }
}

// TODO: We need a pattern for BlocksBuilder, and trait objects for specific block builders,
// like BlockQuoteBuilder, ListBuilder, ListItemBuilder, etc.
// The container blocks, in turn, will have their own BlocksBuilder, and so on, until we reach
// leaf blocks
pub struct BlocksNodeBuilder {
    builder: Option<BlockNode>,
}
