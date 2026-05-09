use std::collections::HashMap;

use changelog_ast::{AstIterator, LinkDef, Node};

pub struct Ast<'source> {
    pub nodes: Vec<Node<'source>>,
    pub reference_definitions: HashMap<String, LinkDef<'source>>,
}

/// Parses the provided text and constructs a navigatable markdown Ast
/// based on pulldown_cmark data types.
pub fn parse_ast(source: &str) -> Ast<'_> {
    let mut iter = AstIterator::new(source);
    let nodes = iter.by_ref().collect();
    let reference_definitions = iter
        .reference_definitions()
        .iter()
        .map(|(k, v)| (k.to_owned(), v.clone()))
        .collect();

    Ast {
        nodes,
        reference_definitions,
    }
}
