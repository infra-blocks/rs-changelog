use crate::{internal::parse::segment::ThematicBreakSegment, Segment};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreak<'a> {
    pub segment: ThematicBreakSegment<'a>,
}

impl<'a> ThematicBreak<'a> {
    pub fn new(segment: ThematicBreakSegment<'a>) -> Self {
        Self { segment }
    }
}

impl<'a> TryFrom<Segment<'a>> for ThematicBreak<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        ThematicBreakSegment::try_from(segment).map(ThematicBreak::new)
    }
}

impl<'a> From<ThematicBreakSegment<'a>> for ThematicBreak<'a> {
    fn from(value: ThematicBreakSegment<'a>) -> Self {
        Self::new(value)
    }
}
