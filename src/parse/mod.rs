mod error;
mod invalid_heading;

use std::{borrow::Borrow, ops::Range};

use pulldown_cmark::{OffsetIter, Parser, TextMergeWithOffset};

pub fn parse<'a>(source: &'a str) -> Markdown<'a> {
    let parser = Parser::new(source);
    let mut iterator = parser.into_offset_iter();
    for value in &mut iterator {}

    let reference_definitions = iterator.reference_definitions();

    Markdown { source }
}

pub fn debug<'a>(source: &'a str) {
    let parser = Parser::new(source);
    let mut iterator = parser.into_offset_iter();
    for value in &mut iterator {
        println!("{:?}", value);
    }

    let reference_definitions = iterator.reference_definitions();
    println!("{:?}", reference_definitions);
}

/// The AST representation of the markdown document.
pub struct Markdown<'a> {
    pub source: &'a str,
}

pub enum Block {
    Text(Range<usize>),
}

impl Block {
    fn parse(iter: &mut OffsetIter) -> Self {
        let (event, range) = iter.next().expect("bug in code esé?");
        match event {
            pulldown_cmark::Event::Start(tag) => todo!(),
            pulldown_cmark::Event::End(tag_end) => panic!("unexpected tag ending {:?}", tag_end),
            pulldown_cmark::Event::Text(_) => Block::Text(range),
            pulldown_cmark::Event::Code(_) => Block::Text(range),
            pulldown_cmark::Event::InlineMath(_) => Block::Text(range),
            pulldown_cmark::Event::DisplayMath(_) => Block::Text(range),
            pulldown_cmark::Event::Html(_) => Block::Text(range),
            pulldown_cmark::Event::InlineHtml(_) => Block::Text(range),
            pulldown_cmark::Event::FootnoteReference(_) => Block::Text(range),
            pulldown_cmark::Event::SoftBreak => Block::Text(range),
            pulldown_cmark::Event::HardBreak => Block::Text(range),
            pulldown_cmark::Event::Rule => Block::Text(range),
            pulldown_cmark::Event::TaskListMarker(_) => Block::Text(range),
        }
    }
}
