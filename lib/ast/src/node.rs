use pulldown_cmark::{Event, OffsetIter, TagEnd};

use crate::{Internal, Leaf, markdown::MarkdownItem};

/// The AST node type.
///
/// It's an enum with 2 variants:
/// - One for [`Internal`] nodes,
/// - One for [`Leaf`] nodes.
///
/// [`Internal`] nodes correspond to [`Event::Start`]. All subsequent events until a matching
/// [`Event::End`] event is found are turned into children nodes.
#[derive(Debug, Clone, PartialEq)]
pub enum Node<'source> {
    /// The leaf variant of nodes. Guaranteed to have no children.
    Leaf(Leaf<'source>),
    /// The internal variat of nodes. Guaranteed to have children.
    Internal(Internal<'source>),
}

impl<'source> Node<'source> {
    pub(crate) fn consume_one(head: MarkdownItem<'source>, iter: &mut OffsetIter<'source>) -> Self {
        match Leaf::try_from(head) {
            Ok(leaf) => Self::Leaf(leaf),
            Err(head) => Self::Internal(
                Internal::try_consume_one(head, iter)
                    // This should work, as it should work for any Event::Start, and the event
                    // should be an Event::Start as this point.
                    .expect("unexpected failure while converting internal"),
            ),
        }
    }

    // TODO: could make a lazy iter instead.
    pub(crate) fn collect_until(until: TagEnd, iter: &mut OffsetIter<'source>) -> Vec<Self> {
        let mut result = vec![];

        while let Some(item) = iter.next() {
            if let Event::End(end) = item.0
                && end == until
            {
                return result;
            }

            result.push(Node::consume_one(item, iter));
        }

        unreachable!(
            "haven't reached expected end {:?} before end of input mfk!!!",
            until
        );
    }
}
