use std::ops::{Range, RangeFrom, RangeTo};

// TODO: do those for inclusive ranges as well.

/// A utility trait for types that can be used to index segments.
pub trait SegmentIndex {
    /// The type must be able to produce a start offset.
    ///
    /// In the event that the return of this function is None, then
    /// the segment is assumed to start at the beginning of the text.
    fn start(&self) -> Option<usize>;
    /// The type must be able to produce an end offset.
    ///
    /// In the event that the return of this function is None, then
    /// the segment is assumed to end at the end of the text.
    fn end(&self) -> Option<usize>;
}

impl SegmentIndex for Range<usize> {
    fn start(&self) -> Option<usize> {
        Some(self.start)
    }

    fn end(&self) -> Option<usize> {
        Some(self.end)
    }
}

impl SegmentIndex for RangeFrom<usize> {
    fn start(&self) -> Option<usize> {
        Some(self.start)
    }

    fn end(&self) -> Option<usize> {
        None
    }
}

impl SegmentIndex for RangeTo<usize> {
    fn start(&self) -> Option<usize> {
        None
    }

    fn end(&self) -> Option<usize> {
        Some(self.end)
    }
}
