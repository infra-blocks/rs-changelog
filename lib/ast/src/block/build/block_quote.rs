use std::sync::LazyLock;

use crate::BlockQuoteNode;

use super::{
    block::BlockNodeBuilder,
    builder::{AddResult, BuilderState, InitResult, NodeBuilder},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockQuoteNodeBuilder {
    location: location::Span,
    children: Vec<Box<BlockNodeBuilder>>,
    state: BuilderState,
}

static REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(^\s{0,3}>\s?)(.*)$").unwrap());

impl NodeBuilder for BlockQuoteNodeBuilder {
    type Node = BlockQuoteNode;

    fn init_from_line(current: location::Position, line: &str) -> InitResult<Self>
    where
        Self: Sized,
    {
        match REGEX.captures(line) {
            Some(captures) => {
                let indicator = captures.get(1).unwrap();
                let indicator_end = location::Position::new(
                    current.line,
                    current.column + indicator.as_str().chars().count(),
                    current.offset + indicator.end(),
                );
                let location = location::Span::new(
                    current,
                    location::Position::new(
                        current.line,
                        current.column + line.chars().count(),
                        current.offset + line.len(),
                    ),
                );

                let content = captures.get(2).unwrap();
                todo!("implement me!");
                /* // TODO: InitResult.unwrap().
                let builder: BlockQuoteBuilder<T> =
                    match T::init_from_line(indicator_end, content.as_str()) {
                        InitResult::Builder(builder) => {
                            Self::new(location, vec![Box::new(builder)])
                        }
                        /// The BlockNodeBuilder is the one builder that cannot return incompatible in practice.
                        InitResult::Incompatible => {
                            panic!("unexpected result from BlockNodeBuilder")
                        }
                    };

                InitResult::Builder(builder) */
            }
            // Supports lazy continuation *only* if the child is a paragraph.
            None => InitResult::Incompatible,
        }
    }

    fn maybe_add_line(&mut self, current: location::Position, line: &str) -> AddResult {
        todo!("implement me!");
    }

    fn state(&self) -> BuilderState {
        self.state
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod single_line_inputs {
        use super::*;

        // TODO: finish those tests: check that the node has a paragraph child and its content.
        #[test]
        fn should_build_block_quote_without_prefix() {
            let line = "> This is a block quote";
            let result = BlockQuoteBuilder::init_from_line(location::Position::first(), line);
            let InitResult::Builder(builder) = result else {
                panic!("unexpected result: {:?}", result);
            };
            assert_eq!(
                builder.indicator_location,
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
            assert_eq!(result, InitResult::Builder);
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
            assert_eq!(result, InitResult::Builder);
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
            assert_eq!(result, InitResult::Builder);
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
            assert_eq!(result, InitResult::Incompatible);
            assert_eq!(builder.indicator_location, None);
        }
    }
}
