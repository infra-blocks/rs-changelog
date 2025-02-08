mod leaf;
pub use leaf::*;
use segment::LineSegment;

use std::convert::Infallible;

use super::parser::{Finalize, Ingest, IngestResult};

// TODO: a trait to revert back into segments. IntoSegments or something.
// For all blocks to implement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block<'a> {
    Leaf(Leaf<'a>),
}

impl<'a> From<Leaf<'a>> for Block<'a> {
    fn from(value: Leaf<'a>) -> Self {
        Self::Leaf(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BlockParser<'a> {
    Leaf(LeafParser<'a>),
}

impl<'a> Default for BlockParser<'a> {
    fn default() -> Self {
        Self::Leaf(LeafParser::default())
    }
}

impl<'a> BlockParser<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

pub(super) enum IngestSuccess<'a> {
    Single(Block<'a>),
    Multiple(Vec<Block<'a>>),
}

impl<'a> Ingest for BlockParser<'a> {
    type Input = LineSegment<'a>;
    type Ready = Self;
    type Success = IngestSuccess<'a>;
    type Failure = Infallible;

    fn ingest(self, input: Self::Input) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        // TODO: try container first.
        match self {
            Self::Leaf(parser) => match parser.ingest(input) {
                IngestResult::Ready(parser) => IngestResult::Ready(Self::Leaf(parser)),
                IngestResult::Success(success) => match success {
                    leaf::IngestSuccess::Single(leaf) => {
                        IngestResult::Success(IngestSuccess::Single(leaf.into()))
                    }
                    leaf::IngestSuccess::SingleWithInterrupt(block, segment) => {
                        // TODO: use the segment and put it back into the shit.
                        IngestResult::Success(IngestSuccess::Single(block.into()))
                    }
                    leaf::IngestSuccess::MultipleWithInterrupt(leaves, segment) => {
                        // TODO: use the segment and put it back into the shit.
                        IngestResult::Success(IngestSuccess::Multiple(
                            leaves.into_iter().map(Block::from).collect(),
                        ))
                    }
                },
                IngestResult::Failure(failure) => {
                    unimplemented!("missing implementation for markdown: {:?}", failure)
                }
            },
        }
    }
}

pub(super) enum FinalizeResult<'a> {
    Nothing,
    Single(Block<'a>),
    Multiple(Vec<Block<'a>>),
}

impl<'a> Finalize for BlockParser<'a> {
    type Result = FinalizeResult<'a>;

    fn finalize(self) -> Self::Result {
        match self {
            BlockParser::Leaf(parser) => match parser.finalize() {
                leaf::FinalizeResult::Nothing => FinalizeResult::Nothing,
                leaf::FinalizeResult::Single(leaf) => FinalizeResult::Single(leaf.into()),
                leaf::FinalizeResult::Multiple(leaves) => {
                    FinalizeResult::Multiple(leaves.into_iter().map(Block::from).collect())
                }
            },
        }
    }
}
