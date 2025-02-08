use segment::LineSegment;

use crate::internal::parse;

/// This struct represents a thematic brak as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#thematic-breaks).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreak<'a>(pub(crate) parse::block::ThematicBreak<'a>);

impl<'a> ThematicBreak<'a> {
    /// Returns the line segment associated with this thematic break.
    pub fn segment(&self) -> LineSegment<'a> {
        self.0 .0.into()
    }
}
