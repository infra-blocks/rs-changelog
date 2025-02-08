use either::Either;

use crate::internal::parse::{
    block::leaf::paragraph::link_reference_definition::LinkReferenceDefinition,
    parser::{Finalize, Ingest, IngestResult},
};
use segment::{LineSegment, Segment};

use super::{
    utils::{try_extract_destination, try_extract_label, try_parse_title},
    WithLabelAndDestinationParsingTitleState, WithLabelAndDestinationState, WithLabelState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdleState<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> IdleState<'a> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

pub enum IdleNextState<'a> {
    WithLabel(WithLabelState<'a>),
    WithLabelAndDestination(WithLabelAndDestinationState<'a>),
    WithLabelAndDestinationParsingTitle(WithLabelAndDestinationParsingTitleState<'a>),
}

impl<'a> From<WithLabelState<'a>> for IdleNextState<'a> {
    fn from(value: WithLabelState<'a>) -> Self {
        IdleNextState::WithLabel(value)
    }
}

impl<'a> From<WithLabelAndDestinationState<'a>> for IdleNextState<'a> {
    fn from(value: WithLabelAndDestinationState<'a>) -> Self {
        IdleNextState::WithLabelAndDestination(value)
    }
}

impl<'a> From<WithLabelAndDestinationParsingTitleState<'a>> for IdleNextState<'a> {
    fn from(value: WithLabelAndDestinationParsingTitleState<'a>) -> Self {
        IdleNextState::WithLabelAndDestinationParsingTitle(value)
    }
}

impl<'a> Ingest for IdleState<'a> {
    type Input = LineSegment<'a>;
    type Ready = IdleNextState<'a>;
    type Success = LinkReferenceDefinition<'a>;
    type Failure = Vec<LineSegment<'a>>;

    fn ingest(
        self,
        segment: Self::Input,
    ) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        let segments = vec![segment];
        match try_extract_label(segment) {
            Ok(extraction) => {
                let link_label = extraction.extracted;
                let remaining = extraction.remaining;
                if remaining.is_empty() {
                    return IngestResult::Ready(WithLabelState::new(segments, link_label).into());
                }

                // Otherwise we have at least one none whitespace character to consume and it
                // has to be a valid link destination.
                match try_extract_destination(remaining) {
                    Ok(extraction) => {
                        let link_destination = extraction.extracted;
                        let remaining = extraction.remaining;
                        if remaining.is_empty() {
                            return IngestResult::Ready(
                                WithLabelAndDestinationState::new(
                                    segments,
                                    link_label,
                                    link_destination,
                                )
                                .into(),
                            );
                        }

                        // If there is still something left, then it has to be a valid link title.
                        // It is possible that the title continutes on the next line, however.
                        match try_parse_title(remaining) {
                            Ok(Either::Left(parser)) => IngestResult::Ready(
                                WithLabelAndDestinationParsingTitleState::new(
                                    segments,
                                    link_label,
                                    link_destination,
                                    parser,
                                )
                                .into(),
                            ),
                            Ok(Either::Right(link_title)) => {
                                IngestResult::Success(LinkReferenceDefinition::with_title(
                                    segments,
                                    link_label,
                                    link_destination,
                                    link_title,
                                ))
                            }
                            Err(_) => IngestResult::Failure(segments),
                        }
                    }
                    Err(_) => IngestResult::Failure(segments),
                }
            }
            Err(_) => IngestResult::Failure(segments),
        }
    }
}

// Nothing to do!
impl<'a> Finalize for IdleState<'a> {
    type Result = ();

    fn finalize(self) -> Self::Result {
        ()
    }
}
