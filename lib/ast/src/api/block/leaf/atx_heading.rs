use crate::internal::parse;
use segment::{Segment, SegmentLike};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtxHeading<'a>(pub(crate) parse::block::AtxHeading<'a>);

impl<'a> AtxHeading<'a> {
    pub fn level(&self) -> u8 {
        self.0.level()
    }

    pub fn title(&self) -> Option<&'a str> {
        self.0.title()
    }

    pub fn segment(&self) -> Segment<'a> {
        Segment::new(
            self.0.segment.segment.start(),
            self.0.segment.segment.text(),
        )
    }
}
