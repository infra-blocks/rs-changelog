use pulldown_cmark::Parser;

/// Prints out all the events, sequentially, as read by pulldown_cmark.
pub fn debug(source: &str) {
    let parser = Parser::new(source);
    let mut iterator = parser.into_offset_iter();
    for value in &mut iterator {
        println!("{:?}", value);
    }

    let reference_definitions = iterator.reference_definitions();
    println!("{:?}", reference_definitions);
}
