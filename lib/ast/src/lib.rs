mod markdown;
mod node;
mod node_children_iterator;

pub use node::*;
pub use node_children_iterator::*;

// Useful reexports.
pub use pulldown_cmark::{
    Alignment, BlockQuoteKind, CodeBlockKind, CowStr, HeadingLevel, LinkDef, LinkType,
    MetadataBlockKind, RefDefs,
};
use pulldown_cmark::{OffsetIter, Parser};

/// Main data structure exported by this crate.
///
/// It is an iterator built on top of the ones provided by [pulldown_cmark]. Unlike the latter,
/// it does not produce events but [Node]s. The main difference between the two is the hierarchical
/// structure the [Node]'s offer. [Node]s have children, [pulldown_cmark]'s event don't.
///
/// The iterator resolves branches eagerly, and yields one branch at a time, with the returned [Node]
/// being the root of the branch.
///
/// The algorithm is quite simple: every [pulldown_cmark::Event::Start] is turned into an internal node,
/// every following event is treated as a child of the node. The final matching [pulldown_cmark::Event::End] signifies
/// the node has been constructed (the event itself is dropped). Every other [pulldown_cmark]
/// event results in a leaf variant of [Node].
pub struct AstIterator<'source> {
    inner: OffsetIter<'source>,
}

impl<'source> AstIterator<'source> {
    pub fn new(source: &'source str) -> Self {
        let parser = Parser::new(source);
        Self::with_inner(parser.into_offset_iter())
    }

    pub fn reference_definitions(&self) -> &RefDefs<'source> {
        self.inner.reference_definitions()
    }

    fn with_inner(inner: OffsetIter<'source>) -> Self {
        Self { inner }
    }
}

impl<'source> Iterator for AstIterator<'source> {
    type Item = Node<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(head) => Some(Node::consume_one(head, &mut self.inner)),
            None => None,
        }
    }
}
