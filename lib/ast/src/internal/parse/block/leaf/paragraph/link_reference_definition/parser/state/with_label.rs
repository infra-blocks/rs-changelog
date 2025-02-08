use either::Either;
use segment::LineSegment;

use crate::internal::parse::{
    block::leaf::paragraph::link_reference_definition::LinkReferenceDefinition,
    link::LinkLabel,
    parser::{Finalize, Ingest, IngestResult},
};

use super::{
    utils::{try_extract_destination, try_parse_title},
    WithLabelAndDestinationParsingTitleState, WithLabelAndDestinationState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithLabelState<'a> {
    // TODO: since it'S pretty much guaranteed to be only one segment, maybe we can type it as such.
    pub segments: Vec<LineSegment<'a>>,
    pub label: LinkLabel<'a>,
}

impl<'a> WithLabelState<'a> {
    pub fn new(segments: Vec<LineSegment<'a>>, label: LinkLabel<'a>) -> Self {
        Self { segments, label }
    }
}

pub enum WithLabelNextState<'a> {
    WithLabelAndDestination(WithLabelAndDestinationState<'a>),
    WithLabelAndDestinationParsingTitle(WithLabelAndDestinationParsingTitleState<'a>),
}

impl<'a> From<WithLabelAndDestinationState<'a>> for WithLabelNextState<'a> {
    fn from(value: WithLabelAndDestinationState<'a>) -> Self {
        WithLabelNextState::WithLabelAndDestination(value)
    }
}

impl<'a> From<WithLabelAndDestinationParsingTitleState<'a>> for WithLabelNextState<'a> {
    fn from(value: WithLabelAndDestinationParsingTitleState<'a>) -> Self {
        WithLabelNextState::WithLabelAndDestinationParsingTitle(value)
    }
}

// TODO: test that failures are this level return 2 segments.
impl<'a> Ingest for WithLabelState<'a> {
    type Input = LineSegment<'a>;
    // Upon success, we will transition to one of two possible states.
    type Ready = WithLabelNextState<'a>;
    type Success = LinkReferenceDefinition<'a>;
    type Failure = Vec<LineSegment<'a>>;

    fn ingest(
        self,
        segment: Self::Input,
    ) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        let mut segments = self.segments;
        let link_label = self.label;

        segments.push(segment);
        // If the first line contained the link label, then this line *must* contain a valid link destination
        // and *may* contain a title.
        let unindented = segment.trim_start();
        match try_extract_destination(unindented) {
            Ok(extraction) => {
                let link_destination = extraction.extracted;
                let remaining = extraction.remaining;
                if remaining.is_empty() {
                    return IngestResult::Ready(
                        WithLabelAndDestinationState::new(segments, link_label, link_destination)
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
}

impl<'a> Finalize for WithLabelState<'a> {
    // A label is not enough to create a valid link reference definition, so we return the segments.
    type Result = Vec<LineSegment<'a>>;

    fn finalize(self) -> Self::Result {
        self.segments
    }
}
