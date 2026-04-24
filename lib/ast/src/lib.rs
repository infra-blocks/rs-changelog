mod internal;
mod leaf;
mod markdown;
mod node;

pub use internal::*;
pub use leaf::*;
pub use node::*;

use pulldown_cmark::{OffsetIter, Parser};

/// Main data structure exported by this crate.
///
/// It is obtained using [`Ast::parse`] on a `&str`. It leverages [`pulldown_cmark`] to parse
/// the markdown and construct the Ast.
///
/// The algorithm is quite simple: every [`Event::Start`] is turned into an [`Node::Internal`] node,
/// every [`Event::End`] is dropped, and every other event is transformed into a [`Node::Leaf`] variant.
#[derive(Debug, Clone, PartialEq)]
pub struct Ast<'source> {
    /// Top-level nodes.
    pub nodes: Vec<Node<'source>>,
}

impl<'source> Ast<'source> {
    pub fn parse(source: &'source str) -> Self {
        let parser = Parser::new(source);
        let iter = NodesIterator::new(parser.into_offset_iter());
        Ast {
            nodes: iter.collect(),
        }
    }
}

struct NodesIterator<'source> {
    inner: OffsetIter<'source>,
}

impl<'source> NodesIterator<'source> {
    pub fn new(inner: OffsetIter<'source>) -> Self {
        Self { inner }
    }
}

impl<'source> From<OffsetIter<'source>> for NodesIterator<'source> {
    fn from(value: OffsetIter<'source>) -> Self {
        Self::new(value)
    }
}

impl<'source> Iterator for NodesIterator<'source> {
    type Item = Node<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(head) => Some(Node::consume_one(head, &mut self.inner)),
            None => None,
        }
    }
}
