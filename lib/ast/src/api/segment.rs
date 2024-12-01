use std::{
    iter::FusedIterator,
    ops::{Range, RangeFrom, RangeTo},
    str::SplitInclusive,
};

use location::Position;

// TODO: change position for offset if we don't make use of column and row count by
// the time we finish this module. Positions could be used in the changelog parser instead.

/// A segment of text that tracks its textual [location::Position].
///
/// Note: For the purposes of this crate, it is likely that the "position"
/// of a segment will be changed for an offset instead. It's not clear at
/// this moment that we would need the positioning mechanics here. We may
/// need it outside this crate for better error reporting. It would mean
/// that the ability to produce the position from the offset is available,
/// so the source text has to be available.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Segment<'a> {
    start: location::Position,
    text: &'a str,
}

impl<'a> Segment<'a> {
    /// Constructs a new [Segment] with the given start position and text.
    pub fn new(start: location::Position, text: &'a str) -> Self {
        Self { start, text }
    }

    /// Constructs an empty [Segment] with the given start position.
    pub fn empty_at(start: location::Position) -> Self {
        Self::new(start, "")
    }

    /// Constructs a new [Segment] with the provided text where the start
    /// position is given by [location::Position::first].
    pub fn first(text: &'a str) -> Self {
        Self::new(location::Position::first(), text)
    }

    /// Returns the position at the provided offset in the segment text.
    ///
    /// # Panics
    /// If the offset is out of bounds, or not pointing to a valid character
    /// start.
    pub fn at(&self, offset: usize) -> Position {
        // TODO: tests
        self.split_at(offset).0.end()
    }

    /// Constructs a new segment that comes right after this one with the provided text.
    ///
    /// In other words, it uses [Segment::new] with [Segment::end] as the start position.
    pub fn next(&self, next_text: &'a str) -> Self {
        // TODO: tests
        Self::new(self.end(), next_text)
    }

    /// Returns true if the segment text is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the length of the segment text.
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Splits the segment into 2 at the given offset.
    ///
    /// The first value of the tuple is the segment at [0, mid) and the second value is
    /// the segment at [mid, len).
    ///
    /// # Panics
    /// If the offset is out of bounds.
    pub fn split_at(&self, mid: usize) -> (Segment<'a>, Segment<'a>) {
        let (left, right) = self.text.split_at(mid);
        let left_segment = Segment::new(self.start, left);
        let right_segment = Segment::new(left_segment.end(), right);
        (left_segment, right_segment)
    }

    // TODO: implement for RangeFrom and RangeTo as well.
    pub fn slice<Idx: SegmentIndex>(&self, index: Idx) -> Segment<'a> {
        let start_offset = index.start().unwrap_or(0);
        let end_offset = index.end().unwrap_or(self.len());
        let substr = &self.text[start_offset..end_offset];
        Segment::new(self.at(start_offset), substr)
    }

    /// Returns the start position of this segment.
    ///
    /// This position is inclusive and points to the first character of the segment.
    pub fn start(&self) -> Position {
        self.start
    }

    /// Returns true if the segment text starts with the provided prefix, false otherwise.
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.text.starts_with(prefix)
    }

    /// Removes all leading whitespace characters from the segment text.
    pub fn trim_start(&self) -> Segment<'a> {
        let trimmed = self.text.trim_start();
        let trimmed_bytes_count = self.len() - trimmed.len();
        // If we trimmed it all, we return an empty segment.
        if trimmed_bytes_count == self.len() {
            return Segment::empty_at(self.start);
        } else {
            return self.slice(trimmed_bytes_count..self.len());
        }
    }

    /// Returns the end position of this segment.
    ///
    /// This position is exclusive and points to the character after the last of the segment.
    /// It is given by [location::Position::across] the text of the segment.
    pub fn end(&self) -> Position {
        self.start.walk(self.text)
    }

    /// Returns the text of this segment.
    pub fn text(&self) -> &'a str {
        self.text
    }
}

// TODO: finish and add the remaining ranges.
pub trait SegmentIndex {
    fn start(&self) -> Option<usize>;
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

// TODO: move to internal.
#[derive(Debug, Clone)]
pub struct LineSegments<'a> {
    current: Segment<'a>,
    inner: SplitInclusive<'a, char>,
}

impl<'a> LineSegments<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            current: Segment::default(),
            inner: text.split_inclusive('\n'),
        }
    }
}

impl<'a> Iterator for LineSegments<'a> {
    type Item = Segment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.inner.next()?;
        let segment = self.current.next(text);
        self.current = segment;
        Some(segment)
    }
}

impl<'a> FusedIterator for LineSegments<'a> {}

impl<'a> From<&'a str> for LineSegments<'a> {
    fn from(text: &'a str) -> Self {
        Self::new(text)
    }
}

