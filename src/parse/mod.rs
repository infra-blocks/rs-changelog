use changelog_ast::Ast;
use pulldown_cmark::Parser;

pub fn parse_ast(source: &str) -> Ast<'_> {
    Ast::parse(source)
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
