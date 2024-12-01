use crate::Segment;

pub trait SegmentExt {
    fn is_blank_line(&self) -> bool;
}

impl<'a> SegmentExt for Segment<'a> {
    fn is_blank_line(&self) -> bool {
        self.text().trim().is_empty()
    }
}
