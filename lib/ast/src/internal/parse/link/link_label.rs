use crate::{
    internal::parse::{
        segment::LinkLabelSegment,
        try_extract::{Extraction, TryExtract},
    },
    Segment,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkLabel<'a> {
    pub segment: LinkLabelSegment<'a>,
}

impl<'a> LinkLabel<'a> {
    pub fn new(segment: LinkLabelSegment<'a>) -> Self {
        Self { segment }
    }
}

impl<'a> TryFrom<Segment<'a>> for LinkLabel<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        segment.try_into().map(Self::new)
    }
}

impl<'a> TryExtract<Segment<'a>> for LinkLabel<'a> {
    type Remaining = Segment<'a>;
    type Error = Segment<'a>;

    fn try_extract(segment: Segment<'a>) -> Result<Extraction<Self, Segment<'a>>, Self::Error> {
        LinkLabelSegment::try_extract(segment).map(|extraction| {
            let extracted = Self::new(extraction.extracted);
            Extraction::new(extracted, extraction.remaining)
        })
    }
}
