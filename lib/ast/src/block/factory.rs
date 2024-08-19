use super::{
    block_quote::BlockQuoteBuilder,
    builder::{BlockNodeBuilder, BuildResult},
    paragraph::ParagraphBuilder,
};

pub struct BuilderFactory {}

impl BuilderFactory {
    // TODO: return the node immediately if the builder could. This is only appropriate for leaf nodes.
    pub fn find_and_initialize_builder(
        current: location::Position,
        line: &str,
    ) -> Box<dyn BlockNodeBuilder> {
        if let Some(block_quote_builder) = BuilderFactory::try_block_quote(current, line) {
            return Box::new(block_quote_builder);
        }
        if let Some(paragraph_builder) = BuilderFactory::try_paragraph(current, line) {
            return Box::new(paragraph_builder);
        }
        panic!("missing builder for line: {}", line);
    }

    fn try_block_quote(current: location::Position, line: &str) -> Option<BlockQuoteBuilder> {
        let mut builder = BlockQuoteBuilder::new();
        match builder.consume_line(current, line) {
            BuildResult::Building => Some(builder),
            _ => None,
        }
    }

    fn try_paragraph(current: location::Position, line: &str) -> Option<ParagraphBuilder> {
        let mut builder = ParagraphBuilder::new();
        match builder.consume_line(current, line) {
            BuildResult::Building => Some(builder),
            _ => None,
        }
    }
}
