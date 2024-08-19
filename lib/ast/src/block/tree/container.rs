use std::marker::PhantomData;

use super::{ingestion::MaybeIngest, IngestionResult};

pub enum ContainerNode<'a> {
    BlockQuote(BlockQuoteNode<'a>),
    /*    List,
    ListItem, */
}

impl<'a> MaybeIngest for ContainerNode<'a> {
    fn maybe_ingest(segment: Segment<'_>) -> IngestionResult<'a, Self> {
        if segment.text.starts_with("```") {
            let mut remainder = segment;
            remainder.text = &remainder.text[3..];
            return Some((ContainerNode { stuff: PhantomData }, remainder));
        }
        None
    }
}
