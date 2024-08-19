use crate::{BlockNode, Document};

pub enum Error {}

// TODO: move this in structure with Document.from_str()
/// Parses Markdown into an AST, using the strategy defined [here](https://spec.commonmark.org/0.31.2/#appendix-a-parsing-strategy).
pub fn parse(markdown: &str) -> Result<Document, Error> {
    Parser::new().parse(markdown)
}

/// TODO: requires block listeners on characters.
pub struct Parser {
    position: location::Position,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            position: location::Position::first(),
        }
    }

    pub fn parse(&mut self, markdown: &str) -> Result<Document, Error> {
        // 1. Parse the input into blocks.
        // 2. Visit the block tree and parse the inline elements.
        Ok(Document { blocks: vec![] })
    }
}
