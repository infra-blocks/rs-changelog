use crate::Segment;

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
pub struct IndentedCode<'a> {
    /// The segments forming the block.
    pub segments: Vec<Segment<'a>>,
}
