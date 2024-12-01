use crate::Segment;

/// This struct represents a [Setext](https://spec.commonmark.org/0.31.2/#setext-headings) heading node.
///
/// Unlike most blocks, Setext headings aren't built from a typical parser. The only way they can be constructed
/// is by starting off as a paragraph. Then, the paragraph holder tries to create a setext heading with the
/// [TryFrom] implementation, that possibly morphs the paragraph into a Setext heading upon success.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetextHeading<'a> {
    /// The level of the heading.
    ///
    /// Either 1 or 2, depending on if the underline is `=` or `-`, respectively.
    pub level: u8,
    /// The segments of the Setext heading.
    pub segments: Vec<Segment<'a>>,
}

impl<'a> SetextHeading<'a> {
    /// Constructs a new [SetextHeading] with the given level and segments.
    ///
    /// This is meant to be used by the [TryFrom] implementation.
    pub(crate) fn new(level: u8, segments: Vec<Segment<'a>>) -> Self {
        Self { level, segments }
    }
}
