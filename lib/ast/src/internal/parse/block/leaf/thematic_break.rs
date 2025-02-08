use crate::internal::parse::segment::ThematicBreakSegment;
use segment::LineSegment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreak<'a>(pub ThematicBreakSegment<'a>);

impl<'a> ThematicBreak<'a> {
    pub fn new(segment: ThematicBreakSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> TryFrom<LineSegment<'a>> for ThematicBreak<'a> {
    type Error = LineSegment<'a>;

    fn try_from(segment: LineSegment<'a>) -> Result<Self, Self::Error> {
        ThematicBreakSegment::try_from(segment).map(ThematicBreak::new)
    }
}

impl<'a> From<ThematicBreakSegment<'a>> for ThematicBreak<'a> {
    fn from(value: ThematicBreakSegment<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> From<&ThematicBreakSegment<'a>> for ThematicBreak<'a> {
    fn from(value: &ThematicBreakSegment<'a>) -> Self {
        Self::new(*value)
    }
}
