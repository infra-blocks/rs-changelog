use crate::internal::parse::segment::BlankLineSegment;
use segment::LineSegment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlankLine<'a>(pub BlankLineSegment<'a>);

impl<'a> BlankLine<'a> {
    fn new(segment: BlankLineSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> From<BlankLineSegment<'a>> for BlankLine<'a> {
    fn from(segment: BlankLineSegment<'a>) -> Self {
        Self::new(segment)
    }
}

impl<'a> TryFrom<LineSegment<'a>> for BlankLine<'a> {
    type Error = LineSegment<'a>;

    fn try_from(segment: LineSegment<'a>) -> Result<Self, Self::Error> {
        Ok(Self::new(segment.try_into()?))
    }
}
