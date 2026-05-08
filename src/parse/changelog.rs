use std::ops::Range;

use changelog_ast::Node;

use crate::parse::releases::{Releases, Unreleased};

// TODO: implement ToOwned
#[derive(Debug, Clone, PartialEq)]
pub struct Changelog<'source> {
    pub title: Title<'source>,
    /// The unreleased section of a document is optional, as it would basically become empty
    /// after each release. So, whether the user decides to have one or not, is up to them.
    pub unreleased: Option<Unreleased>,
    pub releases: Releases,
}

// TODO: move into its own fucking file.
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
