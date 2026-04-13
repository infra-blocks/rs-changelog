use std::{iter::Peekable, ops::Range};

use pulldown_cmark::{Event, OffsetIter, TagEnd};

#[derive(Debug, Clone, PartialEq)]
pub struct Node<'a> {
    pub event: Event<'a>,
    pub range: Range<usize>,
    pub children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(event: Event<'a>, range: Range<usize>, children: Vec<Node<'a>>) -> Self {
        Node {
            event,
            range,
            children,
        }
    }

    fn new_childless(event: Event<'a>, range: Range<usize>) -> Self {
        Self::new(event, range, vec![])
    }

    pub(super) fn parse_node(iter: &mut Peekable<OffsetIter<'a>>) -> Option<Self> {
        if let Some(value) = iter.next() {
            match value.0 {
                Event::Start(ref tag) => {
                    let children = parse_children(iter, tag.to_end());
                    let node = Node::new(value.0, value.1, children);
                    Some(node)
                }
                // Tag endings should always match a start tag, which should always dispatch to a parse_children.
                Event::End(tag_end) => panic!("unexpected tag ending {:?} while parsing", tag_end),
                _ => Some(Self::new_childless(value.0, value.1)),
            }
        } else {
            None
        }
    }
}

fn parse_children<'a>(iter: &mut Peekable<OffsetIter<'a>>, until: TagEnd) -> Vec<Node<'a>> {
    let mut nodes = vec![];
    while let Some(value) = iter.peek() {
        match value.0 {
            Event::End(tag_end) => {
                assert!(
                    tag_end == until,
                    "invalid tag end while parsing children: expected {:?}, got {:?}",
                    until,
                    tag_end
                );
                iter.next();
                return nodes;
            }
            _ => nodes.push(Node::parse_node(iter).unwrap_or_else(|| {
                panic!("unexpected end of input reached before tag end {:?}", until)
            })),
        }
    }
    panic!(
        "unexpected end of input while parsing children until {:?}",
        until
    );
}
