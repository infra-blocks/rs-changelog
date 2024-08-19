use crate::{position, text};
use markdown::{
    mdast::{Heading, Node},
    unist::Position,
};

#[derive(Debug)]
pub struct HeadingFields {
    pub position: Position,
    pub depth: u8,
    pub children: Vec<Node>,
}

impl Default for HeadingFields {
    fn default() -> Self {
        Self {
            position: position::position(()),
            depth: 1,
            children: text::text_as_children("Some Heading"),
        }
    }
}

impl From<u8> for HeadingFields {
    fn from(depth: u8) -> Self {
        HeadingFields {
            depth,
            ..Default::default()
        }
    }
}

impl<T: Into<String>> From<(u8, T)> for HeadingFields {
    fn from(stuff: (u8, T)) -> Self {
        HeadingFields {
            depth: stuff.0,
            children: text::text_as_children(stuff.1.into()),
            ..Default::default()
        }
    }
}

pub fn heading_node<T: Into<HeadingFields>>(fields: T) -> Node {
    Node::Heading(heading(fields))
}

pub fn heading<T: Into<HeadingFields>>(fields: T) -> Heading {
    let fields = fields.into();
    Heading {
        depth: fields.depth,
        position: Some(fields.position),
        children: fields.children,
    }
}
