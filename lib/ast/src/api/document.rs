use std::fmt::Formatter;

use crate::{
    block::{Block, Leaf, LinkReferenceDefinition},
    internal::{parse::Blocks, render::DisplayHtml},
    StrExt, ToHtml,
};

pub struct Document<'a> {
    /// Without the link reference definitions?
    pub blocks: Vec<Block<'a>>,
    pub link_reference_definitions: Vec<LinkReferenceDefinition<'a>>,
}

impl<'a> Document<'a> {
    fn new(
        blocks: Vec<Block<'a>>,
        link_reference_definitions: Vec<LinkReferenceDefinition<'a>>,
    ) -> Self {
        Self {
            blocks,
            link_reference_definitions,
        }
    }

    pub fn parse(input: &'a str) -> Self {
        let blocks = input.line_segments().blocks();
        Self::from(blocks)
    }
}

impl<'a, I> From<I> for Document<'a>
where
    I: IntoIterator<Item = Block<'a>>,
{
    fn from(blocks: I) -> Self {
        let mut without_link_reference_definitions = Vec::new();
        let mut link_reference_definitions = Vec::new();

        for block in blocks {
            match block {
                Block::Leaf(leaf) => {
                    if let Leaf::LinkReferenceDefinition(definition) = leaf {
                        link_reference_definitions.push(definition);
                    } else {
                        without_link_reference_definitions.push(leaf.into());
                    }
                }
                _ => without_link_reference_definitions.push(block),
            }
        }

        Self::new(
            without_link_reference_definitions,
            link_reference_definitions,
        )
    }
}

impl<'a> ToHtml for Document<'a> {
    fn to_html(&self) -> String {
        let mut buffer = String::new();
        self.display_html(&mut buffer, &self.link_reference_definitions);
        buffer
    }
}
