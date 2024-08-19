use std::{iter::FusedIterator, str::SplitInclusive};

use location::Position;

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

    /// Returns the start position of this segment.
    ///
    /// This position is inclusive and points to the first character of the segment.
    pub fn start(&self) -> Position {
        self.start
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
        let segment = Segment::new(self.current.end(), text);
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
