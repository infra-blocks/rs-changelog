use crate::Segment;

// TODO: on display, trim the last segment of trailing whitespace.

/// This struct represents a paragraph as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#paragraphs).
///
/// It can be constructed with the [ParagraphParser].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph<'a> {
    /// The segments making up the block.
    pub segments: Vec<Segment<'a>>,
}

impl<'a> Paragraph<'a> {
    /// Constructs a new [Paragraph] with the given segments.
    ///
    /// Meant to be called by the [ParagraphParser], or used into tests.
    pub(crate) fn new(segments: Vec<Segment<'a>>) -> Self {
        Self { segments }
    }
}
