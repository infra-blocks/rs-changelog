use crate::{block::tree::ParagraphNode, Segment};

use super::{
    blank_line::BlankLineNode,
    error::Error,
    ingestion::{Consume, NodeState},
    AtxHeadingNode, FencedCodeNode, IndentedCodeNode, LinkReferenceDefinitionNode,
    SetextHeadingNode, ThematicBreakNode,
};

// TODO: order alphabetically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafNode<'a> {
    ThematicBreak(ThematicBreakNode<'a>),
    AtxHeading(AtxHeadingNode<'a>),
    IndentedCode(IndentedCodeNode<'a>),
    FencedCode(FencedCodeNode<'a>),
    /* Html, */
    LinkReferenceDefinition(LinkReferenceDefinitionNode<'a>),
    Paragraph(ParagraphNode<'a>),
    BlankLine(BlankLineNode<'a>),
    SetextHeading(SetextHeadingNode<'a>),
}

impl<'a> LeafNode<'a> {
    pub fn unwrap_atx_heading(self) -> AtxHeadingNode<'a> {
        match self {
            LeafNode::AtxHeading(node) => node,
            _ => panic!("cannot unwrap AtxHeading on {:?}", self),
        }
    }

    pub fn unwrap_thematic_break(self) -> ThematicBreakNode<'a> {
        match self {
            LeafNode::ThematicBreak(node) => node,
            _ => panic!("cannot unwrap ThematicBreak on {:?}", self),
        }
    }

    pub fn unwrap_indented_code(self) -> IndentedCodeNode<'a> {
        match self {
            LeafNode::IndentedCode(node) => node,
            _ => panic!("cannot unwrap IndentedCode on {:?}", self),
        }
    }

    pub fn unwrap_fenced_code(self) -> FencedCodeNode<'a> {
        match self {
            LeafNode::FencedCode(node) => node,
            _ => panic!("cannot unwrap FencedCode on {:?}", self),
        }
    }

    pub fn unwrap_paragraph(self) -> ParagraphNode<'a> {
        match self {
            LeafNode::Paragraph(node) => node,
            _ => panic!("cannot unwrap Paragraph on {:?}", self),
        }
    }

    pub fn unwrap_blank_line(self) -> BlankLineNode<'a> {
        match self {
            LeafNode::BlankLine(node) => node,
            _ => panic!("cannot unwrap BlankLine on {:?}", self),
        }
    }

    pub fn unwrap_setext_heading(self) -> SetextHeadingNode<'a> {
        match self {
            LeafNode::SetextHeading(node) => node,
            _ => panic!("cannot unwrap SetextHeading on {:?}", self),
        }
    }

    pub fn unwrap_link_reference_definition(self) -> LinkReferenceDefinitionNode<'a> {
        match self {
            LeafNode::LinkReferenceDefinition(node) => node,
            _ => panic!("cannot unwrap LinkReferenceDefinition on {:?}", self),
        }
    }
}

impl<'a> TryFrom<Segment<'a>> for LeafNode<'a> {
    type Error = Error;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        // TODO: These should respect precedence ordering.
        if let Ok(node) = AtxHeadingNode::try_from(segment) {
            return Ok(LeafNode::AtxHeading(node));
        }
        if let Ok(node) = ThematicBreakNode::try_from(segment) {
            return Ok(LeafNode::ThematicBreak(node));
        }
        if let Ok(node) = IndentedCodeNode::try_from(segment) {
            return Ok(LeafNode::IndentedCode(node));
        }
        if let Ok(node) = FencedCodeNode::try_from(segment) {
            return Ok(LeafNode::FencedCode(node));
        }
        if let Ok(node) = BlankLineNode::try_from(segment) {
            return Ok(LeafNode::BlankLine(node));
        }
        // Paragraph node has the least precedence.
        // Because Setext headings always start off as paragraphs, we intentionally don't
        // attempt to construct them here, on first segment.
        if let Ok(node) = ParagraphNode::try_from(segment) {
            return Ok(LeafNode::Paragraph(node));
        }
        panic!("unexpected invalid markdown")
    }
}

impl<'a> Consume<'a> for LeafNode<'a> {
    fn consume(&mut self, segment: Segment<'a>) -> NodeState<'a> {
        match self {
            LeafNode::AtxHeading(node) => node.consume(segment),
            LeafNode::BlankLine(node) => node.consume(segment),
            LeafNode::FencedCode(node) => node.consume(segment),
            LeafNode::Paragraph(node) => node.consume(segment),
            LeafNode::IndentedCode(node) => node.consume(segment),
            LeafNode::SetextHeading(node) => node.consume(segment),
            LeafNode::ThematicBreak(node) => node.consume(segment),
            // TODO: remove at the end yo.
            _ => panic!("unsupported consume for node {:?}", self),
        }
    }
}
