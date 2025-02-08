use location::Position;

use crate::Segment;

/// This trait defines commonalities between different segment types.
pub trait SegmentLike<'a> {
    /// Returns the start [Position] of this segment.
    ///
    /// This position is inclusive and points to the first character of the segment.
    fn start(&self) -> Position;

    /// Returns the end position of this segment.
    ///
    /// This position is exclusive and points to the character after the last of the segment.
    /// It is given by [location::Position::across] the text of the segment.
    fn end(&self) -> Position;

    /// Returns the text of this segment.
    ///
    /// The lifetime associated with the text is the same as the one inferred when constructing
    /// the segment, and not the one of the segment itself.
    fn text(&self) -> &'a str;

    /// Returns the length, in bytes, of this segment.
    fn len(&self) -> usize {
        self.text().len()
    }

    /// Returns whether this segment is made only of whitespaces or empty.
    fn is_blank(&self) -> bool {
        self.text().trim().is_empty()
    }
}

pub trait AsSegment<'a> {
    /// Returns a reference to this [LineSegment] as a [Segment].
    fn as_segment(&self) -> &Segment<'a>;
}

impl<'a, T> SegmentLike<'a> for T
where
    T: AsSegment<'a>,
{
    fn start(&self) -> Position {
        self.as_segment().start()
    }

    fn end(&self) -> Position {
        self.as_segment().end()
    }

    fn text(&self) -> &'a str {
        self.as_segment().text()
    }
}
