use crate::Segment;

/// This struct represents a thematic brak as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#thematic-breaks).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreak<'a> {
    /// The text segment forming the thematic break.
    pub segment: Segment<'a>,
}
