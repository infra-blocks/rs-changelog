use crate::internal::parse;

// TODO: on display, trim the last segment of trailing whitespace.

/// This struct represents a paragraph as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#paragraphs).
///
/// It can be constructed with the [ParagraphParser].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph<'a>(pub(crate) parse::block::Paragraph<'a>);
