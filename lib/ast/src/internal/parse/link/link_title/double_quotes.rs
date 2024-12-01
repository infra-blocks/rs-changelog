use crate::{
    internal::parse::{
        parser::{Finalize, Ingest, IngestResult},
        segment::{DoubleQuotesLinkTitleContinuationSegment, DoubleQuotesLinkTitleOpeningSegment},
    },
    Segment,
};
use std::{convert::Infallible, iter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleQuotesLinkTitle<'a> {
    pub opening_segments: DoubleQuotesLinkTitleOpeningSegment<'a>,
    pub continuation_segments: Vec<DoubleQuotesLinkTitleContinuationSegment<'a>>,
}

impl<'a> DoubleQuotesLinkTitle<'a> {
    pub fn new(
        opening_segments: DoubleQuotesLinkTitleOpeningSegment<'a>,
        continuation_segments: Vec<DoubleQuotesLinkTitleContinuationSegment<'a>>,
    ) -> Self {
        Self {
            opening_segments,
            continuation_segments,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoubleQuotesLinkTitleParser<'a> {
    Idle,
    Building(
        DoubleQuotesLinkTitleOpeningSegment<'a>,
        Vec<DoubleQuotesLinkTitleContinuationSegment<'a>>,
    ),
}

impl<'a> DoubleQuotesLinkTitleParser<'a> {
    pub fn new() -> Self {
        Self::Idle
    }
}

impl<'a> Ingest for DoubleQuotesLinkTitleParser<'a> {
    type Input = Segment<'a>;
    type Ready = Self;
    type Success = DoubleQuotesLinkTitle<'a>;
    type Failure = Vec<Segment<'a>>;

    fn ingest(
        self,
        segment: Self::Input,
    ) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        match self {
            Self::Idle => match DoubleQuotesLinkTitleOpeningSegment::try_from(segment) {
                Ok(opening) => {
                    // The link title can be wholly contained within a single segment, if it ends with a closing symbol.
                    if opening.is_closing() {
                        IngestResult::Success(DoubleQuotesLinkTitle::new(opening, Vec::new()))
                    } else {
                        IngestResult::Ready(Self::Building(opening, Vec::new()))
                    }
                }
                Err(segment) => IngestResult::Failure(vec![segment]),
            },
            Self::Building(opening, mut continuation_segments) => {
                match DoubleQuotesLinkTitleContinuationSegment::try_from(segment) {
                    Ok(continuation) => {
                        if continuation.is_closing() {
                            continuation_segments.push(continuation);
                            IngestResult::Success(DoubleQuotesLinkTitle::new(
                                opening,
                                continuation_segments,
                            ))
                        } else {
                            continuation_segments.push(continuation);
                            IngestResult::Ready(Self::Building(opening, continuation_segments))
                        }
                    }
                    Err(segment) => {
                        let mut segments = iter::once(opening.segment)
                            .chain(
                                continuation_segments
                                    .into_iter()
                                    .map(|continuation| continuation.segment),
                            )
                            .collect::<Vec<_>>();
                        segments.push(segment);
                        IngestResult::Failure(segments)
                    }
                }
            }
        }
    }
}

impl<'a> Finalize for DoubleQuotesLinkTitleParser<'a> {
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
    use super::*;
    use crate::{Segment, StrExt};

    #[test]
    fn idle_reject_empty() {
        let parser = DoubleQuotesLinkTitleParser::new();
        let segment = Segment::default();
        assert_eq!(parser.ingest(segment), IngestResult::Failure(vec![segment]));
    }

    #[test]
    fn idle_reject_invalid() {
        let parser = DoubleQuotesLinkTitleParser::new();
        let segment = Segment::default();
        assert_eq!(parser.ingest(segment), IngestResult::Failure(vec![segment]));
    }

    #[test]
    fn idle_finalize() {
        let parser = DoubleQuotesLinkTitleParser::new();
        assert_eq!(parser.finalize(), Err(Vec::new()));
    }

    #[test]
    fn idle_success() {
        let parser = DoubleQuotesLinkTitleParser::new();
        let segment = Segment::first("\"the word is the bird\"");
        assert_eq!(
            parser.ingest(segment).unwrap_success(),
            DoubleQuotesLinkTitle::new(
                DoubleQuotesLinkTitleOpeningSegment::try_from(segment).unwrap(),
                Vec::new()
            )
        );
    }

    #[test]
    fn idle_building_success() {
        let segments = "\"the word\nis the bird\""
            .line_segments()
            .collect::<Vec<_>>();
        let parser = DoubleQuotesLinkTitleParser::new();
        let parser = parser.ingest(segments[0]).unwrap_ready();
        assert_eq!(
            parser.ingest(segments[1]).unwrap_success(),
            DoubleQuotesLinkTitle::new(
                DoubleQuotesLinkTitleOpeningSegment::try_from(segments[0]).unwrap(),
                vec![DoubleQuotesLinkTitleContinuationSegment::try_from(segments[1]).unwrap()]
            )
        );
    }

    #[test]
    fn idle_building_reject() {
        let segments = "\"the word\n\n".line_segments().collect::<Vec<_>>();
        let parser = DoubleQuotesLinkTitleParser::new();
        let parser = parser.ingest(segments[0]).unwrap_ready();
        assert_eq!(parser.ingest(segments[1]).unwrap_failure(), segments);
    }

    #[test]
    fn idle_building_finalize() {
        let segment = Segment::first("\"the word\n");
        let parser = DoubleQuotesLinkTitleParser::new();
        let parser = parser.ingest(segment).unwrap_ready();
        assert_eq!(parser.finalize(), Err(vec![segment]));
    }
}
