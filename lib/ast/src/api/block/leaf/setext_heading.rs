use crate::internal::parse;

/// This struct represents a [Setext](https://spec.commonmark.org/0.31.2/#setext-headings) heading node.
///
/// Unlike most blocks, Setext headings aren't built from a typical parser. The only way they can be constructed
/// is by starting off as a paragraph. Then, the paragraph holder tries to create a setext heading with the
/// [TryFrom] implementation, that possibly morphs the paragraph into a Setext heading upon success.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetextHeading<'a>(pub(crate) parse::block::SetextHeading<'a>);
