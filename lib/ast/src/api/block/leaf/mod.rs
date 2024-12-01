pub use atx_heading::*;
pub use blank_line::*;
pub use fenced_code::*;
pub use indented_code::*;
pub use link_reference_definition::*;
pub use paragraph::*;
pub use setext_heading::*;
pub use thematic_break::*;

mod atx_heading;
mod blank_line;
mod fenced_code;
mod indented_code;
mod link_reference_definition;
mod paragraph;
mod setext_heading;
mod thematic_break;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Leaf<'a> {
    AtxHeading(AtxHeading<'a>),
    BlankLine(BlankLine<'a>),
    FencedCode(FencedCode<'a>),
    IndentedCode(IndentedCode<'a>),
    /* Html, */
    LinkReferenceDefinition(LinkReferenceDefinition<'a>),
    Paragraph(Paragraph<'a>),
    SetextHeading(SetextHeading<'a>),
    ThematicBreak(ThematicBreak<'a>),
}

// Dems is test utils and should only be provided as such?
impl<'a> Leaf<'a> {
    pub fn unwrap_atx_heading(self) -> AtxHeading<'a> {
        match self {
            Leaf::AtxHeading(node) => node,
            _ => panic!("cannot unwrap AtxHeading on {:?}", self),
        }
    }

    pub fn unwrap_thematic_break(self) -> ThematicBreak<'a> {
        match self {
            Leaf::ThematicBreak(node) => node,
            _ => panic!("cannot unwrap ThematicBreak on {:?}", self),
        }
    }

    pub fn unwrap_indented_code(self) -> IndentedCode<'a> {
        match self {
            Leaf::IndentedCode(node) => node,
            _ => panic!("cannot unwrap IndentedCode on {:?}", self),
        }
    }

    pub fn unwrap_fenced_code(self) -> FencedCode<'a> {
        match self {
            Leaf::FencedCode(node) => node,
            _ => panic!("cannot unwrap FencedCode on {:?}", self),
        }
    }

    pub fn unwrap_paragraph(self) -> Paragraph<'a> {
        match self {
            Leaf::Paragraph(node) => node,
            _ => panic!("cannot unwrap Paragraph on {:?}", self),
        }
    }

    pub fn unwrap_blank_line(self) -> BlankLine<'a> {
        match self {
            Leaf::BlankLine(node) => node,
            _ => panic!("cannot unwrap BlankLine on {:?}", self),
        }
    }

    pub fn unwrap_setext_heading(self) -> SetextHeading<'a> {
        match self {
            Leaf::SetextHeading(node) => node,
            _ => panic!("cannot unwrap SetextHeading on {:?}", self),
        }
    }

    pub fn unwrap_link_reference_definition(self) -> LinkReferenceDefinition<'a> {
        match self {
            Leaf::LinkReferenceDefinition(node) => node,
            _ => panic!("cannot unwrap LinkReferenceDefinition on {:?}", self),
        }
    }
}

impl<'a> From<AtxHeading<'a>> for Leaf<'a> {
    fn from(node: AtxHeading<'a>) -> Self {
        Leaf::AtxHeading(node)
    }
}

impl<'a> From<BlankLine<'a>> for Leaf<'a> {
    fn from(node: BlankLine<'a>) -> Self {
        Leaf::BlankLine(node)
    }
}

impl<'a> From<FencedCode<'a>> for Leaf<'a> {
    fn from(node: FencedCode<'a>) -> Self {
        Leaf::FencedCode(node)
    }
}

impl<'a> From<IndentedCode<'a>> for Leaf<'a> {
    fn from(node: IndentedCode<'a>) -> Self {
        Leaf::IndentedCode(node)
    }
}

impl<'a> From<ThematicBreak<'a>> for Leaf<'a> {
    fn from(node: ThematicBreak<'a>) -> Self {
        Leaf::ThematicBreak(node)
    }
}
