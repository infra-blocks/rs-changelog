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

    // TODO: could make a lazy iter instead, and collect outside.
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

    pub fn is_internal_that<F: FnOnce(&Internal<'source>) -> bool>(&self, predicate: F) -> bool {
        match self {
            Node::Internal(internal) => predicate(internal),
            _ => false,
        }
    }

    pub fn unwrap_internal(self) -> Internal<'source> {
        match self {
            Node::Internal(internal) => internal,
            _ => panic!("cannot unwrap internal on {:?}", self),
        }
    }

    pub fn unwrap_leaf(self) -> Leaf<'source> {
        match self {
            Node::Leaf(leaf) => leaf,
            _ => panic!("cannot unwrap leaf on {:?}", self),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod unwrap_internal {
        use crate::{InternalEvent, LeafEvent};

        use super::*;

        #[test]
        #[should_panic]
        fn should_fail_with_leaf() {
            Node::Leaf(Leaf::new(LeafEvent::SoftBreak, Default::default())).unwrap_internal();
        }

        #[test]
        fn should_work_with_internal() {
            let internal = Internal::new(
                InternalEvent::Paragraph,
                Default::default(),
                Default::default(),
            );
            let node = Node::Internal(internal.clone());
            assert_eq!(internal, node.unwrap_internal());
        }
    }

    mod unwrap_leaf {
        use crate::{InternalEvent, LeafEvent};

        use super::*;

        #[test]
        #[should_panic]
        fn should_fail_with_internal() {
            Node::Internal(Internal::new(
                InternalEvent::Paragraph,
                Default::default(),
                Default::default(),
            ))
            .unwrap_leaf();
        }

        #[test]
        fn should_work_with_leaf() {
            let leaf = Leaf::new(LeafEvent::SoftBreak, Default::default());
            let node = Node::Leaf(leaf.clone());
            assert_eq!(leaf, node.unwrap_leaf());
        }
    }
}
