use crate::{
    internal::parse::{
        parser::{Finalize, Ingest, IngestResult},
        segment::{SingleQuotesLinkTitleContinuationSegment, SingleQuotesLinkTitleOpeningSegment},
    },
    Segment,
};
use std::{convert::Infallible, iter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleQuotesLinkTitle<'a> {
    pub opening_segment: SingleQuotesLinkTitleOpeningSegment<'a>,
    pub continuation_segments: Vec<SingleQuotesLinkTitleContinuationSegment<'a>>,
}

impl<'a> SingleQuotesLinkTitle<'a> {
    pub fn new(
        opening_segment: SingleQuotesLinkTitleOpeningSegment<'a>,
        continuation_segments: Vec<SingleQuotesLinkTitleContinuationSegment<'a>>,
    ) -> Self {
        Self {
            opening_segment,
            continuation_segments,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleQuotesLinkTitleParser<'a> {
    Idle,
    Building(
        SingleQuotesLinkTitleOpeningSegment<'a>,
        Vec<SingleQuotesLinkTitleContinuationSegment<'a>>,
    ),
}

impl<'a> SingleQuotesLinkTitleParser<'a> {
    pub fn new() -> Self {
        Self::Idle
    }
}

impl<'a> Ingest for SingleQuotesLinkTitleParser<'a> {
    type Input = Segment<'a>;
    type Ready = Self;
    type Success = SingleQuotesLinkTitle<'a>;
    type Failure = Vec<Segment<'a>>;

    fn ingest(
        self,
        segment: Self::Input,
    ) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        match self {
            Self::Idle => match SingleQuotesLinkTitleOpeningSegment::try_from(segment) {
                Ok(opening) => {
                    // The link title can be wholly contained within a single segment, if it ends with a closing symbol.
                    if opening.is_closing() {
                        IngestResult::Success(SingleQuotesLinkTitle::new(opening, Vec::new()))
                    } else {
                        IngestResult::Ready(Self::Building(opening, Vec::new()))
                    }
                }
                Err(segment) => IngestResult::Failure(vec![segment]),
            },
            Self::Building(opening, mut continuation_segments) => {
                match SingleQuotesLinkTitleContinuationSegment::try_from(segment) {
                    Ok(continuation) => {
                        if continuation.is_closing() {
                            continuation_segments.push(continuation);
                            IngestResult::Success(SingleQuotesLinkTitle::new(
                                opening,
                                continuation_segments,
                            ))
                        } else {
                            continuation_segments.push(continuation);
                            IngestResult::Ready(Self::Building(opening, continuation_segments))
                        }
                    }
                    Err(segment) => {
                        let mut rejected: Vec<_> = iter::once(opening.segment)
                            .chain(
                                continuation_segments
                                    .into_iter()
                                    .map(|continuation| continuation.segment),
                            )
                            .collect();
                        rejected.push(segment);
                        IngestResult::Failure(rejected)
                    }
                }
            }
        }
    }
}

impl<'a> Finalize for SingleQuotesLinkTitleParser<'a> {
    // We can finalize in only 2 scenarios: the parser is idle, or the parser is holding an unclosed title.
    // In both cases, a valid result cannot be built.
    type Result = Result<Infallible, Vec<Segment<'a>>>;

    fn finalize(self) -> Self::Result {
        match self {
            Self::Idle => Err(Vec::new()),
            Self::Building(opening, continuation_segments) => {
                let segments = iter::once(opening.segment)
                    .chain(
                        continuation_segments
                            .into_iter()
                            .map(|continuation| continuation.segment),
                    )
                    .collect();
                Err(segments)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::StrExt;

    use super::*;

    #[test]
    fn idle_reject_empty() {
        let parser = SingleQuotesLinkTitleParser::new();
        let segment = Segment::default();
        assert_eq!(parser.ingest(segment), IngestResult::Failure(vec![segment]));
    }

    #[test]
    fn idle_reject_invalid() {
        let parser = SingleQuotesLinkTitleParser::new();
        let segment = Segment::default();
        assert_eq!(parser.ingest(segment), IngestResult::Failure(vec![segment]));
    }

    #[test]
    fn idle_finalize() {
        let parser = SingleQuotesLinkTitleParser::new();
        assert_eq!(parser.finalize(), Err(Vec::new()));
    }

    #[test]
    fn idle_success() {
        let parser = SingleQuotesLinkTitleParser::new();
        let segment = Segment::first("'the word is the bird'");
        assert_eq!(
            parser.ingest(segment).unwrap_success(),
            SingleQuotesLinkTitle::new(
                SingleQuotesLinkTitleOpeningSegment::try_from(segment).unwrap(),
                Vec::new()
            )
        );
    }

    #[test]
    fn idle_building_success() {
        let segments = "'the word\nis the bird'"
            .line_segments()
            .collect::<Vec<_>>();
        let parser = SingleQuotesLinkTitleParser::new();
        let parser = parser.ingest(segments[0]).unwrap_ready();
        assert_eq!(
            parser.ingest(segments[1]).unwrap_success(),
            SingleQuotesLinkTitle::new(
                SingleQuotesLinkTitleOpeningSegment::try_from(segments[0]).unwrap(),
                vec![SingleQuotesLinkTitleContinuationSegment::try_from(segments[1]).unwrap()]
            )
        );
    }

    #[test]
    fn idle_building_reject() {
        let segments = "'the word\n\n".line_segments().collect::<Vec<_>>();
        let parser = SingleQuotesLinkTitleParser::new();
        let parser = parser.ingest(segments[0]).unwrap_ready();
        assert_eq!(parser.ingest(segments[1]).unwrap_failure(), segments);
    }

    #[test]
    fn idle_building_finalize() {
        let segment = Segment::first("'the word\n");
        let parser = SingleQuotesLinkTitleParser::new();
        let parser = parser.ingest(segment).unwrap_ready();
        assert_eq!(parser.finalize(), Err(vec![segment]));
    }
}
