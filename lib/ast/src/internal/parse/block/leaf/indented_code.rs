use crate::internal::parse::{
    parser::{Finalize, Ingest, IngestResult},
    segment::{BlankLineSegment, IndentedCodeOrBlankLineSegment, IndentedCodeSegment},
};
use segment::LineSegment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentedCode<'a> {
    pub opening_segment: IndentedCodeSegment<'a>,
    // The last segment is guaranteed not to be a blank line segment.
    pub continuation_segments: Option<ContinuationSegments<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuationSegments<'a> {
    // Could be empty.
    pub segments: Vec<IndentedCodeOrBlankLineSegment<'a>>,
    pub closing_segment: IndentedCodeSegment<'a>,
}

impl<'a> ContinuationSegments<'a> {
    fn new(
        segments: Vec<IndentedCodeOrBlankLineSegment<'a>>,
        closing_segment: IndentedCodeSegment<'a>,
    ) -> Self {
        Self {
            segments,
            closing_segment,
        }
    }
}

impl<'a> IndentedCode<'a> {
    fn new(
        opening_segment: IndentedCodeSegment<'a>,
        continuation_segments: Option<ContinuationSegments<'a>>,
    ) -> Self {
        Self {
            opening_segment,
            continuation_segments,
        }
    }

    fn single_segment(opening_segment: IndentedCodeSegment<'a>) -> Self {
        Self::new(opening_segment, None)
    }

    fn multi_segments(
        opening_segment: IndentedCodeSegment<'a>,
        continuation_segments: ContinuationSegments<'a>,
    ) -> Self {
        Self::new(opening_segment, Some(continuation_segments))
    }
}

// TODO: merge the opened and continuing states. An empty vec does not allocate.
// Blank lines at the end need to be stripped off as they are not part of the indented code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum IndentedCodeParser<'a> {
    Idle,
    Opened(IndentedCodeSegment<'a>),
    Continuing(
        IndentedCodeSegment<'a>,
        Vec<IndentedCodeOrBlankLineSegment<'a>>,
    ),
}

impl<'a> Default for IndentedCodeParser<'a> {
    fn default() -> Self {
        Self::Idle
    }
}

impl<'a> IndentedCodeParser<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

// Stripped off trailing blank lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentedCodeStripResult<'a> {
    pub indented_code: IndentedCode<'a>,
    pub trailing_blank_lines: Vec<BlankLineSegment<'a>>,
}

impl<'a> IndentedCodeStripResult<'a> {
    fn new(
        indented_code: IndentedCode<'a>,
        trailing_blank_lines: Vec<BlankLineSegment<'a>>,
    ) -> Self {
        Self {
            indented_code,
            trailing_blank_lines,
        }
    }

    fn from_single_segment(opening_segment: IndentedCodeSegment<'a>) -> Self {
        Self::new(IndentedCode::single_segment(opening_segment), Vec::new())
    }

    fn from_multi_segments(
        opening_segment: IndentedCodeSegment<'a>,
        mut continuation_segments: Vec<IndentedCodeOrBlankLineSegment<'a>>,
    ) -> IndentedCodeStripResult<'a> {
        let Some(closing_segment) = continuation_segments.pop() else {
            return Self::from_single_segment(opening_segment);
        };

