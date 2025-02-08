use crate::{
    block::{AtxHeading, BlankLine, Block, FencedCode, IndentedCode, Leaf, ThematicBreak},
    internal::parse,
};

impl<'a> From<parse::block::Block<'a>> for Block<'a> {
    fn from(value: parse::block::Block<'a>) -> Self {
        match value {
            parse::block::Block::Leaf(leaf) => Block::Leaf(leaf.into()),
        }
    }
}

impl<'a> From<parse::block::Leaf<'a>> for Leaf<'a> {
    fn from(value: parse::block::Leaf<'a>) -> Self {
        match value {
            parse::block::Leaf::AtxHeading(atx_heading) => Leaf::AtxHeading(atx_heading.into()),
            parse::block::Leaf::BlankLine(blank_line) => Leaf::BlankLine(blank_line.into()),
            parse::block::Leaf::FencedCode(fenced_code) => Leaf::FencedCode(fenced_code.into()),
            parse::block::Leaf::IndentedCode(indented_code) => {
                Leaf::IndentedCode(indented_code.into())
            }
            parse::block::Leaf::ThematicBreak(thematic_break) => {
                Leaf::ThematicBreak(thematic_break.into())
            }
        }
    }
}

impl<'a> AtxHeading<'a> {
    /// Constructs a new instance of [AtxHeading] from the provided parsed result.
    fn new(atx_heading: parse::block::AtxHeading<'a>) -> Self {
        Self(atx_heading)
    }
}

impl<'a> From<parse::block::AtxHeading<'a>> for AtxHeading<'a> {
    fn from(value: parse::block::AtxHeading<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> BlankLine<'a> {
    /// Constructs a new instance of [BlankLine] from the provided parsed result.
    fn new(blank_line: parse::block::BlankLine<'a>) -> Self {
        Self(blank_line)
    }
}

impl<'a> From<parse::block::BlankLine<'a>> for BlankLine<'a> {
    fn from(value: parse::block::BlankLine<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> FencedCode<'a> {
    /// Constructs a new [FencedCode] from the provided parsed result.
    pub(crate) fn new(fenced_code: parse::block::FencedCode<'a>) -> Self {
        Self(fenced_code)
    }
}

impl<'a> From<parse::block::FencedCode<'a>> for FencedCode<'a> {
    fn from(value: parse::block::FencedCode<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> IndentedCode<'a> {
    /// Constructs a new instance of [IndentedCode] from the provided parsed result.
    fn new(indented_code: parse::block::IndentedCode<'a>) -> Self {
        Self(indented_code)
    }
}

impl<'a> From<parse::block::IndentedCode<'a>> for IndentedCode<'a> {
    fn from(value: parse::block::IndentedCode<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> ThematicBreak<'a> {
    /// Constructs a new instance of [ThematicBreak] from the provided parsed result.
    fn new(thematic_break: parse::block::ThematicBreak<'a>) -> Self {
        Self(thematic_break)
    }
}

impl<'a> From<parse::block::ThematicBreak<'a>> for ThematicBreak<'a> {
    fn from(value: parse::block::ThematicBreak<'a>) -> Self {
        Self::new(value)
    }
}
