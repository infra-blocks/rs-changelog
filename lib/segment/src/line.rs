use crate::{segment_index::SegmentIndex, AsSegment};

use super::{Segment, SegmentLike};

// TODO: distinguish with the concept of [Line] and [LineSegment]. Most parsers are going to use straight up
// Lines. Lines can always be turned into a line segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Line<'a> {
    /// This variant represents a line of text that ends with a newline character.
    ///
    /// Note that, it is entirely possible that the last line of a text ends with a newline character.
    WithNewline(LineSegment<'a>),
    /// This variant represents a line of text that doesn't end with a newline character,
    /// and therefore is assumed to be the last line of the text.
    EndOfFile(LineSegment<'a>),
}

impl<'a> Line<'a> {
    pub(crate) fn new(line_segment: LineSegment<'a>) -> Self {
        if line_segment.text().ends_with('\n') {
            Line::WithNewline(line_segment)
        } else {
            Line::EndOfFile(line_segment)
        }
    }

    pub fn as_line_segment(&self) -> &LineSegment<'a> {
        match self {
            Line::WithNewline(segment) => segment,
            Line::EndOfFile(segment) => segment,
        }
    }

    pub fn as_segment(&self) -> &Segment<'a> {
        self.as_line_segment().as_segment()
    }
}

impl<'a> Default for Line<'a> {
    fn default() -> Self {
        Line::EndOfFile(LineSegment::default())
    }
}

impl<'a> From<LineSegment<'a>> for Line<'a> {
    fn from(segment: LineSegment<'a>) -> Self {
        Line::new(segment)
    }
}

/// [LineSegment] are a speciliazation of the [Segment] concept that represents text within a single line.
///
/// [LineSegment] can be contained in various ways and they always maintain this invariant:
/// - If a newline character exists, it must be the last character of the segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSegment<'a>(Segment<'a>);

impl<'a> Default for LineSegment<'a> {
    fn default() -> Self {
        Self::new(Segment::default())
    }
}

impl<'a> LineSegment<'a> {
    /// Creates a [LineSegment] from a [Segment].
    ///
    /// This function panics when the invariant is not met. For a safe version, use the [TryFrom] implementation
    /// instead.
    ///
    /// # Panics
    ///
    /// If the segment contains a newline character that is not the last character.
    pub fn new(segment: Segment<'a>) -> Self {
        segment.try_into().unwrap()
    }

    // TODO: tests.
    /// Constructs a line segment following the current one.
    ///
    /// This function is the unsafe version of [LineSegment::try_next].
    ///
    /// # Panics
    ///
    /// This function panics if the next segment is not a valid [LineSegment].
    pub fn next<'b>(&self, text: &'b str) -> LineSegment<'b> {
        self.try_next(text).unwrap()
    }

    /// Tries to construct a line segment following the current one.
    ///
    /// This function returns an error if the provided text cannot be turned into a valid [LineSegment].
    pub fn try_next<'b>(&self, text: &'b str) -> Result<LineSegment<'b>, ()> {
        let next_segment = Segment::new(self.end(), text);
        LineSegment::try_from(next_segment)
    }

    // TODO: tests.
    /// Slices the current segment at the provided indices, and returns an instance of [Segment].
    ///
    /// Because the invariant cannnot be broken by slicing, this function is safe and returns
    /// a [LineSegment] instance.
    pub fn slice<Idx: SegmentIndex>(&self, index: Idx) -> LineSegment<'a> {
        LineSegment::new(self.as_segment().slice(index))
    }

    // TODO: tests.
    pub fn trim_start(&self) -> LineSegment<'a> {
        LineSegment::new(self.as_segment().trim_start())
    }
}

impl<'a> AsSegment<'a> for LineSegment<'a> {
    fn as_segment(&self) -> &Segment<'a> {
        &self.0
    }
}

impl<'a> From<LineSegment<'a>> for Segment<'a> {
    fn from(segment: LineSegment<'a>) -> Self {
        *segment.as_segment()
    }
}

impl<'a> TryFrom<Segment<'a>> for LineSegment<'a> {
    type Error = ();

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if let Some(offset) = segment.text().find('\n') {
            if offset != segment.text().len() - 1 {
                return Err(());
            }
        }
        Ok(LineSegment(segment))
    }
}
