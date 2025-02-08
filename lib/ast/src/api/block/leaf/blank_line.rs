use crate::internal::parse;

/// This struct represents a blank line as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#blank-lines).
///
/// It can be constructed with the [BlankLineParser] parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlankLine<'a>(pub(crate) parse::block::BlankLine<'a>);
