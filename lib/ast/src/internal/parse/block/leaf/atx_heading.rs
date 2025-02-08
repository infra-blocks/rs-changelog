use crate::internal::parse::segment::AtxHeadingSegment;
use segment::LineSegment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtxHeading<'a> {
    pub segment: AtxHeadingSegment<'a>,
}

impl<'a> AtxHeading<'a> {
    fn new(segment: AtxHeadingSegment<'a>) -> Self {
        Self { segment }
    }

    pub fn level(&self) -> u8 {
        self.segment.level
    }

    pub fn title(&self) -> Option<&'a str> {
        self.segment.title
    }
}

impl<'a> TryFrom<LineSegment<'a>> for AtxHeading<'a> {
    type Error = LineSegment<'a>;

    fn try_from(segment: LineSegment<'a>) -> Result<Self, Self::Error> {
        AtxHeadingSegment::try_from(segment).map(AtxHeading::new)
    }
}
