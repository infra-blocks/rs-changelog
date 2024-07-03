use crate::position;
use markdown::mdast::Node;
use markdown::unist;

#[derive(Debug)]
pub struct DefinitionFields {
    pub position: unist::Position,
    pub identifier: String,
    pub url: String,
}

impl Default for DefinitionFields {
    fn default() -> Self {
        Self {
            position: position::position(()),
            identifier: "".to_string(),
            url: "".to_string(),
        }
    }
}

impl<L: Into<String>, R: Into<String>> From<(L, R)> for DefinitionFields {
    fn from(stuff: (L, R)) -> Self {
        DefinitionFields {
            identifier: stuff.0.into(),
            url: stuff.1.into(),
            ..Default::default()
        }
    }
}

pub fn definition_node<T: Into<DefinitionFields>>(fields: T) -> Node {
    let fields = fields.into();
    Node::Definition(markdown::mdast::Definition {
        identifier: fields.identifier,
        url: fields.url,
        position: Some(fields.position),
        title: None,
        label: None,
    })
}
