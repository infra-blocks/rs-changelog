use std::ops::Range;

use changelog_ast::Node;

use crate::parse::releases::Releases;

// TODO: implement ToOwned
#[derive(Debug, Clone, PartialEq)]
pub struct Changelog<'source> {
    pub title: Title<'source>,
    pub releases: Releases,
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
}

impl<'source> TitleHeading<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}
