use crate::{
    internal::parse::{
        block::leaf::paragraph::link_reference_definition::LinkReferenceDefinition,
        link::{LinkDestination, LinkLabel, LinkTitleParser},
        parser::{Finalize, Ingest, IngestResult},
    },
    Segment,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithLabelAndDestinationParsingTitleState<'a> {
    pub segments: Vec<Segment<'a>>,
    pub link_label: LinkLabel<'a>,
    pub link_destination: LinkDestination<'a>,
    pub link_title_parser: LinkTitleParser<'a>,
}

impl<'a> WithLabelAndDestinationParsingTitleState<'a> {
    pub fn new(
        segments: Vec<Segment<'a>>,
        link_label: LinkLabel<'a>,
        link_destination: LinkDestination<'a>,
        link_title_parser: LinkTitleParser<'a>,
    ) -> Self {
        Self {
            segments,
            link_label,
            link_destination,
            link_title_parser,
        }
    }
}

impl<'a> Ingest for WithLabelAndDestinationParsingTitleState<'a> {
    type Input = Segment<'a>;
    type Ready = Self;
    type Success = LinkReferenceDefinition<'a>;
    type Failure = Vec<Segment<'a>>;

    fn ingest(
        self,
        segment: Self::Input,
    ) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        let mut segments = self.segments;
        let link_label = self.link_label;
        let link_destination = self.link_destination;
        let parser = self.link_title_parser;

        segments.push(segment);
        // The indents are not part of the title.
        let unindented = segment.trim_start();
        match parser.ingest(unindented) {
            IngestResult::Ready(parser) => {
                IngestResult::Ready(Self::new(segments, link_label, link_destination, parser))
            }
            IngestResult::Success(link_title) => {
                IngestResult::Success(LinkReferenceDefinition::with_title(
                    segments,
                    link_label,
                    link_destination,
                    link_title,
                ))
            }
            IngestResult::Failure(_) => IngestResult::Failure(segments),
        }
    }
}

impl<'a> Finalize for WithLabelAndDestinationParsingTitleState<'a> {
    // Always a failure if the parser hasn't finalized yet.
    type Result = Vec<Segment<'a>>;

    fn finalize(self) -> Self::Result {
        self.segments
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn ingest_ready() {
        let first_segment = Segment::first("[toto]: www.tata.com 'title start\n");
        let link_label = LinkLabel::new(first_segment.slice(0..6).try_into().unwrap());
        let link_destination = LinkDestination::new(first_segment.slice(8..20).try_into().unwrap());
        let link_title_parser = LinkTitleParser::new()
            .ingest(first_segment.slice(21..))
            .unwrap_ready();
        let state = WithLabelAndDestinationParsingTitleState::new(
            vec![first_segment],
            link_label.clone(),
            link_destination.clone(),
            link_title_parser.clone(),
        );
        let next_segment = first_segment.next("not quite the end yet!\n");
        let link_title_parser = link_title_parser.ingest(next_segment).unwrap_ready();
        let state = state.ingest(next_segment).unwrap_ready();
        assert_eq!(
            state,
            WithLabelAndDestinationParsingTitleState::new(
                vec![first_segment, next_segment],
                link_label,
                link_destination,
                link_title_parser
            )
        );
    }

    #[test]
    fn ingest_reject() {
        let first_segment = Segment::first("[toto]: www.tata.com 'title start\n");
        let link_label = LinkLabel::new(first_segment.slice(0..6).try_into().unwrap());
        let link_destination = LinkDestination::new(first_segment.slice(8..20).try_into().unwrap());
        let link_title_parser = LinkTitleParser::new()
            .ingest(first_segment.slice(21..))
            .unwrap_ready();
        let state = WithLabelAndDestinationParsingTitleState::new(
            vec![first_segment],
            link_label.clone(),
            link_destination.clone(),
            link_title_parser.clone(),
        );
        let invalid_segment = first_segment.next("' nope!\n");
        let state = state.ingest(invalid_segment).unwrap_failure();
        // It rejects with all the segments.
        assert_eq!(state, vec![first_segment, invalid_segment]);
    }

    #[test]
    fn ingest_success() {
        let first_segment = Segment::first("[toto]: www.tata.com 'title start\n");
        let link_label = LinkLabel::new(first_segment.slice(0..6).try_into().unwrap());
        let link_destination = LinkDestination::new(first_segment.slice(8..20).try_into().unwrap());
        let link_title_parser = LinkTitleParser::new()
            .ingest(first_segment.slice(21..))
            .unwrap_ready();
        let state = WithLabelAndDestinationParsingTitleState::new(
            vec![first_segment],
            link_label.clone(),
            link_destination.clone(),
            link_title_parser.clone(),
        );
        // TODO: to fix this test, we need to make the link title parser expect the newline, or take it out in the state.
        let next_segment = first_segment.next("    valid segment that should be trimmed!'\n");
        let link_reference_definition = state.ingest(next_segment).unwrap_success();
        let link_title = link_title_parser
            .ingest(next_segment.slice(4..))
            .unwrap_success();
        assert_eq!(
            link_reference_definition,
            LinkReferenceDefinition::with_title(
                vec![first_segment, next_segment],
                link_label,
                link_destination,
                link_title
            )
        );
    }

    #[test]
    fn finalize() {}
}
