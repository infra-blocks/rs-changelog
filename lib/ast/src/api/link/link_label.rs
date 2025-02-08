use segment::Segment;

/// This struct represents a link lable as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#link-label).
///
/// It can be constructed from a [Segment] using the [TryFrom] trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkLabel<'a> {
    pub segment: Segment<'a>,
}

impl<'a> LinkLabel<'a> {
    pub(crate) fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }
}
