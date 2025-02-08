use segment::LineSegment;

use super::{
    block::{self, Block, BlockParser},
    parser::{Finalize, Ingest, IngestResult},
};

#[derive(Debug, Clone)]
pub struct BlocksIterator<'a, I> {
    source: I,
    // Behind an option to use move semantics, but it should only be none in the
    // case where the user calls next successively after no more outputs have been produced.
    parser: Option<BlockParser<'a>>,
    // TODO: look into flatten?
    cache: Option<Vec<Block<'a>>>,
}

impl<'a, I> BlocksIterator<'a, I>
where
    I: Iterator + Sized,
{
    pub fn new(source: I) -> Self {
        Self {
            source,
            parser: Some(BlockParser::new()),
            cache: None,
        }
    }

    fn pop_from_cache(&mut self) -> Option<Block<'a>> {
        match self.cache.take() {
            Some(mut blocks) => {
                let block = blocks.pop().unwrap();
                if blocks.is_empty() {
                    self.cache = None;
                } else {
                    self.cache = Some(blocks);
                }
                Some(block)
            }
            None => None,
        }
    }
}

pub trait Blocks
where
    Self: Iterator + Sized,
{
    fn blocks<'a>(self) -> BlocksIterator<'a, Self>;
}

impl<I> Blocks for I
where
    I: Iterator + Sized,
{
    fn blocks<'a>(self) -> BlocksIterator<'a, I> {
        BlocksIterator::new(self)
    }
}

impl<'a, I> Iterator for BlocksIterator<'a, I>
where
    I: Iterator<Item = LineSegment<'a>>,
{
    type Item = crate::block::Block<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(block) = self.pop_from_cache() {
            return Some(block.into());
        }

        match self.source.next() {
            Some(segment) => match self.parser.take().unwrap().ingest(segment) {
                IngestResult::Ready(parser) => {
                    self.parser = Some(parser);
                    // TODO: Uselessly pops from the cache here. We should have a utility function that
                    // passes the segments until the parser finalizes.
                    self.next()
                }
                IngestResult::Success(success) => match success {
                    block::IngestSuccess::Single(block) => {
                        self.parser = Some(BlockParser::new());
                        Some(block.into())
                    }
                    block::IngestSuccess::Multiple(blocks) => {
                        self.parser = Some(BlockParser::new());
                        self.cache = Some(blocks);
                        Some(self.pop_from_cache().unwrap().into())
                    }
                },
            },
            None => match self.parser.take() {
                Some(parser) => match parser.finalize() {
                    block::FinalizeResult::Nothing => None,
                    block::FinalizeResult::Single(block) => Some(block.into()),
                    block::FinalizeResult::Multiple(blocks) => {
                        self.cache = Some(blocks);
                        self.pop_from_cache().map(|block| block.into())
                    }
                },
                None => None,
            },
        }
    }
}
