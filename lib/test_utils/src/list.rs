use crate::position::position;
use markdown::mdast::{List, ListItem, Node, Text};
use markdown::unist;

pub struct ListFields {
    pub position: unist::Position,
    pub children: Vec<Node>,
    pub ordered: bool,
    pub spread: bool,
    pub start: Option<u32>,
}

impl Default for ListFields {
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

impl From<()> for ListFields {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<&[Node]> for ListFields {
    fn from(children: &[Node]) -> Self {
        Self {
            children: children.to_vec(),
            ..Self::default()
        }
    }
}

pub fn list_node<T: Into<ListFields>>(fields: T) -> Node {
    Node::List(list(fields))
}

pub fn list<T: Into<ListFields>>(fields: T) -> List {
    let fields = fields.into();
    List {
        position: Some(fields.position),
        children: fields.children,
        ordered: fields.ordered,
        spread: fields.spread,
        start: fields.start,
    }
}

pub struct ListItemFields {
    pub position: unist::Position,
    pub children: Vec<Node>,
    pub spread: bool,
    pub checked: Option<bool>,
}

impl Default for ListItemFields {
    fn default() -> Self {
        Self {
            position: position(()),
            children: vec![],
            spread: false,
            checked: None,
        }
    }
}

impl<T: Into<String>> From<T> for ListItemFields {
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

pub fn list_item_node<T: Into<ListItemFields>>(fields: T) -> Node {
    Node::ListItem(list_item(fields))
}

pub fn list_item<T: Into<ListItemFields>>(fields: T) -> ListItem {
    let fields = fields.into();
    ListItem {
        position: Some(fields.position),
        children: fields.children,
        spread: fields.spread,
        checked: fields.checked,
    }
}
