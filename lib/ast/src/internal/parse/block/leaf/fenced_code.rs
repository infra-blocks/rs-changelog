use crate::{
    internal::parse::{
        parser::{Finalize, Ingest, IngestResult},
        segment::{
            BackticksFencedCodeClosingSegment, BackticksFencedCodeOpeningSegment,
            TildesFencedCodeClosingSegment, TildesFencedCodeOpeningSegment,
        },
    },
    Segment,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FencedCode<'a> {
    Backticks(BackticksFencedCode<'a>),
    Tildes(TildesFencedCode<'a>),
}

impl<'a> FencedCode<'a> {
    pub fn backticks(fenced_code: BackticksFencedCode<'a>) -> Self {
        Self::Backticks(fenced_code)
    }

    pub fn tildes(fenced_code: TildesFencedCode<'a>) -> Self {
        Self::Tildes(fenced_code)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackticksFencedCode<'a> {
    pub opening_segment: BackticksFencedCodeOpeningSegment<'a>,
    pub content_segments: Vec<Segment<'a>>,
    /// The closing segment is allowed to be None in one scenario: when the end of input is reached
    /// before a closing segment. This is allowed by the spec.
    pub closing_segment: Option<BackticksFencedCodeClosingSegment<'a>>,
}

impl<'a> BackticksFencedCode<'a> {
    fn new(
        opening_segment: BackticksFencedCodeOpeningSegment<'a>,
        content_segments: Vec<Segment<'a>>,
        closing_segment: Option<BackticksFencedCodeClosingSegment<'a>>,
    ) -> Self {
        Self {
            opening_segment,
            content_segments,
            closing_segment,
        }
    }

    fn with_closing_segment(
        opening_segment: BackticksFencedCodeOpeningSegment<'a>,
        content_segments: Vec<Segment<'a>>,
        closing_segment: BackticksFencedCodeClosingSegment<'a>,
    ) -> Self {
        Self::new(opening_segment, content_segments, Some(closing_segment))
    }

    fn without_closing_segment(
        opening_segment: BackticksFencedCodeOpeningSegment<'a>,
        content_segments: Vec<Segment<'a>>,
    ) -> Self {
        Self::new(opening_segment, content_segments, None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TildesFencedCode<'a> {
    pub opening_segment: TildesFencedCodeOpeningSegment<'a>,
    pub content_segments: Vec<Segment<'a>>,
    pub closing_segment: Option<TildesFencedCodeClosingSegment<'a>>,
}

impl<'a> TildesFencedCode<'a> {
    fn new(
        opening_segment: TildesFencedCodeOpeningSegment<'a>,
        content_segments: Vec<Segment<'a>>,
        closing_segment: Option<TildesFencedCodeClosingSegment<'a>>,
    ) -> Self {
        Self {
            opening_segment,
            content_segments,
            closing_segment,
        }
    }

    fn with_closing_segment(
        opening_segment: TildesFencedCodeOpeningSegment<'a>,
        content_segments: Vec<Segment<'a>>,
        closing_segment: TildesFencedCodeClosingSegment<'a>,
    ) -> Self {
        Self::new(opening_segment, content_segments, Some(closing_segment))
    }

    fn without_closing_segment(
        opening_segment: TildesFencedCodeOpeningSegment<'a>,
        content_segments: Vec<Segment<'a>>,
    ) -> Self {
        Self::new(opening_segment, content_segments, None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FencedCodeParser<'a> {
    Idle,
    Backticks(BackticksFencedCodeOpeningSegment<'a>, Vec<Segment<'a>>),
    Tildes(TildesFencedCodeOpeningSegment<'a>, Vec<Segment<'a>>),
}

impl<'a> Default for FencedCodeParser<'a> {
    fn default() -> Self {
        Self::Idle
    }
}

impl<'a> FencedCodeParser<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> Ingest for FencedCodeParser<'a> {
    type Input = Segment<'a>;
    type Ready = Self;
    type Success = FencedCode<'a>;
    type Failure = Segment<'a>;

    fn ingest(
        self,
        segment: Self::Input,
    ) -> crate::internal::parse::parser::IngestResult<Self::Ready, Self::Success, Self::Failure>
    {
        match self {
            FencedCodeParser::Idle => match BackticksFencedCodeOpeningSegment::try_from(segment) {
                Ok(opening) => {
                    IngestResult::Ready(FencedCodeParser::Backticks(opening, Vec::new()))
                }
                Err(segment) => match TildesFencedCodeOpeningSegment::try_from(segment) {
                    Ok(opening) => {
                        IngestResult::Ready(FencedCodeParser::Tildes(opening, Vec::new()))
                    }
                    Err(segment) => return IngestResult::Failure(segment),
                },
            },
            FencedCodeParser::Backticks(opening, mut content) => {
                // If the segment is a closing segment, we are done. Otherwise, we keep building!
                match BackticksFencedCodeClosingSegment::try_from(segment) {
                    Ok(closing) => {
                        // It is only a closing segment if the fence length is at least as long as the opening's.
                        if closing.fence_length >= opening.fence_length {
                            IngestResult::Success(FencedCode::backticks(
                                BackticksFencedCode::with_closing_segment(
                                    opening, content, closing,
                                ),
                            ))
                        // Otherwise, it will be treated as a regular content segment.
                        } else {
                            let segment = closing.segment;
                            content.push(segment);
                            IngestResult::Ready(FencedCodeParser::Backticks(opening, content))
                        }
                    }
                    Err(segment) => IngestResult::Ready(FencedCodeParser::Backticks(opening, {
                        let mut content = content;
                        content.push(segment);
                        content
                    })),
                }
            }
            FencedCodeParser::Tildes(opening, mut content) => {
                match TildesFencedCodeClosingSegment::try_from(segment) {
                    Ok(closing) => {
                        // It is only a closing segment if the fence length is at least as long as the opening's.
                        if closing.fence_length >= opening.fence_length {
                            IngestResult::Success(FencedCode::tildes(
                                TildesFencedCode::with_closing_segment(opening, content, closing),
                            ))
                        // Otherwise, it will be treated as a regular content segment.
                        } else {
                            let segment = closing.segment;
                            content.push(segment);
                            IngestResult::Ready(FencedCodeParser::Tildes(opening, content))
                        }
                    }
                    Err(segment) => IngestResult::Ready(FencedCodeParser::Tildes(opening, {
                        let mut content = content;
                        content.push(segment);
                        content
                    })),
                }
            }
        }
    }
}

impl<'a> Finalize for FencedCodeParser<'a> {
    type Result = Option<FencedCode<'a>>;

    fn finalize(self) -> Self::Result {
        match self {
            FencedCodeParser::Idle => None,
            FencedCodeParser::Backticks(opening, content) => Some(FencedCode::backticks(
                BackticksFencedCode::without_closing_segment(opening, content),
            )),
            FencedCodeParser::Tildes(openning, content) => Some(FencedCode::tildes(
                TildesFencedCode::without_closing_segment(openning, content),
            )),
        }
    }
}

// TODO: tests, then remove the garbage under, then implement the rendering.
#[cfg(test)]
mod test {
    use super::*;

    // Test state transitions.
    mod parser {
        use super::*;

        #[test]
        fn idle_invalid() {}
        #[test]
        fn idle_backticks_finalize() {}
        #[test]
        fn idle_backticks_success() {}
        #[test]
        fn idle_tildes_finalize() {}
        #[test]
        fn idle_tildes_success() {}
        // TODO: test that a smaller fence cannot close a larger fence.
    }
}
