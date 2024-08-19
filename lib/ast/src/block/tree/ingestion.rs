use std::collections::VecDeque;

use crate::Segment;

use super::BlockNode;

//TODO: merge the interrupted by and the replaced by variants. They can both be expressed
// as a deque of results. An interruption returns 2 nodes, the current, finalized one and
// the new, in progress one. A replacement returns only one node, where the caller replaces
// the current one with the new one. In addition, a finalized node can be replaced by
// several nodes (as for the paragraph turning into several link reference definitions, for example).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalParseResult<'a> {
    blocks: Vec<BlockNode<'a>>,
    // TODO: build the enum for that. That should go in a ChainedParseResult.
    /* next_parser: BlockParser<'a>, */
}

/* // TODO: move out of here.
pub struct BlockParseResult<'a> {
    /// The nodes parsed from the segments.
    ///
    /// Because some blocks can morph into several other blocks upon finalization (like a paragraph
    /// into link refence deinitions), and because nodes can be interrupted, the uniform interface
    /// is a vector of nodes. The last node is always meant to be the current, ready
    pub nodes: Vec<BlockNode<'a>>,
    pub builder: BlockNodeBuilder<'a>,
}
 */

/// The state of a node in the ingestion process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeState<'a> {
    /// This variant is returned when the node believes it can still consume
    /// segments.
    InProgress,
    /// This variant is returned when the current node has finished consuming
    /// segments and another one takes over. The current node should be considered
    /// finalized and the new one: in progress.
    InterruptedBy(BlockNode<'a>),
    // TODO: why a vecdeque? Just return a vec bro.
    /// This occurrence indicates that the current node was superseded and needs
    /// to be replaced by the one or several nodes. This can happen predictibly,
    /// for example, when parsing link reference definitions: they always start out
    /// as paragraphs, but then the paragraph is possibly replaced upon finalization.
    ReplacedBy(VecDeque<BlockNode<'a>>),
}

impl<'a> NodeState<'a> {
    pub fn unwrap_interrupting_node(self) -> BlockNode<'a> {
        match self {
            Self::InterruptedBy(node) => node,
            _ => panic!("cannot unwrap interrupting node on {:?}", self),
        }
    }

    pub fn unwrap_replacement_nodes(self) -> VecDeque<BlockNode<'a>> {
        match self {
            Self::ReplacedBy(nodes) => nodes,
            _ => panic!("cannot unwrap replacement node on {:?}", self),
        }
    }

    pub fn in_progress(&self) -> bool {
        matches!(self, NodeState::InProgress)
    }
}

pub trait Consume<'a> {
    fn consume(&mut self, segment: Segment<'a>) -> NodeState<'a>;
}

// TODO: trait for ingest_line_segment. Maybe even LineSegment type.
// TODO: Consider a two state parameterized trait: Ingest<S> where S is either InProgess or Complete.

// TODO: put this documentation somewhere better.
// Because every block can be embedded in another one, like a paragraph can be embedded in a list item,
// a block quote or whatever, its content is represented as a list of segments that are not necessarily
// contiguous. This also means that the outer node has first dibs on ingestion.
