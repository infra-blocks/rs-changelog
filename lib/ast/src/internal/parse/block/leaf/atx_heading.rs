use crate::{internal::parse::segment::AtxHeadingSegment, Segment};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtxHeading<'a> {
    pub segment: AtxHeadingSegment<'a>,
}

impl<'a> AtxHeading<'a> {
    fn new(segment: AtxHeadingSegment<'a>) -> Self {
        Self { segment }
    }
}

impl<'a> TryFrom<Segment<'a>> for AtxHeading<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        AtxHeadingSegment::try_from(segment).map(AtxHeading::new)
    }
}
