use std::{fs, path::Path};

use pulldown_cmark::Parser;

pub fn parse_file<P: AsRef<Path>>(path: P) {
    let content: String =
        fs::read_to_string(&path).expect(&format!("unable to read file: {:?}", path.as_ref()));
    parse_str(&content);
}

pub fn parse_str(content: &str) {
    let parser = Parser::new(content);
    let mut iterator = parser.into_offset_iter();
    for (event, range) in &mut iterator {
        println!("{:?}{:?}", event, range);
    }

    let reference_definitions = iterator.reference_definitions();
    println!("{:?}", reference_definitions);
}
