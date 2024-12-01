mod parser;

use itertools::PutBackN;
pub use parser::*;

use crate::{
    internal::{
        parse::{
            link::{LinkDestination, LinkLabel, LinkTitle},
            parser::{Finalize, Ingest, IngestResult},
        },
        utils::iter_ext::PutBackChunk,
    },
    Segment,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkReferenceDefinition<'a> {
    pub segments: Vec<Segment<'a>>,
    pub label: LinkLabel<'a>,
    pub destination: LinkDestination<'a>,
    pub title: Option<LinkTitle<'a>>,
}

impl<'a> LinkReferenceDefinition<'a> {
    fn new(
        segments: Vec<Segment<'a>>,
        label: LinkLabel<'a>,
        destination: LinkDestination<'a>,
        title: Option<LinkTitle<'a>>,
    ) -> Self {
        Self {
            segments,
            label,
            destination,
            title,
        }
    }

    fn without_title(
        segments: Vec<Segment<'a>>,
        label: LinkLabel<'a>,
        destination: LinkDestination<'a>,
    ) -> LinkReferenceDefinition<'a> {
        Self::new(segments, label, destination, None)
    }

    fn with_title(
        segments: Vec<Segment<'a>>,
        label: LinkLabel<'a>,
        destination: LinkDestination<'a>,
        title: LinkTitle<'a>,
    ) -> LinkReferenceDefinition<'a> {
        Self::new(segments, label, destination, Some(title))
    }

    // TODO: in trait, plus add it on iters.
    pub fn try_read(segments: &mut PutBackN<impl Iterator<Item = Segment<'a>>>) -> Option<Self> {
        let mut parser = LinkReferenceDefinitionParser::new();
        while let Some(segment) = segments.next() {
            match parser.ingest(segment) {
                IngestResult::Ready(next_parser) => parser = next_parser,
                IngestResult::Success(link_refence_definition) => {
                    return Some(link_refence_definition)
                }
                IngestResult::Failure(failure) => match failure {
                    LinkReferenceDefinitionParserFailure::Complete(rejected) => {
                        segments.put_back_chunk(rejected);
                        return None;
                    }
                    LinkReferenceDefinitionParserFailure::Partial(
                        link_reference_definition,
                        segment,
                    ) => {
                        segments.put_back(segment);
                        return Some(link_reference_definition);
                    }
                },
            }
        }
        match parser.finalize() {
            LinkReferenceDefinitionParserFinalizeResult::Failure(rejected) => {
                segments.put_back_chunk(rejected);
                None
            }
            LinkReferenceDefinitionParserFinalizeResult::Success(link_reference_definition) => {
                Some(link_reference_definition)
            }
        }
    }
}
