use crate::position;
use markdown::mdast::{Node, Text};

pub fn text_as_children<T: Into<String>>(text: T) -> Vec<Node> {
    vec![Node::Text(Text {
        position: Some(position::position(())),
        value: text.into(),
    })]
}
