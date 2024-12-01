use crate::{
    internal::parse::{
        segment::{LinkDestinationSegment, LooseLinkDestinationSegment},
        try_extract::{Extraction, TryExtract},
    },
    Segment,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkDestination<'a> {
    pub segment: LinkDestinationSegment<'a>,
}

impl<'a> LinkDestination<'a> {
    pub fn new(segment: LinkDestinationSegment<'a>) -> Self {
        Self { segment }
    }
}

impl<'a> TryFrom<Segment<'a>> for LinkDestination<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        Ok(Self::new(segment.try_into()?))
    }
}

impl<'a> TryExtract<Segment<'a>> for LinkDestination<'a> {
    type Remaining = Segment<'a>;
    type Error = Segment<'a>;

    fn try_extract(segment: Segment<'a>) -> Result<Extraction<Self, Segment<'a>>, Self::Error> {
        LinkDestinationSegment::try_extract(segment)
            .map(|extraction| extraction.map_extracted(Self::new))
    }
}
