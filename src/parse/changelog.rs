use std::ops::Range;

use changelog_ast::{Heading, Internal, InternalEvent, Node};
use pulldown_cmark::{CowStr, HeadingLevel};

// TODO: implement ToOwned
#[derive(Debug, Clone, PartialEq)]
pub struct Changelog<'source> {
    pub title: Option<Title<'source>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Title<'source> {
    // TODO: maybe would be more useful to have a single HeadingNode with the event & range & children
    // TODO: attempt the refactor after having tests.
    heading: TitleHeading<'source>,
    text: Vec<Node<'source>>,
}

impl<'source> Title<'source> {
    pub(crate) fn new(heading: TitleHeading<'source>, text: Vec<Node<'source>>) -> Self {
        Self { heading, text }
    }
}

// This is guaranteed to be a heading of level 1.
#[derive(Debug, Clone, PartialEq)]
pub struct TitleHeading<'source> {
    pub children: Vec<Node<'source>>,
    pub range: Range<usize>,
    // TODO: investigate if those fields are relevant, useful in the context of changelog parsing.
    pub id: Option<CowStr<'source>>,
    pub classes: Vec<CowStr<'source>>,
    /// The first item of the tuple is the attr and second one the value.
    pub attrs: Vec<(CowStr<'source>, Option<CowStr<'source>>)>,
}

impl<'source> TitleHeading<'source> {
    pub fn is_title_heading(node: &Node<'_>) -> bool {
        matches!(
            node,
            Node::Internal(Internal {
                event: InternalEvent::Heading(Heading {
                    level: HeadingLevel::H1,
                    id: _,
                    classes: _,
                    attrs: _,
                }),
                range: _,
                children: _,
            })
        )
    }
}
