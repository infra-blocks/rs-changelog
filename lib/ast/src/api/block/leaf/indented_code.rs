use std::{
    iter::{once, FusedIterator},
    slice,
};

use segment::LineSegment;

use crate::internal::parse::{
    self,
    segment::{IndentedCodeOrBlankLineSegment, IndentedCodeSegment},
};

// TODO: unlike the paragraph's raw content, this one's formatting is preserved almost perfectly.
// New lines are new lines, spaces are not trimmed, etc...
// Blank lines are turned into a single newline.
// Leading and trailing blank lines are not included.
// TODO: an inner raw content struct might be useless here, since it is trivial to follow the
// spec rules to recreate from block segments.

/// This struct represents an indented code block as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#indented-code-blocks).
///
/// It can be constructed with the [IndentedCodeParser].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentedCode<'a>(pub(crate) parse::block::IndentedCode<'a>);

pub struct IndentedCodeSegments<'a, 'b> {
    opening_segment: Option<&'b IndentedCodeSegment<'a>>,
    continuation_segments: Option<slice::Iter<'b, IndentedCodeOrBlankLineSegment<'a>>>,
    closing_segment: Option<&'b IndentedCodeSegment<'a>>,
}

impl<'a, 'b> IndentedCodeSegments<'a, 'b> {
    pub fn new(
        opening_segment: Option<&'b IndentedCodeSegment<'a>>,
        continuation_segments: Option<slice::Iter<'b, IndentedCodeOrBlankLineSegment<'a>>>,
        closing_segment: Option<&'b IndentedCodeSegment<'a>>,
    ) -> Self {
        Self {
            opening_segment,
            continuation_segments,
            closing_segment,
        }
    }
}

impl<'a, 'b> Iterator for IndentedCodeSegments<'a, 'b> {
    type Item = LineSegment<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.opening_segment.take() {
            Some(opening_segment) => Some(opening_segment.into()),
            None => match self.continuation_segments.take() {
                Some(mut iter) => {
                    let result = iter.next().map(LineSegment::from);
                    if result.is_some() {
                        self.continuation_segments = Some(iter);
                    }
                    result
                }
                None => self.closing_segment.take().map(LineSegment::from),
            },
        }
    }
}

impl<'a, 'b> FusedIterator for IndentedCodeSegments<'a, 'b> {}

impl<'a, 'b> From<&'b IndentedCode<'a>> for IndentedCodeSegments<'a, 'b> {
    fn from(value: &'b IndentedCode<'a>) -> Self {
        let opening_segment = Some(&value.0.opening_segment);
        match &value.0.continuation_segments {
            Some(continuation_segments) => {
                let closing_segment = &continuation_segments.closing_segment;
                let continuation_segments = continuation_segments.segments.iter();
                IndentedCodeSegments::new(
                    opening_segment,
                    Some(continuation_segments),
                    Some(closing_segment),
                )
            }
            None => IndentedCodeSegments::new(opening_segment, None, None),
        }
    }
}

impl<'a> IndentedCode<'a> {
    pub fn line_segments(&self) -> impl Iterator<Item = LineSegment<'a>> {
        once(self.0.opening_segment.into())
    }
}
