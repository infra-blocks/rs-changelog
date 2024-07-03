use crate::{position, text};
use markdown::mdast::Node;
use markdown::unist;

#[derive(Debug)]
pub struct HeadingNodeFields {
    pub position: unist::Position,
    pub depth: u8,
    pub children: Vec<Node>,
}

impl Default for HeadingNodeFields {
    fn default() -> Self {
        Self {
            position: position::position(()),
            depth: 1,
            children: text::text_as_children("Some Heading"),
        }
    }
}

impl From<u8> for HeadingNodeFields {
    fn from(depth: u8) -> Self {
        HeadingNodeFields {
            depth,
            ..Default::default()
        }
    }
}

impl<T: Into<String>> From<(u8, T)> for HeadingNodeFields {
    fn from(stuff: (u8, T)) -> Self {
        HeadingNodeFields {
            depth: stuff.0,
            children: text::text_as_children(stuff.1.into()),
            ..Default::default()
        }
    }
}

pub fn heading_node<T: Into<HeadingNodeFields>>(fields: T) -> Node {
    let fields = fields.into();
    Node::Heading(markdown::mdast::Heading {
        depth: fields.depth,
        position: Some(fields.position),
        children: fields.children,
    })
}
