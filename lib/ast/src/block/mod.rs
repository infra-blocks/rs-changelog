mod block_quote;
mod builder;
mod node;
mod paragraph;

use std::sync::LazyLock;

use node::{AddResult, InitResult, Node, NodeBuilder};

use crate::InlineNode;

/// Implementation of a Markdown Block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Container(Container),
    Leaf(Leaf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockNode {
    Container(ContainerNode),
    Leaf(LeafNode),
}

impl BlockNode {
    pub fn paragraph(location: location::Span, children: Vec<InlineNode>) -> Self {
        BlockNode::Leaf(LeafNode::paragraph(location, children))
    }
}

impl Node for BlockNode {
    type Builder = BlockNodeBuilder;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BlockNodeBuilder {
    Container(ContainerNodeBuilder),
    Leaf(LeafNodeBuilder),
}

impl NodeBuilder for BlockNodeBuilder {
    type Node = BlockNode;

    fn init_from_line(current: location::Position, line: &str) -> InitResult<Self>
    where
        Self: Sized,
    {
        match ContainerNodeBuilder::init_from_line(current, line) {
            InitResult::Finished(node) => InitResult::Finished(BlockNode::Container(node)),
            InitResult::Building(builder) => {
                InitResult::Building(BlockNodeBuilder::Container(builder))
            }
            InitResult::Incompatible => match LeafNodeBuilder::init_from_line(current, line) {
                InitResult::Finished(node) => InitResult::Finished(BlockNode::Leaf(node)),
                InitResult::Building(builder) => {
                    InitResult::Building(BlockNodeBuilder::Leaf(builder))
                }
                InitResult::Incompatible => InitResult::Incompatible,
            },
        }
    }

    fn maybe_add_line(&mut self, current: location::Position, line: &str) -> AddResult {
        match self {
            BlockNodeBuilder::Container(builder) => builder.maybe_add_line(current, line),
            BlockNodeBuilder::Leaf(builder) => builder.maybe_add_line(current, line),
        }
    }

    fn finish(self) -> Self::Node {
        match self {
            BlockNodeBuilder::Container(builder) => BlockNode::Container(builder.finish()),
            BlockNodeBuilder::Leaf(builder) => BlockNode::Leaf(builder.finish()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerNode {
    BlockQuote(BlockQuoteNode),
    List,
    ListItem,
}

impl Node for ContainerNode {
    type Builder = ContainerNodeBuilder;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ContainerNodeBuilder {
    BlockQuote(BlockQuoteBuilder),
    List,
    ListItem,
}

impl NodeBuilder for ContainerNodeBuilder {
    type Node = ContainerNode;

    fn init_from_line(current: location::Position, line: &str) -> InitResult<Self>
    where
        Self: Sized,
    {
        match BlockQuoteBuilder::init_from_line(current, line) {
            InitResult::Building(builder) => {
                InitResult::Building(ContainerNodeBuilder::BlockQuote(builder))
            }
            InitResult::Incompatible => todo!("implement me!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockQuoteNode {
    location: location::Span,
    children: Vec<BlockNode>,
}

impl BlockQuoteNode {
    pub fn new(location: location::Span, children: Vec<BlockNode>) -> Self {
        BlockQuoteNode { location, children }
    }
}

impl Node for BlockQuoteNode {
    type Builder = BlockQuoteBuilder;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockQuoteBuilder {
    location: location::Span,
    indicator_location: location::Span,
    chidlren: Vec<BlockNode>,
    child_builder: Box<BlockNodeBuilder>,
}

static REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(^\s{0,3}>\s?)(.*)$").unwrap());

impl NodeBuilder for BlockQuoteBuilder {
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
                let indicator_location = location::Span::new(current, indicator_end.clone());

                let content = captures.get(2).unwrap();
                let mut child_builder;
                let mut children = Vec::new();
                match BlockeNodeBuilder::init_from_line(indicator_end, content.as_str()) {
                    InitResult::Building(builder) => InitResult::Building(Self {
                        indicator_location,
                        child_builder: Box::new(builder),
                    }),
                    /// The BlockNodeBuilder is the one builder that cannot return incompatible in practice.
                    InitResult::Incompatible => panic!("unexpected result: {:?}", result),
                }

                InitResult::Building(Self {
                    indicator_location,
                    child_builder,
                })
            }
            // Supports lazy continuation *only* if the child is a paragraph.
            None => InitResult::Incompatible,
        }
    }

    fn maybe_add_line(&mut self, current: location::Position, line: &str) -> AddResult {
        todo!("implement me!");
    }

    fn finish(self) -> BlockNode {
        todo!("implement me!");
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Leaf {
    ThematicBreak,
    AtxHeading,
    SetextHeading,
    IndentedCode,
    FencedCode,
    Html,
    LinkReferenceDefinition,
    Paragraph,
    BlankLine,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafNode {
    kind: Leaf,
    location: location::Span,
    children: Vec<InlineNode>,
}

impl LeafNode {
    pub fn new(kind: Leaf, location: location::Span, children: Vec<InlineNode>) -> Self {
        LeafNode {
            kind,
            location,
            children,
        }
    }

    pub fn paragraph(location: location::Span, children: Vec<InlineNode>) -> Self {
        LeafNode::new(Leaf::Paragraph, location, children)
    }
}

impl Node for LeafNode {
    type Builder = LeafNodeBuilder;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LeafNodeBuilder {}

impl NodeBuilder for LeafNodeBuilder {
    type Node = LeafNode;
}

#[cfg(test)]
mod test {
    use super::*;

    mod block_quote {
        use super::*;

        mod single_line_inputs {
            use super::*;

            // TODO: finish those tests: check that the node has a paragraph child and its content.
            #[test]
            fn should_build_block_quote_without_prefix() {
                let line = "> This is a block quote";
                let result = BlockQuoteBuilder::init_from_line(location::Position::first(), line);
                let InitResult::Building(builder) = result else {
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
                assert_eq!(result, InitResult::Building);
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
                assert_eq!(result, InitResult::Building);
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
                assert_eq!(result, InitResult::Building);
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
}
