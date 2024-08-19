use std::sync::LazyLock;

use super::{
    builder::{BlockNodeBuilder, BuildResult},
    factory::BuilderFactory,
};

pub struct BlockQuoteBuilder {
    indicator_location: Option<location::Span>,
    child_builder: Option<Box<dyn BlockNodeBuilder>>,
}

impl BlockQuoteBuilder {
    pub fn new() -> Self {
        BlockQuoteBuilder {
            indicator_location: None,
            child_builder: None,
        }
    }
}

static REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(^\s{0,3}>\s?)(.*)$").unwrap());

impl BlockNodeBuilder for BlockQuoteBuilder {
    fn consume_line(&mut self, current_position: location::Position, line: &str) -> BuildResult {
        // TODO: this logic is different for future lines.
        match REGEX.captures(line) {
            Some(captures) => {
                let indicator = captures.get(1).unwrap();
                let indicator_end = location::Position::new(
                    current_position.line,
                    current_position.column + indicator.as_str().chars().count(),
                    current_position.offset + indicator.end(),
                );
                let indicator_location =
                    location::Span::new(current_position, indicator_end.clone());
                self.indicator_location = Some(indicator_location);

                let content = captures.get(2).unwrap();
                self.child_builder = Some(BuilderFactory::find_and_initialize_builder(
                    indicator_end,
                    content.as_str(),
                ));

                BuildResult::Building
            }
            // Supports lazy continuation *only* if the child is a paragraph.
            None => BuildResult::Incompatible,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_build_block_quote_without_prefix() {
        let mut builder = BlockQuoteBuilder::new();
        let line = "> This is a block quote";
        let result = builder.consume_line(location::Position::first(), line);
        assert_eq!(result, BuildResult::Building);
        assert_eq!(
            builder.indicator_location.unwrap(),
            location::Span::new(
                location::Position::first(),
                location::Position::new(1, 3, 2)
            )
        );
    }

    #[test]
    fn should_build_block_quote_without_suffix() {
        let mut builder = BlockQuoteBuilder::new();
        let line = ">This is a block quote";
        let result = builder.consume_line(location::Position::first(), line);
        assert_eq!(result, BuildResult::Building);
        assert_eq!(
            builder.indicator_location.unwrap(),
            location::Span::new(
                location::Position::first(),
                location::Position::new(1, 2, 1)
            )
        );
    }

    #[test]
    fn should_build_block_quote_with_one_space_prefix() {
        let mut builder = BlockQuoteBuilder::new();
        let line = " > This is a block quote";
        let result = builder.consume_line(location::Position::first(), line);
        assert_eq!(result, BuildResult::Building);
        assert_eq!(
            builder.indicator_location.unwrap(),
            location::Span::new(
                location::Position::first(),
                location::Position::new(1, 4, 3)
            )
        );
    }

    #[test]
    fn should_build_block_quote_with_three_spaces_prefix() {
        let mut builder = BlockQuoteBuilder::new();
        let line = "   > This is a block quote";
        let result = builder.consume_line(location::Position::first(), line);
        assert_eq!(result, BuildResult::Building);
        assert_eq!(
            builder.indicator_location.unwrap(),
            location::Span::new(
                location::Position::first(),
                location::Position::new(1, 6, 5)
            )
        );
    }

    #[test]
    fn should_reject_input_that_starts_with_4_spaces() {
        let mut builder = BlockQuoteBuilder::new();
        let line = "    > This is not really a block quote anymore";
        let result = builder.consume_line(location::Position::first(), line);
        assert_eq!(result, BuildResult::Incompatible);
        assert_eq!(builder.indicator_location, None);
    }
}
