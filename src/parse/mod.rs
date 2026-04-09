mod ast;

pub use ast::{Node, Tree};

use pulldown_cmark::Parser;

pub fn parse_ast<'a>(source: &'a str) -> Tree<'a> {
    let parser = Parser::new(source);
    Tree::parse(parser)
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
