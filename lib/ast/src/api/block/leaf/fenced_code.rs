use std::slice;

use crate::internal::parse;
use segment::LineSegment;

/// This struct represents a fenced code block as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#fenced-code-blocks).
///
/// It can be constructed with the [FencedCodeParser].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FencedCode<'a>(pub(crate) parse::block::FencedCode<'a>);

pub type ContentSegments<'a, 'b> = slice::Iter<'b, LineSegment<'a>>;

impl<'a> FencedCode<'a> {
    pub fn content_segments<'b>(&'b self) -> ContentSegments<'a, 'b> {
        match &self.0 {
            parse::block::FencedCode::Backticks(backticks) => backticks.content_segments.iter(),
            parse::block::FencedCode::Tildes(tildes) => tildes.content_segments.iter(),
        }
    }
}