/// Adds the [StrExt::line_segments] method to the [str] type.
pub trait StrExt<'a> {
    /// Returns an iterator over the line segments of this text.
    ///
    /// The iterator splits inclusively the text at the newline character (`\n`) and
    /// produces successive segments from the result.
    fn line_segments(self) -> LineSegments<'a>;
}

impl<'a> StrExt<'a> for &'a str {
    fn line_segments(self) -> LineSegments<'a> {
        self.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn should_work_for_empty_text_at_first_position() {
            let start = location::Position::first();
            let text = "";
            let segment = Segment::new(start, text);
            assert_eq!(segment.start, start);
            assert_eq!(segment.text, text);
        }

        #[test]
        fn should_work_for_any_segment() {
            let start = location::Position::new(5, 13, 127);
            let text = "Bitcoin will save us all!";
            let segment = Segment::new(start, text);
            assert_eq!(segment.start, location::Position::new(5, 13, 127));
            assert_eq!(segment.text, text);
        }
    }

    mod default {
        use super::*;

        #[test]
        fn should_produce_empty_segment_at_first_position() {
            let segment = Segment::default();
            assert_eq!(segment.start, location::Position::first());
            assert_eq!(segment.text, "");
        }
    }

    mod first {
        use super::*;

        #[test]
        fn should_work_for_empty_text() {
            let text = "";
            let segment = Segment::first(text);
            assert_eq!(segment.start, location::Position::first());
            assert_eq!(segment.text, text);
        }

        #[test]
        fn should_work_for_any_text() {
            let text = "Bitcoin replaces central banking!";
            let segment = Segment::first(text);
            assert_eq!(segment.start, location::Position::first());
            assert_eq!(segment.text, text);
        }
    }

    mod is_empty {
        use super::*;

        #[test]
        fn should_return_true_for_empty_segment() {
            let segment = Segment::default();
            assert!(segment.is_empty());
        }

        #[test]
        fn should_return_false_otherwise() {
            assert!(!Segment::first(" ").is_empty());
        }
    }

    mod split_at {
        use super::*;

        #[test]
        #[should_panic]
        fn should_panic_with_offset_out_of_bounds() {
            let segment = Segment::first("Bitcoin replaces central banking!");
            segment.split_at(100);
        }

        #[test]
        fn should_work_with_empty_segments() {
            let segment = Segment::default();
            let (left, right) = segment.split_at(0);
            assert_eq!(left, Segment::default());
            assert_eq!(right, Segment::default());
        }

        #[test]
        fn should_work_with_across_newline() {
            let segment = Segment::first("Make\nBig monies");
            let (left, right) = segment.split_at(5);
            assert_eq!(left, Segment::first("Make\n"));
            assert_eq!(
                right,
                Segment::new(location::Position::new(2, 1, 5), "Big monies")
            );
        }
    }

    mod trim_start {
        use super::*;

        #[test]
        fn should_work_with_empty_segment() {
            let segment = Segment::default();
            assert_eq!(segment.trim_start(), Segment::default());
        }

        #[test]
        fn should_work_with_segment_without_leading_whitespace() {
            let segment = Segment::first("Bitcoin replaces central banking!");
            assert_eq!(segment.trim_start(), segment);
        }

        #[test]
        fn should_stop_at_first_non_whitespace_character() {
            let segment = Segment::first(" \t\nBitcoin replaces central banking!");
            assert_eq!(
                segment.trim_start(),
                Segment::new(
                    location::Position::new(2, 1, 3),
                    "Bitcoin replaces central banking!"
                )
            );
        }
    }

    mod line_segments {
        use super::*;

        #[test]
        fn empty_text_should_produce_no_segments() {
            let text = "";
            assert_eq!(text.line_segments().count(), 0);
        }

        #[test]
        fn text_without_newline_should_produce_one_segment() {
            let text = "This is a line segment!";
            let segments = text.line_segments().collect::<Vec<_>>();
            assert_eq!(segments, vec![Segment::first(text)]);
        }

        #[test]
        fn text_ending_with_newline_should_produce_one_segment() {
            let text = "This is a line segment!\n";
            let segments = text.line_segments().collect::<Vec<_>>();
            assert_eq!(segments, vec![Segment::first(text)]);
        }

        #[test]
        fn should_work_with_multiline_text_ending_without_newline() {
            let text = r"This is the first segment.
This is the second segment.
This is the third segment.";
            let segments = text.line_segments().collect::<Vec<_>>();
            assert_eq!(
                segments,
                vec![
                    Segment::first("This is the first segment.\n"),
                    Segment::new(
                        location::Position::new(2, 1, 27),
                        "This is the second segment.\n"
                    ),
                    Segment::new(
                        location::Position::new(3, 1, 55),
                        "This is the third segment."
                    )
                ]
            );
        }
    }
}
