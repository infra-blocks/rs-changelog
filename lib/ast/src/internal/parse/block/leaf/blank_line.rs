use crate::{internal::parse::segment::BlankLineSegment, Segment};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlankLine<'a> {
    pub segment: BlankLineSegment<'a>,
}

impl<'a> BlankLine<'a> {
    fn new(segment: BlankLineSegment<'a>) -> Self {
        Self { segment }
    }
}

impl<'a> From<BlankLineSegment<'a>> for BlankLine<'a> {
    fn from(segment: BlankLineSegment<'a>) -> Self {
        Self::new(segment)
    }
}

impl<'a> TryFrom<Segment<'a>> for BlankLine<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        Ok(Self::new(segment.try_into()?))
    }
}
