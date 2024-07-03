use markdown::mdast::{Definition, Heading, List, ListItem, Node, Paragraph};

use crate::markdown::node_kind::MarkdownNodeKind;
use crate::{ParseError, Position};

// TODO: define its own error type.
pub trait NodeUtils {
    fn unwrap_position(&self) -> Position;
    fn children_text(&self) -> String;
    // TODO: doesn't have to be on the node type. Might be fucked up this way actually.
    fn validate_heading_with_depth(&self, expected_depth: u8) -> Result<&Heading, ParseError>;
    fn validate_list(&self) -> Result<&List, ParseError>;
    fn validate_list_item(&self) -> Result<&ListItem, ParseError>;
    fn validate_paragraph(&self) -> Result<&Paragraph, ParseError>;
    fn validate_definition(&self) -> Result<&Definition, ParseError>;
}

impl NodeUtils for Node {
    fn unwrap_position(&self) -> Position {
        self.position().unwrap().into()
    }

    fn children_text(&self) -> String {
        match self.children() {
            Some(vec) => vec.iter().map(ToString::to_string).collect(),
            None => "".to_string(),
        }
    }

    fn validate_heading_with_depth(&self, expected_depth: u8) -> Result<&Heading, ParseError> {
        match self {
            Node::Heading(heading) => {
                if heading.depth != expected_depth {
                    Err(ParseError::invalid_node(format!(
                        "found heading with depth {}, but expected a depth of {}",
                        heading.depth, expected_depth
                    )))
                } else {
                    Ok(heading)
                }
            }
            _ => Err(ParseError::invalid_node_type(
                MarkdownNodeKind::Heading,
                self,
            )),
        }
    }

    fn validate_list(&self) -> Result<&List, ParseError> {
        match self {
            Node::List(list) => Ok(list),
            _ => Err(ParseError::invalid_node_type(MarkdownNodeKind::List, self)),
        }
    }

    fn validate_list_item(&self) -> Result<&ListItem, ParseError> {
        match self {
            Node::ListItem(list_item) => Ok(list_item),
            _ => Err(ParseError::invalid_node_type(
                MarkdownNodeKind::ListItem,
                self,
            )),
        }
    }

    fn validate_paragraph(&self) -> Result<&Paragraph, ParseError> {
        match self {
            Node::Paragraph(paragraph) => Ok(paragraph),
            _ => Err(ParseError::invalid_node_type(
                MarkdownNodeKind::Paragraph,
                self,
            )),
        }
    }

    fn validate_definition(&self) -> Result<&Definition, ParseError> {
        match self {
            Node::Definition(definition) => Ok(definition),
            _ => Err(ParseError::invalid_node_type(
                MarkdownNodeKind::Definition,
                self,
            )),
        }
    }
}
