use std::{convert::Infallible, iter};

use crate::internal::parse::{
    parser::{Finalize, Ingest, IngestResult},
    segment::{ParenthesesLinkTitleContinuationSegment, ParenthesesLinkTitleOpeningSegment},
};
use segment::Segment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesesLinkTitle<'a> {
    pub opening_segments: ParenthesesLinkTitleOpeningSegment<'a>,
    pub continuation_segments: Vec<ParenthesesLinkTitleContinuationSegment<'a>>,
}

impl<'a> ParenthesesLinkTitle<'a> {
    pub fn new(
        opening_segments: ParenthesesLinkTitleOpeningSegment<'a>,
        continuation_segments: Vec<ParenthesesLinkTitleContinuationSegment<'a>>,
    ) -> Self {
        Self {
            opening_segments,
            continuation_segments,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParenthesesLinkTitleParser<'a> {
    Idle,
    Building(
        ParenthesesLinkTitleOpeningSegment<'a>,
        Vec<ParenthesesLinkTitleContinuationSegment<'a>>,
    ),
}

impl<'a> ParenthesesLinkTitleParser<'a> {
    pub fn new() -> Self {
        Self::Idle
    }
}

impl<'a> Ingest for ParenthesesLinkTitleParser<'a> {
    type Input = Segment<'a>;
    type Ready = Self;
    type Success = ParenthesesLinkTitle<'a>;
    type Failure = Vec<Segment<'a>>;

    fn ingest(
        self,
        segment: Self::Input,
    ) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        match self {
            Self::Idle => match ParenthesesLinkTitleOpeningSegment::try_from(segment) {
                Ok(opening) => {
                    // The link title can be wholly contained within a single segment, if it ends with a closing symbol.
                    if opening.is_closing() {
                        IngestResult::Success(ParenthesesLinkTitle::new(opening, Vec::new()))
                    } else {
                        IngestResult::Ready(Self::Building(opening, Vec::new()))
                    }
                }
                Err(segment) => IngestResult::Failure(vec![segment]),
            },
            Self::Building(opening, mut continuation_segments) => {
                match ParenthesesLinkTitleContinuationSegment::try_from(segment) {
                    Ok(continuation) => {
                        if continuation.is_closing() {
                            continuation_segments.push(continuation);
                            IngestResult::Success(ParenthesesLinkTitle::new(
                                opening,
                                continuation_segments,
                            ))
                        } else {
                            continuation_segments.push(continuation);
                            IngestResult::Ready(Self::Building(opening, continuation_segments))
                        }
                    }
                    Err(segment) => {
                        let mut rejected: Vec<_> = iter::once(opening.0)
                            .chain(
                                continuation_segments
                                    .into_iter()
                                    .map(|continuation| continuation.0),
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

impl<'a> Finalize for ParenthesesLinkTitleParser<'a> {
    // We can finalize in only 2 scenarios: the parser is idle, or the parser is holding an unclosed title.
    // In both cases, a valid result cannot be built.
    type Result = Result<Infallible, Vec<Segment<'a>>>;

    fn finalize(self) -> Self::Result {
        match self {
            Self::Idle => Err(Vec::new()),
            Self::Building(opening, continuation_segments) => {
                let segments = iter::once(opening.0)
                    .chain(
                        continuation_segments
                            .into_iter()
                            .map(|continuation| continuation.0),
                    )
                    .collect();
                Err(segments)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use segment::SegmentStrExt;

    #[test]
    fn idle_reject_empty() {
        let parser = ParenthesesLinkTitleParser::new();
        let segment = Segment::default();
        assert_eq!(parser.ingest(segment), IngestResult::Failure(vec![segment]));
    }

    #[test]
    fn idle_reject_invalid() {
        let parser = ParenthesesLinkTitleParser::new();
        let segment = Segment::default();
        assert_eq!(parser.ingest(segment), IngestResult::Failure(vec![segment]));
    }

    #[test]
    fn idle_finalize() {
        let parser = ParenthesesLinkTitleParser::new();
        assert_eq!(parser.finalize(), Err(Vec::new()));
    }

    #[test]
    fn idle_success() {
        let parser = ParenthesesLinkTitleParser::new();
        let segment = Segment::first("(the word is the bird)");
        assert_eq!(
            parser.ingest(segment).unwrap_success(),
            ParenthesesLinkTitle::new(
                ParenthesesLinkTitleOpeningSegment::try_from(segment).unwrap(),
                Vec::new()
            )
        );
    }

    #[test]
    fn idle_building_success() {
        let segments = "(the word\nis the bird)"
            .lines()
            .collect::<Vec<_>>();
        let parser = ParenthesesLinkTitleParser::new();
        let parser = parser.ingest(segments[0]).unwrap_ready();
        assert_eq!(
            parser.ingest(segments[1]).unwrap_success(),
            ParenthesesLinkTitle::new(
                ParenthesesLinkTitleOpeningSegment::try_from(segments[0]).unwrap(),
                vec![ParenthesesLinkTitleContinuationSegment::try_from(segments[1]).unwrap()]
            )
        );
    }

    #[test]
    fn idle_building_reject() {
        let segments = "(the word\n\n".lines().collect::<Vec<_>>();
        let parser = ParenthesesLinkTitleParser::new();
        let parser = parser.ingest(segments[0]).unwrap_ready();
        assert_eq!(parser.ingest(segments[1]).unwrap_failure(), segments);
    }

    #[test]
    fn idle_building_finalize() {
        let segment = Segment::first("(the word\n");
        let parser = ParenthesesLinkTitleParser::new();
        let parser = parser.ingest(segment).unwrap_ready();
        assert_eq!(parser.finalize(), Err(vec![segment]));
    }
}
