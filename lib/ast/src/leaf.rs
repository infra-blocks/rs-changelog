use std::ops::Range;

use pulldown_cmark::{CowStr, Event};

use crate::markdown::MarkdownItem;

/// The internal structure of [`Node::Leaf`] variants.
#[derive(Debug, Clone, PartialEq)]
pub struct Leaf<'source> {
    /// The converted event from [pulldown_cmark].
    pub event: LeafEvent<'source>,
    /// The range, copied directly from the corresponding [`Event::Start`] event of this node.
    pub range: Range<usize>,
}

impl<'source> Leaf<'source> {
    pub fn new(event: LeafEvent<'source>, range: Range<usize>) -> Self {
        Self { event, range }
    }
}

impl<'source> TryFrom<MarkdownItem<'source>> for Leaf<'source> {
    type Error = MarkdownItem<'source>;

    fn try_from(value: MarkdownItem<'source>) -> Result<Self, Self::Error> {
        match LeafEvent::try_from(value.0) {
            Ok(leaf_event) => Ok(Self::new(leaf_event, value.1)),
            Err(event) => Err((event, value.1)),
        }
    }
}

/// A convenient narrowing of [`Event`] that are guaranteed to *not* have children,
/// i.e., non [`Event::Start`] events.
///
/// This type respects the structure of [`pulldown_cmark`]'s own types.
#[derive(Debug, Clone, PartialEq)]
pub enum LeafEvent<'source> {
    Text(CowStr<'source>),
    Code(CowStr<'source>),
    InlineMath(CowStr<'source>),
    DisplayMath(CowStr<'source>),
    InlineHtml(CowStr<'source>),
    Html(CowStr<'source>),
    FootnoteReference(CowStr<'source>),
    SoftBreak,
    HardBreak,
    Rule,
    TaskListMarker(bool),
}

impl<'source> TryFrom<Event<'source>> for LeafEvent<'source> {
    type Error = Event<'source>;

    fn try_from(value: Event<'source>) -> Result<Self, Self::Error> {
        match value {
            Event::Text(text) => Ok(Self::Text(text)),
            Event::Code(text) => Ok(Self::Code(text)),
            Event::InlineMath(text) => Ok(Self::InlineMath(text)),
            Event::DisplayMath(text) => Ok(Self::DisplayMath(text)),
            Event::Html(text) => Ok(Self::Html(text)),
            Event::InlineHtml(text) => Ok(Self::InlineHtml(text)),
            Event::FootnoteReference(text) => Ok(Self::FootnoteReference(text)),
            Event::SoftBreak => Ok(Self::SoftBreak),
            Event::HardBreak => Ok(Self::HardBreak),
            Event::Rule => Ok(Self::Rule),
            Event::TaskListMarker(checked) => Ok(Self::TaskListMarker(checked)),
            // Leaf nodes are, by definition, not those.
            Event::Start(_) | Event::End(_) => Err(value),
        }
    }
}
