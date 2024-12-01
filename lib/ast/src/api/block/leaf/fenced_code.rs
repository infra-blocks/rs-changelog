use crate::Segment;

/// This struct represents a fenced code block as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#fenced-code-blocks).
///
/// It can be constructed with the [FencedCodeParser].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FencedCode<'a> {
    /// The opening segment of the block.
    ///
    /// Contains the fence and the info string.
    pub opening_segment: Segment<'a>,
    /// The info string follows the opening segment's symbol.
    ///
    /// It's typically used to identify the language of the code block.
    /// Only set if there was a least one non whitespace character provided.
    pub info_string: Option<Segment<'a>>,
    /// The main content of the fenced code block.
    pub content_segments: Vec<Segment<'a>>,
    /// The closing segment of the block.
    ///
    /// It's optional because there exists a case where it doesn't need
    /// to be provided for the block to be valid: when EOF is reached.
    /// Otherwise, it will always be set.
    pub closing_segment: Option<Segment<'a>>,
}
