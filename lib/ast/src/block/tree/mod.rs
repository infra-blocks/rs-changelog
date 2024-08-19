mod block;
/* mod block_quote; */
/* mod container; */
mod atx_heading;
mod blank_line;
mod error;
mod fenced_code;
mod indented_code;
mod ingestion;
mod leaf;
pub mod link;
mod link_reference_definition;
mod paragraph;
mod parser;
mod setext_heading;
mod thematic_break;

pub use block::*;
use ingestion::{Consume, NodeState};
pub use leaf::*;
pub use paragraph::*;
/* pub use block_quote::*; */
/* pub use container::*; */
pub use atx_heading::*;
pub use fenced_code::*;
pub use indented_code::*;
pub use link_reference_definition::*;
pub use setext_heading::*;
pub use thematic_break::*;

use crate::segment::*;

pub struct BlockTree<'a> {
    pub block_nodes: Vec<BlockNode<'a>>,
}

impl<'a> From<&'a str> for BlockTree<'a> {
    fn from(text: &'a str) -> Self {
        let mut block_nodes = Vec::new();

        let mut line_segments = text.line_segments();
        let Some(first_segment) = line_segments.next() else {
            return Self { block_nodes };
        };
        let mut current_block = BlockNode::from(first_segment);

        for segment in line_segments {
            match current_block.consume(segment) {
                // Nothing to do if the block is willing to ingest more.
                NodeState::InProgress => {}
                // If the block completed its ingestion and was interrupted by the beginning
                // of a new block, then we reset the current block and store the previous one.
                NodeState::InterruptedBy(new_block) => {
                    block_nodes.push(current_block);
                    current_block = new_block;
                }
                // If the block was superseded by a new block, then we just replace the current
                // block with the new one.
                NodeState::ReplacedBy(mut nodes) => {
                    while nodes.len() > 1 {
                        let new_block = nodes.pop_front().unwrap();
                        block_nodes.push(new_block);
                    }
                    current_block = nodes.pop_front().unwrap();
                }
            }
        }

        // TODO: finalize.
        block_nodes.push(current_block);
        Self { block_nodes }
    }
}
