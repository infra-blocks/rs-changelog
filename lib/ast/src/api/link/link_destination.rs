use crate::Segment;

/// This struct represents a link destination as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#link-destination).
///
/// It can be constructed from a [Segment] using the [TryFrom] trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkDestination<'a> {
    segment: Segment<'a>,
}

impl<'a> LinkDestination<'a> {
    pub(crate) fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }
}