        match closing_segment {
            IndentedCodeOrBlankLineSegment::IndentedCode(indented_code_segment) => {
                let closing_segment = indented_code_segment;
                let indented_code = IndentedCode::multi_segments(
                    opening_segment,
                    ContinuationSegments::new(continuation_segments, closing_segment),
                );
                Self::new(indented_code, Vec::new())
            }
            IndentedCodeOrBlankLineSegment::BlankLine(blank_line_segment) => {
                // We are constructing it in reverse order. The final result will have to be reversed before
                // returning.
                let mut trailing_blank_lines = vec![blank_line_segment];
                // Pop until we reach a non-blank line.
                loop {
                    match continuation_segments.pop() {
                        Some(last_segment) => {
                            match last_segment {
                                IndentedCodeOrBlankLineSegment::BlankLine(blank_line_segment) => {
                                    trailing_blank_lines.push(blank_line_segment);
                                }
                                // We stop at the first non blank line segment.
                                IndentedCodeOrBlankLineSegment::IndentedCode(
                                    indented_code_segment,
                                ) => {
                                    let closing_segment = indented_code_segment;
                                    let indented_code = IndentedCode::multi_segments(
                                        opening_segment,
                                        ContinuationSegments::new(
                                            continuation_segments,
                                            closing_segment,
                                        ),
                                    );
                                    trailing_blank_lines.reverse();
                                    return Self::new(indented_code, trailing_blank_lines);
                                }
                            }
                        }
                        None => {
                            trailing_blank_lines.reverse();
                            return Self::new(
                                IndentedCode::single_segment(opening_segment),
                                trailing_blank_lines,
                            );
                        }
                    }
                }
            }
        }
    }
}

/// The new segment was rejected, but we were able to construct a block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct IngestSuccess<'a> {
    pub result: IndentedCodeStripResult<'a>,
    // Interrupting segment.
    pub rejected_segment: LineSegment<'a>,
}

impl<'a> IngestSuccess<'a> {
    fn new(result: IndentedCodeStripResult<'a>, rejected_segment: LineSegment<'a>) -> Self {
        Self {
            result,
            rejected_segment,
        }
    }
}

impl<'a> Ingest for IndentedCodeParser<'a> {
    type Input = LineSegment<'a>;
    type Ready = Self;
    type Success = IngestSuccess<'a>;
    type Failure = LineSegment<'a>;

