use either::Either;
use segment::{LineSegment, Segment};

use crate::internal::parse::{
    block::leaf::paragraph::link_reference_definition::LinkReferenceDefinition,
    link::{LinkDestination, LinkLabel},
    parser::{Finalize, Ingest, IngestResult},
};

use super::{utils::try_parse_title, WithLabelAndDestinationParsingTitleState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithLabelAndDestinationState<'a> {
    pub segments: Vec<LineSegment<'a>>,
    pub link_label: LinkLabel<'a>,
    pub link_destination: LinkDestination<'a>,
}

impl<'a> WithLabelAndDestinationState<'a> {
    pub fn new(
        segments: Vec<LineSegment<'a>>,
        link_label: LinkLabel<'a>,
        link_destination: LinkDestination<'a>,
    ) -> Self {
        Self {
            segments,
            link_label,
            link_destination,
        }
    }
}

impl<'a> Ingest for WithLabelAndDestinationState<'a> {
    type Input = LineSegment<'a>;
    // If the input is good, we necessarily move to the next state, although we could finalize too.
    type Ready = WithLabelAndDestinationParsingTitleState<'a>;
    type Success = LinkReferenceDefinition<'a>;
    // Because we already have a valid partial result, when there is a failure, we return that result
    // plus the invalid segment.
    type Failure = (LinkReferenceDefinition<'a>, LineSegment<'a>);

    fn ingest(
        self,
        segment: Self::Input,
    ) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        let mut segments = self.segments;
        let link_label = self.link_label;
        let link_destination = self.link_destination;

        // If we already have a link label and a link destination, two things can occur:
        // We are able to create/start a link title from the new segment, or not. In the latter
        // case, we still have a partial result with the existing fields we have already extracted.
        let unindented = segment.trim_start();
        match try_parse_title(unindented) {
            Ok(Either::Left(parser)) => {
                segments.push(segment);
                IngestResult::Ready(WithLabelAndDestinationParsingTitleState::new(
                    segments,
                    link_label,
                    link_destination,
                    parser,
                ))
            }
            Ok(Either::Right(link_title)) => {
                segments.push(segment);
                IngestResult::Success(LinkReferenceDefinition::new(
                    segments,
                    link_label,
                    link_destination,
                    Some(link_title),
                ))
            }
            Err(_) => IngestResult::Failure((
                LinkReferenceDefinition::without_title(segments, link_label, link_destination),
                segment,
            )),
        }
    }
}

impl<'a> Finalize for WithLabelAndDestinationState<'a> {
    type Result = LinkReferenceDefinition<'a>;

    fn finalize(self) -> Self::Result {
        LinkReferenceDefinition::without_title(
            self.segments,
            self.link_label,
            self.link_destination,
        )
    }
}
