use crate::position::position;
use markdown::mdast::{List, ListItem, Node, Text};
use markdown::unist;

pub struct ListNodeFields {
    pub position: unist::Position,
    pub children: Vec<Node>,
    pub ordered: bool,
    pub spread: bool,
    pub start: Option<u32>,
}

impl Default for ListNodeFields {
    fn default() -> Self {
        Self {
            position: position(()),
            children: vec![],
            ordered: false,
            spread: false,
            start: None,
        }
    }
}

impl From<()> for ListNodeFields {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<&[Node]> for ListNodeFields {
    fn from(children: &[Node]) -> Self {
        Self {
            children: children.to_vec(),
            ..Self::default()
        }
    }
}

pub fn list_node<T: Into<ListNodeFields>>(fields: T) -> Node {
    let fields = fields.into();
    Node::List(List {
        position: Some(fields.position),
        children: fields.children,
        ordered: fields.ordered,
        spread: fields.spread,
        start: fields.start,
    })
}

pub struct ListItemNodeFields {
    pub position: unist::Position,
    pub children: Vec<Node>,
    pub spread: bool,
    pub checked: Option<bool>,
}

impl Default for ListItemNodeFields {
    fn default() -> Self {
        Self {
            position: position(()),
            children: vec![],
            spread: false,
            checked: None,
        }
    }
}

impl<T: Into<String>> From<T> for ListItemNodeFields {
    fn from(text: T) -> Self {
        Self {
            children: vec![Node::Text(Text {
                position: None,
                value: text.into(),
            })],
            ..Default::default()
        }
    }
}

pub fn list_item_node<T: Into<ListItemNodeFields>>(fields: T) -> Node {
    let fields = fields.into();
    Node::ListItem(ListItem {
        position: Some(fields.position),
        children: fields.children,
        spread: fields.spread,
        checked: fields.checked,
    })
}
