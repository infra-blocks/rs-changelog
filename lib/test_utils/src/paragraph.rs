use crate::position::position;
use crate::text::text_as_children;
use markdown::mdast::{Node, Paragraph};
use markdown::unist;

pub struct ParagraphFields {
    pub position: unist::Position,
    pub children: Vec<Node>,
}

impl Default for ParagraphFields {
    fn default() -> Self {
        Self {
            position: position(()),
            children: text_as_children("Ipsum lorem whatever the fuck typically comes after that."),
        }
    }
}

impl<T: Into<String>> From<T> for ParagraphFields {
    fn from(value: T) -> Self {
        Self {
            children: text_as_children(value),
            ..ParagraphFields::default()
        }
    }
}

pub fn paragraph_node<T: Into<ParagraphFields>>(fields: T) -> Node {
    Node::Paragraph(paragraph(fields))
}

pub fn paragraph<T: Into<ParagraphFields>>(fields: T) -> Paragraph {
    let fields = fields.into();
    Paragraph {
        position: Some(fields.position),
        children: fields.children,
    }
}
