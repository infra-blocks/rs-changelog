use crate::position::position;
use crate::text::text_as_children;
use markdown::mdast::Node;
use markdown::unist;

pub struct ParagraphNodeFields {
    pub position: unist::Position,
    pub children: Vec<Node>,
}

impl Default for ParagraphNodeFields {
    fn default() -> Self {
        Self {
            position: position(()),
            children: text_as_children("Ipsum lorem whatever the fuck typically comes after that."),
        }
    }
}

impl<T: Into<String>> From<T> for ParagraphNodeFields {
    fn from(value: T) -> Self {
        Self {
            children: text_as_children(value),
            ..ParagraphNodeFields::default()
        }
    }
}

pub fn paragraph_node<T: Into<ParagraphNodeFields>>(fields: T) -> Node {
    let fields = fields.into();
    Node::Paragraph(markdown::mdast::Paragraph {
        position: Some(fields.position),
        children: fields.children,
    })
}
