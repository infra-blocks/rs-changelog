mod node;

pub use node::*;
use pulldown_cmark::Parser;

/// This module contains the logic required to build the AST from the pulldown-cmark
/// iterator.

#[derive(Debug, Clone, PartialEq)]
pub struct Tree<'a> {
    //TODO: link reference definitions?
    pub branches: Vec<Node<'a>>,
}

impl<'a> Tree<'a> {
    pub fn parse(source: &'a str) -> Self {
        let parser = Parser::new(source);
        let mut iter = parser.into_offset_iter().peekable();
        let mut branches = Vec::new();
        while let Some(value) = Node::parse_node(&mut iter) {
            branches.push(value)
        }
        Tree { branches }
    }
}