    fn ingest(self, input: Self::Input) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        match self {
            Self::Idle => match IndentedCodeSegment::try_from(input) {
                Ok(segment) => IngestResult::Ready(Self::Opened(segment)),
                Err(segment) => IngestResult::Failure(segment),
            },
            Self::Opened(opening_segment) => {
                match IndentedCodeOrBlankLineSegment::try_from(input) {
                    Ok(continuation_segment) => IngestResult::Ready(Self::Continuing(
                        opening_segment,
                        vec![continuation_segment],
                    )),
                    Err(segment) => IngestResult::Success(IngestSuccess::new(
                        IndentedCodeStripResult::from_single_segment(opening_segment),
                        segment,
                    )),
                }
            }
            Self::Continuing(opening_segment, mut continuation_segments) => {
                match IndentedCodeOrBlankLineSegment::try_from(input) {
                    Ok(segment) => {
                        continuation_segments.push(segment);
                        IngestResult::Ready(Self::Continuing(
                            opening_segment,
                            continuation_segments,
                        ))
                    }
                    Err(segment) => IngestResult::Success(IngestSuccess::new(
                        IndentedCodeStripResult::from_multi_segments(
                            opening_segment,
                            continuation_segments,
                        ),
                        segment,
                    )),
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FinalizeResult<'a> {
    Nothing,
    Success(IndentedCodeStripResult<'a>),
}

impl<'a> Finalize for IndentedCodeParser<'a> {
    type Result = FinalizeResult<'a>;

    fn finalize(self) -> Self::Result {
        match self {
            Self::Idle => FinalizeResult::Nothing,
            Self::Opened(opening_segment) => FinalizeResult::Success(
                IndentedCodeStripResult::from_single_segment(opening_segment),
            ),
            Self::Continuing(opening_segment, continuation_segments) => {
                FinalizeResult::Success(IndentedCodeStripResult::from_multi_segments(
                    opening_segment,
                    continuation_segments,
                ))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use segment::SegmentStrExt;

    // Tests that it properly strips off trailing blank lines when present.
    mod indented_code_stripping {
        use super::*;

        impl<'a> From<Vec<LineSegment<'a>>> for IndentedCodeStripResult<'a> {
            /// An implementation to easily generate a result from segments for testing.
            ///
            /// # Panics
            /// - If the segments are empty.
            /// - If the first segment is not an [IndentedCodeSegment].
            /// - If any of the subsequent segment cannot be turned into [IndentedCodeOrBlankLineSegment].
            fn from(segments: Vec<LineSegment<'a>>) -> Self {
                if segments.is_empty() {
                    panic!("Segments cannot be empty.");
                }
                let mut iter = segments.into_iter();
                let opening_segment = iter
                    .next()
                    .unwrap()
                    .try_into()
                    .expect("opening segment is not an indented code segment");
                let continuation_segments = iter
                    .map(|segment| {
                        segment.try_into().expect(
                            "in between segment is not an indented code or blank line segment",
                        )
                    })
                    .collect();
                Self::from_multi_segments(opening_segment, continuation_segments)
            }
        }

        impl<'a> From<Vec<LineSegment<'a>>> for IndentedCode<'a> {
            /// This implementation is just to generate a value to compare against for tests.
            ///
            /// # Panics
            /// - If the segments are empty.
            /// - If the first segment cannot be turned into an [IndentedCodeSegment].
            /// - If the last segment cannot be turned into an [IndentedCodeSegment].
            /// - If any of the segments in between cannot be turned into an [IndentedCodeOrBlankLineSegment].
            fn from(mut segments: Vec<LineSegment<'a>>) -> Self {
                if segments.is_empty() {
                    panic!("Segments cannot be empty.");
                }
                let closing_segment = segments
                    .pop()
                    .unwrap()
                    .try_into()
                    .expect("opening segment is not an indented code segment");
                // If there was only one segment, then the closing segment is also the opening segment.
                if segments.is_empty() {
                    return IndentedCode::single_segment(closing_segment);
                }
                // Otherwise, we had at least two segments. So the first one needs to be an opening segment.
                let mut iter = segments.into_iter();
                let opening_segment = iter
                    .next()
                    .unwrap()
                    .try_into()
                    .expect("opening segment is not an indented code segment");
                let in_between = iter
                    .map(|segment| {
                        segment.try_into().expect(
                            "in between segment is not an indented code or blank line segment",
                        )
                    })
                    .collect();

                return IndentedCode::multi_segments(
                    opening_segment,
                    ContinuationSegments::new(in_between, closing_segment),
                );
            }
        }

        #[test]
        fn should_work_with_opening_segment() {
            let opening_segment = "    This is indented code.\n".line().try_into().unwrap();
            assert_eq!(
                IndentedCodeStripResult::from_single_segment(opening_segment),
                IndentedCodeStripResult::new(
                    IndentedCode::single_segment(opening_segment),
                    Vec::new()
                )
            )
        }

        #[test]
        fn should_work_with_multi_segments_but_empty() {
            let opening_segment = "    This is indented code.\n".line().try_into().unwrap();
            assert_eq!(
                IndentedCodeStripResult::from_multi_segments(opening_segment, Vec::new()),
                IndentedCodeStripResult::new(
                    IndentedCode::single_segment(opening_segment),
                    Vec::new()
                )
            )
        }

        #[test]
        fn should_work_with_multi_segments() {
            let segments: Vec<_> = r"    This is indented code
    and so is this
    and this"
                .lines()
                .collect();
            assert_eq!(
                IndentedCodeStripResult::from(segments.clone()),
                IndentedCodeStripResult::new(segments.into(), Vec::new())
            )
        }

        #[test]
        fn should_not_strip_off_trailing_lines_if_the_last_one_is_not_blank() {
            let segments: Vec<_> = r"    This is indented code mixed with blank lines

    One blank line above


    Two blank lines above"
                .lines()
                .collect();
            assert_eq!(
                IndentedCodeStripResult::from(segments.clone()),
                IndentedCodeStripResult::new(segments.into(), Vec::new())
            )
        }

        #[test]
        fn should_correctly_strip_off_one_blank_line() {
            let segments: Vec<_> = r"    This is indented code ending with a single blank line

    Hello

    Big code
"
            .lines()
            .collect();
            let mut indented_code_segments = segments.clone();
            let trailing_blank_lines = indented_code_segments
                .split_off(5)
                .into_iter()
                .map(|segment| BlankLineSegment::try_from(segment).unwrap())
                .collect();
            assert_eq!(
                IndentedCodeStripResult::from(segments),
                IndentedCodeStripResult::new(indented_code_segments.into(), trailing_blank_lines)
            )
        }

        #[test]
        fn should_correctly_strip_off_many_blank_lines() {
            let segments: Vec<_> = r"    This is indented code ending with several blank lines

    Hello, is it me you are looking for?


    "
            .lines()
            .collect();
            let mut indented_code_segments = segments.clone();
            let trailing_blank_lines = indented_code_segments
                .split_off(3)
                .into_iter()
                .map(|segment| BlankLineSegment::try_from(segment).unwrap())
                .collect();
            assert_eq!(
                IndentedCodeStripResult::from(segments),
                IndentedCodeStripResult::new(indented_code_segments.into(), trailing_blank_lines)
            )
        }
    }

    // Test state transitions. We won't be testing every state transition with every type of
    // result stripping here. We just go for the happy path in every state transition.
    mod parser {
        use segment::SegmentStrExt;

        use super::*;

        #[test]
        fn idle_to_invalid() {
            let parser = IndentedCodeParser::new();
            let segment = LineSegment::default();
            assert_eq!(parser.ingest(segment), IngestResult::Failure(segment));
        }

        #[test]
        fn idle_to_finalize() {
            let parser = IndentedCodeParser::new();
            assert_eq!(parser.finalize(), FinalizeResult::Nothing);
        }

        // TODO: for testing, there should be a utility function to run line segments through the parser.
        #[test]
        fn idle_opened_interrupted() {
            let parser = IndentedCodeParser::new();

            let first_segment = "    first line\n".line();
            let parser = parser.ingest(first_segment).unwrap_ready();

            let second_segment = first_segment.next("```rust\n");
            assert_eq!(
                parser.ingest(second_segment),
                IngestResult::Success(IngestSuccess::new(
                    IndentedCodeStripResult::from_single_segment(first_segment.try_into().unwrap()),
                    second_segment
                ))
            );
        }

        #[test]
        fn idle_opened_finalize() {
            let parser = IndentedCodeParser::new();

            let first_segment = "    first line\n".line();
            let parser = parser.ingest(first_segment).unwrap_ready();

            assert_eq!(
                parser.finalize(),
                FinalizeResult::Success(IndentedCodeStripResult::from_single_segment(
                    first_segment.try_into().unwrap()
                ))
            );
        }

        #[test]
        fn idle_opened_continuing_interrupted() {
            let parser = IndentedCodeParser::new();

            let first_segment = "    first line\n".line();
            let parser = parser.ingest(first_segment).unwrap_ready();

            let second_segment = first_segment.next("    second line\n");
            let parser = parser.ingest(second_segment).unwrap_ready();

            let third_segment = second_segment.next("```rust\n");
            assert_eq!(
                parser.ingest(third_segment),
                IngestResult::Success(IngestSuccess::new(
                    IndentedCodeStripResult::from_multi_segments(
                        first_segment.try_into().unwrap(),
                        vec![second_segment.try_into().unwrap()]
                    ),
                    third_segment
                ))
            );
        }

        #[test]
        fn idle_opened_continuing_finalize() {
            let parser = IndentedCodeParser::new();

            let first_segment = "    first line\n".line();
            let parser = parser.ingest(first_segment).unwrap_ready();

            let second_segment = first_segment.next("    second line\n");
            let parser = parser.ingest(second_segment).unwrap_ready();

            assert_eq!(
                parser.finalize(),
                FinalizeResult::Success(IndentedCodeStripResult::from_multi_segments(
                    first_segment.try_into().unwrap(),
                    vec![second_segment.try_into().unwrap()]
                ))
            );
        }
    }
}
