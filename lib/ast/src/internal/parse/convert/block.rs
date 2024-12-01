use crate::{
    block::{AtxHeading, BlankLine, Block, FencedCode, IndentedCode, Leaf, ThematicBreak},
    internal::parse,
    Segment,
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
    /// Constructs a new instance of [AtxHeading] with the given segment.
    fn new(segment: Segment<'a>, title: Option<&'a str>, level: u8) -> Self {
        Self {
            segment,
            title,
            level,
        }
    }
}

impl<'a> From<parse::segment::AtxHeadingSegment<'a>> for AtxHeading<'a> {
    fn from(value: parse::segment::AtxHeadingSegment<'a>) -> Self {
        Self::new(value.segment, value.title, value.level)
    }
}

impl<'a> From<parse::block::AtxHeading<'a>> for AtxHeading<'a> {
    fn from(value: parse::block::AtxHeading<'a>) -> Self {
        value.segment.into()
    }
}

impl<'a> BlankLine<'a> {
    /// Constructs a new instance of [BlankLine] with the given segment.
    fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }
}

impl<'a> From<parse::block::BlankLine<'a>> for BlankLine<'a> {
    fn from(value: parse::block::BlankLine<'a>) -> Self {
        BlankLine::new(value.segment.into())
    }
}

impl<'a> FencedCode<'a> {
    /// Constructs a new [FencedCode] with the given fields.
    pub(crate) fn new(
        opening_segment: Segment<'a>,
        info_string: Option<Segment<'a>>,
        content_segments: Vec<Segment<'a>>,
        closing_segment: Option<Segment<'a>>,
    ) -> Self {
        Self {
            opening_segment,
            info_string,
            content_segments,
            closing_segment: closing_segment,
        }
    }
}

impl<'a> From<parse::block::FencedCode<'a>> for FencedCode<'a> {
    fn from(fenced_code: parse::block::FencedCode<'a>) -> Self {
        match fenced_code {
            parse::block::FencedCode::Backticks(backticks) => Self::new(
                backticks.opening_segment.segment,
                backticks.opening_segment.info_string,
                backticks.content_segments,
                backticks.closing_segment.map(Segment::from),
            ),
            parse::block::FencedCode::Tildes(tildes) => Self::new(
                tildes.opening_segment.segment,
                tildes.opening_segment.info_string,
                tildes.content_segments,
                tildes.closing_segment.map(Segment::from),
            ),
        }
    }
}

impl<'a> From<parse::segment::BackticksFencedCodeClosingSegment<'a>> for Segment<'a> {
    fn from(value: parse::segment::BackticksFencedCodeClosingSegment<'a>) -> Self {
        value.segment
    }
}

impl<'a> From<parse::segment::TildesFencedCodeClosingSegment<'a>> for Segment<'a> {
    fn from(value: parse::segment::TildesFencedCodeClosingSegment<'a>) -> Self {
        value.segment
    }
}

impl<'a> IndentedCode<'a> {
    /// Constructs a new instance of [IndentedCode] with the given segments.
    fn new(segments: Vec<Segment<'a>>) -> Self {
        Self { segments }
    }
}

impl<'a> From<parse::block::IndentedCode<'a>> for IndentedCode<'a> {
    fn from(value: parse::block::IndentedCode<'a>) -> Self {
        match value.continuation_segments {
            Some(continuation_segments) => {
                let mut segments = Vec::with_capacity(2 + continuation_segments.segments.len());
                segments.push(value.opening_segment.into());
                for segment in continuation_segments.segments {
                    segments.push(segment.into());
                }
                segments.push(continuation_segments.closing_segment.into());
                IndentedCode::new(segments)
            }
            None => IndentedCode::new(vec![value.opening_segment.into()]),
        }
    }
}

impl<'a> ThematicBreak<'a> {
    /// Constructs a new instance of [ThematicBreak] with the given segment.
    fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }
}

impl<'a> From<parse::block::ThematicBreak<'a>> for ThematicBreak<'a> {
    fn from(value: parse::block::ThematicBreak<'a>) -> Self {
        ThematicBreak::new(value.segment.into())
    }
}
