use crate::internal::parse::segment::{ParagraphSegments, SetextHeadingUnderlineSegment};
use segment::LineSegment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphSegmentsAndNext<'a> {
    pub paragraph_segments: ParagraphSegments<'a>,
    pub next_segment: LineSegment<'a>,
}

impl<'a> ParagraphSegmentsAndNext<'a> {
    pub fn new(paragraph_segments: ParagraphSegments<'a>, next_segment: LineSegment<'a>) -> Self {
        Self {
            paragraph_segments,
            next_segment,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetextHeading<'a> {
    pub paragraph_segments: ParagraphSegments<'a>,
    pub underline_segment: SetextHeadingUnderlineSegment<'a>,
}

impl<'a> SetextHeading<'a> {
    pub fn new(
        paragraph_segments: ParagraphSegments<'a>,
        underline_segment: SetextHeadingUnderlineSegment<'a>,
    ) -> Self {
        Self {
            paragraph_segments,
            underline_segment,
        }
    }

    pub fn level(&self) -> u8 {
        self.underline_segment.level()
    }
}

// TODO: maybe this try from is overkill? We could just do it outside.
impl<'a> TryFrom<ParagraphSegmentsAndNext<'a>> for SetextHeading<'a> {
    type Error = ParagraphSegmentsAndNext<'a>;

    fn try_from(value: ParagraphSegmentsAndNext<'a>) -> Result<Self, Self::Error> {
        let underline_segment = SetextHeadingUnderlineSegment::try_from(value.next_segment);
        match underline_segment {
            Ok(underline_segment) => Ok(Self::new(value.paragraph_segments, underline_segment)),
            Err(_) => Err(value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from {
        use crate::internal::parse::try_extract::TryExtract;
        use segment::{Segment, SegmentStrExt};

        use super::*;

        macro_rules! failure_case {
            ($name:ident, $paragraph_segments:expr, $next_segment:expr) => {
                #[test]
                fn $name() {
                    let extract = ParagraphSegments::try_extract($paragraph_segments).unwrap();
                    let paragraph_segments = extract.extracted;
                    assert!(extract.remaining.is_none());

                    let conversion_params =
                        ParagraphSegmentsAndNext::new(paragraph_segments, $next_segment);

                    assert_eq!(
                        SetextHeading::try_from(conversion_params.clone()),
                        Err(conversion_params)
                    );
                }
            };
        }

        macro_rules! success_case {
            ($name:ident, $all_segments:expr, $expected_level:expr) => {
                #[test]
                fn $name() {
                    let all_segments: Vec<_> = $all_segments.collect();
                    let mut paragraph_segments: Vec<_> = all_segments.clone();
                    let next_segment = paragraph_segments.pop().unwrap();
                    let extract =
                        ParagraphSegments::try_extract(paragraph_segments.into_iter()).unwrap();
                    assert!(extract.remaining.is_none());
                    let conversion_params =
                        ParagraphSegmentsAndNext::new(extract.extracted.clone(), next_segment);
                    let setext_heading = SetextHeading::try_from(conversion_params).unwrap();

                    assert_eq!(setext_heading.level(), $expected_level);
                    assert_eq!(
                        setext_heading,
                        SetextHeading::new(extract.extracted, next_segment.try_into().unwrap()),
                    );
                }
            };
        }

        failure_case!(
            should_reject_an_empty_segment,
            "aaa\n".lines(),
            LineSegment::new(Segment::empty_at(location::Position::new(5, 2, 4)))
        );
        failure_case!(
            should_reject_a_whitespace_segment,
            "aaa\n".lines(),
            LineSegment::new(Segment::new(location::Position::new(5, 2, 4), " \n"))
        );
        failure_case!(
            should_reject_an_underline_segment_with_other_characters,
            "aaa\n".lines(),
            LineSegment::new(Segment::new(location::Position::new(5, 2, 4), "===a\n"))
        );

        success_case!(should_work_with_equals_underline, "aaa\n===\n".lines(), 1);
        success_case!(should_work_with_hyphens_underline, "aaa\n---\n".lines(), 2);
        success_case!(
            should_work_with_a_single_character_underline,
            "aaa\n=\n".lines(),
            1
        );
        success_case!(
            should_work_with_many_characters_underline,
            "aaa\n============\n".lines(),
            1
        );
        success_case!(
            should_work_with_3_spaces_before_the_underline,
            "aaa\n   ===\n".lines(),
            1
        );
        success_case!(
            should_work_with_trailing_whitespaces_underline,
            "aaa\n===  \n".lines(),
            1
        );
        success_case!(
            should_work_with_multiline_paragraph,
            "aaa\n         hello this is a continuation line\n===\n".lines(),
            1
        );
    }
}

/* #[cfg(test)]
mod test {
    use crate::parser::Parser;

    use super::*;

    mod try_from {
        macro_rules! failure_case {
            ($name:ident, $paragraph_segments:expr, $next_segment:expr) => {
                #[test]
                fn $name() {
                    let paragraph = Paragraph::from($paragraph_segments);

                    assert_eq!(
                        SetextHeading::try_from((paragraph.clone(), $next_segment)),
                        Err(paragraph)
                    );
                }
            };
        }

        macro_rules! success_case {
            ($name:ident, $all_segments:expr, $expected_level:expr) => {
                #[test]
                fn $name() {
                    let all_segments: Vec<_> = $all_segments.collect();
                    let mut paragraph_segments: Vec<_> = all_segments.clone();
                    let setex_heading_segment = paragraph_segments.pop().unwrap();
                    let paragraph = Paragraph::from(paragraph_segments);
                    let setext_heading =
                        SetextHeading::try_from((paragraph.clone(), setex_heading_segment))
                            .unwrap();

                    assert_eq!(setext_heading.level, $expected_level);
                    assert_eq!(
                        setext_heading.segments, all_segments,
                        "unexpected setex heading segments"
                    );
                }
            };
        }

        failure_case!(
            should_reject_an_empty_segment,
            "aaa\n".line_segments(),
            Segment::empty_at(location::Position::new(5, 2, 4))
        );
        failure_case!(
            should_reject_a_whitespace_segment,
            "aaa\n".line_segments(),
            Segment::new(location::Position::new(5, 2, 4), " \n")
        );
        failure_case!(
            should_reject_an_underline_segment_with_other_characters,
            "aaa\n".line_segments(),
            Segment::new(location::Position::new(5, 2, 4), "===a\n")
        );

        success_case!(
            should_work_with_equals_underline,
            "aaa\n===\n".line_segments(),
            1
        );
        success_case!(
            should_work_with_hyphens_underline,
            "aaa\n---\n".line_segments(),
            2
        );
        success_case!(
            should_work_with_a_single_character_underline,
            "aaa\n=\n".line_segments(),
            1
        );
        success_case!(
            should_work_with_many_characters_underline,
            "aaa\n============\n".line_segments(),
            1
        );
        success_case!(
            should_work_with_3_spaces_before_the_underline,
            "aaa\n   ===\n".line_segments(),
            1
        );
        success_case!(
            should_work_with_trailing_whitespaces_underline,
            "aaa\n===  \n".line_segments(),
            1
        );
        success_case!(
            should_work_with_multiline_paragraph,
            "aaa\n         hello this is a continuation line\n===\n".line_segments(),
            1
        );
    }
}
 */
