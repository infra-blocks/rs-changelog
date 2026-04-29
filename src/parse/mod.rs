use changelog_ast::{AstIterator, Node};
use pulldown_cmark::Parser;

pub struct Ast<'source> {
    pub nodes: Vec<Node<'source>>,
}

pub fn parse_ast(source: &str) -> Ast<'_> {
    Ast {
        nodes: AstIterator::new(source).collect(),
    }
}

pub fn debug(source: &str) {
    let parser = Parser::new(source);
    let mut iterator = parser.into_offset_iter();
    for value in &mut iterator {
        println!("{:?}", value);
    }

    let reference_definitions = iterator.reference_definitions();
    println!("{:?}", reference_definitions);
}
