use segment::Segment;

/// This enum identifies the possible kinds of link titles.
///
/// It is obtained by parsing the first segment of a link title using a [LinkTitleParser].
/// Once an instance is created, it is used to validate subsequent segments of
/// the same kind until a closing symbol is found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkTitleKind {
    DoubleQuoted,
    SingleQuoted,
    Parenthesized,
}

/// This struct represents a link title as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#link-title).
///
/// It can be constructed using a [LinkTitleParser].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkTitle<'a> {
    /// The kind of link title.
    kind: LinkTitleKind,
    /// The segments forming the link title.
    segments: Vec<Segment<'a>>,
}

impl<'a> LinkTitle<'a> {
    pub(crate) fn new(kind: LinkTitleKind, segments: Vec<Segment<'a>>) -> Self {
        Self { kind, segments }
    }
}
