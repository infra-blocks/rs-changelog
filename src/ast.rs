use changelog_ast::{AstIterator, Node};

pub struct Ast<'source> {
    pub nodes: Vec<Node<'source>>,
}

/// Parses the provided text and constructs a navigatable markdown Ast
/// based on pulldown_cmark data types.
pub fn parse_ast(source: &str) -> Ast<'_> {
    Ast {
        nodes: AstIterator::new(source).collect(),
    }
}
