use std::{iter::FusedIterator, str::SplitInclusive};

use crate::{Line, LineSegment, Segment};

/// The iterator returned by the [StrExt::line_segments] method.
///
/// The iterator produces sequential [LineSegment]s from the text it was initialized with.
/// The text is split using the [str::split_inclusive] method, which includes the newline character in the produced segments.
/// This means the each line that isn't the last one is guaranteeed to be a [Line::WithNewline] variant.
/// The last line, can be either of the [LineSegment] variant, depending on its terminating character.
///
/// When a [Line::EndOfFile] is returned, it is guaranteed that the next call to [Iterator::next] will return [None].
/// It is also entirely possible that all the segments produced by the iterator are [Line::WithNewline] variants,
/// including the last one.
#[derive(Debug, Clone)]
pub struct Lines<'a> {
    current: Segment<'a>,
    inner: SplitInclusive<'a, char>,
}

impl<'a> Lines<'a> {
    fn new(inner: SplitInclusive<'a, char>) -> Self {
        Self {
            current: Segment::default(),
            inner,
        }
    }
}

impl<'a> From<&'a str> for Lines<'a> {
    fn from(text: &'a str) -> Self {
        Self::new(text.split_inclusive('\n'))
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.inner.next()?;
        let segment = self.current.next(text);
        self.current = segment;
        Some(Line::new(LineSegment::new(segment)).into())
    }
}

impl<'a> FusedIterator for Lines<'a> {}

/// Adds the [StrExt::line_segments] method to the [str] type.
pub trait SegmentStrExt<'a> {
    /// Returns an iterator over the lines of this text.
    ///
    /// The iterator splits inclusively the text at the newline character (`\n`) and
    /// produces successive [Line]s from the result. The terminating newline character
    /// is included in the produced items.
    fn split_lines(self) -> Lines<'a>;

    /// Transforms the slice into a single [Line].
    ///
    /// # Panics
    ///
    /// If the text contains a newline character that is not the last character.
    fn line(self) -> Line<'a>;
}

impl<'a> SegmentStrExt<'a> for &'a str {
    fn split_lines(self) -> Lines<'a> {
        self.into()
    }

    fn line(self) -> Line<'a> {
        Line::new(Segment::first(self).try_into().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod split_lines {
        use super::*;

        #[test]
        fn empty_text_should_produce_no_lines() {
            let text = "";
            assert_eq!(text.split_lines().count(), 0);
        }

        #[test]
        fn text_without_newline_should_produce_one_line() {
            let text = "This is a line!";
            let segments = text.split_lines().collect::<Vec<_>>();
            assert_eq!(
                segments,
                vec![Line::EndOfFile(Segment::first(text).try_into().unwrap())]
            );
        }

        #[test]
        fn text_ending_with_newline_should_produce_one_line() {
            let text = "This is a line!\n";
            let segments = text.split_lines().collect::<Vec<_>>();
            assert_eq!(
                segments,
                vec![Line::WithNewline(Segment::first(text).try_into().unwrap())]
            );
        }

        #[test]
        fn should_work_with_multiline_text_ending_without_newline() {
            let text = r"This is the first line.
This is the second line.
This is the third line.";
            let segments = text.split_lines().collect::<Vec<_>>();
            assert_eq!(
                segments,
                vec![
                    Line::WithNewline(
                        Segment::first("This is the first line.\n")
                            .try_into()
                            .unwrap()
                    ),
                    Line::WithNewline(
                        Segment::new(
                            location::Position::new(2, 1, 24),
                            "This is the second line.\n"
                        )
                        .try_into()
                        .unwrap()
                    ),
                    Line::EndOfFile(
                        Segment::new(location::Position::new(3, 1, 49), "This is the third line.")
                            .try_into()
                            .unwrap()
                    )
                ]
            );
        }

        #[test]
        fn should_work_with_multiline_text_ending_with_newline() {
            let text = r"This is the first line.
This is the second line.
This is the third line.
";
            let segments = text.split_lines().collect::<Vec<_>>();
            assert_eq!(
                segments,
                vec![
                    Line::WithNewline(
                        Segment::first("This is the first line.\n")
                            .try_into()
                            .unwrap()
                    ),
                    Line::WithNewline(
                        Segment::new(
                            location::Position::new(2, 1, 24),
                            "This is the second line.\n"
                        )
                        .try_into()
                        .unwrap()
                    ),
                    Line::WithNewline(
                        Segment::new(
                            location::Position::new(3, 1, 49),
                            "This is the third line.\n"
                        )
                        .try_into()
                        .unwrap()
                    )
                ]
            );
        }
    }

    mod line {
        use super::*;

        #[test]
        fn should_work_with_empty_string() {
            assert_eq!(
                "".line(),
                Line::EndOfFile(Segment::first("").try_into().unwrap())
            );
        }

        #[test]
        fn should_work_without_terminating_newline() {
            assert_eq!(
                "This is a line!".line(),
                Line::EndOfFile(Segment::first("This is a line!").try_into().unwrap())
            );
        }

        #[test]
        fn should_work_with_terminating_newline() {
            assert_eq!(
                "This is a line!\n".line(),
                Line::WithNewline(Segment::first("This is a line!\n").try_into().unwrap())
            );
        }

        #[test]
        #[should_panic]
        fn should_fail_with_non_terminating_newline() {
            "This is more than\njust a line!".line();
        }
    }
}
