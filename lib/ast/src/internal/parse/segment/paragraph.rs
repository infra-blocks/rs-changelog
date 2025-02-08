use std::{
    iter::{once, Chain, Once},
    sync::LazyLock,
};

use crate::{
    internal::parse::try_extract::{Extraction, TryExtract},
    IntoLineSegments,
};

use segment::{LineSegment, SegmentLike};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphSegments<'a> {
    pub opening: ParagraphOpeningSegment<'a>,
    pub continuations: Vec<ParagraphContinuationSegment<'a>>,
}

impl<'a> ParagraphSegments<'a> {
    pub fn new(
        opening: ParagraphOpeningSegment<'a>,
        continuations: Vec<ParagraphContinuationSegment<'a>>,
    ) -> Self {
        Self {
            opening,
            continuations,
        }
    }
}

pub struct ParagraphSegmentsIter<'a> {
    opening_segment: Option<ParagraphOpeningSegment<'a>>,
    continuations: std::vec::IntoIter<ParagraphContinuationSegment<'a>>,
}

impl<'a> From<ParagraphSegments<'a>> for ParagraphSegmentsIter<'a> {
    fn from(paragraph_segments: ParagraphSegments<'a>) -> Self {
        Self {
            opening_segment: Some(paragraph_segments.opening),
            continuations: paragraph_segments.continuations.into_iter(),
        }
    }
}

impl<'a> Iterator for ParagraphSegmentsIter<'a> {
    type Item = LineSegment<'a>;

    fn next(&mut self) -> Option<LineSegment<'a>> {
        match self.opening_segment.take() {
            Some(opening) => Some(LineSegment::from(opening.0)),
            None => self
                .continuations
                .next()
                .map(|continuation| LineSegment::from(continuation.0)),
        }
    }
}

impl<'a> IntoLineSegments<'a> for ParagraphSegments<'a> {
    fn into_line_segments(self) -> impl Iterator<Item = LineSegment<'a>> {
        ParagraphSegmentsIter::from(self)
    }
}

impl<'a> From<ParagraphOpeningSegment<'a>> for ParagraphSegments<'a> {
    fn from(opening: ParagraphOpeningSegment<'a>) -> Self {
        Self::new(opening, Vec::new())
    }
}

impl<'a, I> TryExtract<I> for ParagraphSegments<'a>
where
    I: Iterator<Item = LineSegment<'a>>,
{
    type Remaining = Option<Chain<Once<LineSegment<'a>>, I>>;
    type Error = Option<LineSegment<'a>>;

    fn try_extract(
        mut segments: I,
    ) -> Result<Extraction<Self, Option<Chain<Once<LineSegment<'a>>, I>>>, Self::Error> {
        let Some(first) = segments.next() else {
            return Err(None);
        };

        let Ok(opening) = ParagraphOpeningSegment::try_from(first) else {
            return Err(Some(first));
        };

        let mut continuations = Vec::new();
        while let Some(segment) = segments.next() {
            match ParagraphContinuationSegment::try_from(segment) {
                Ok(continuation) => continuations.push(continuation),
                Err(segment) => {
                    let paragraph_segments = ParagraphSegments::new(opening, continuations);
                    return Ok(Extraction::new(
                        paragraph_segments,
                        Some(once(segment).chain(segments)),
                    ));
                }
            }
        }
        let segments = ParagraphSegments::new(opening, continuations);
        Ok(Extraction::new(segments, None))
    }
}

/// A paragraph opening segment.
///
/// Unlike [ParagraphContinuationSegment]s, this type cannot contain text with more than 3 leading whitespaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphOpeningSegment<'a>(pub LineSegment<'a>);

impl<'a> ParagraphOpeningSegment<'a> {
    fn new(segment: LineSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> From<ParagraphOpeningSegment<'a>> for LineSegment<'a> {
    fn from(paragraph_opening: ParagraphOpeningSegment<'a>) -> Self {
        paragraph_opening.0
    }
}

// Paragraphs require at least one non whitespace character.
static OPENING_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[ ]{0,3}\S+.*\n?$").unwrap());

impl<'a> TryFrom<LineSegment<'a>> for ParagraphOpeningSegment<'a> {
    type Error = LineSegment<'a>;

    fn try_from(segment: LineSegment<'a>) -> Result<Self, Self::Error> {
        if OPENING_REGEX.is_match(segment.text()) {
            Ok(Self::new(segment))
        } else {
            Err(segment)
        }
    }
}

/// A paragraph continuation segment.
///
/// Unlike [ParagraphOpeningSegment]s, this type can contain text with more than 3 leading whitespaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphContinuationSegment<'a>(pub LineSegment<'a>);

impl<'a> ParagraphContinuationSegment<'a> {
    fn new(segment: LineSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> From<ParagraphContinuationSegment<'a>> for LineSegment<'a> {
    fn from(paragraph_continuation: ParagraphContinuationSegment<'a>) -> Self {
        paragraph_continuation.0
    }
}

static CONTINUATION_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^\s*\S.*\n?$").unwrap());

// TODO: use regex on text_without_newline() or something? That would remove the need for all them optional \n at the end of the regexes.
impl<'a> TryFrom<LineSegment<'a>> for ParagraphContinuationSegment<'a> {
    type Error = LineSegment<'a>;

    fn try_from(segment: LineSegment<'a>) -> Result<Self, Self::Error> {
        if CONTINUATION_REGEX.is_match(&segment.text()) {
            Ok(Self::new(segment))
        } else {
            Err(segment)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod opening {
        use segment::SegmentStrExt;

        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        ParagraphOpeningSegment::try_from($segment.clone()),
                        Err($segment)
                    );
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        ParagraphOpeningSegment::try_from($segment.clone()),
                        Ok(ParagraphOpeningSegment::new($segment))
                    );
                }
            };
        }

        failure_case!(should_reject_empty, LineSegment::default());
        failure_case!(should_reject_blank_line, "\n".line());
        failure_case!(should_reject_4_indents, "    Hello\n".line());
        // TODO: this should pass now, as the paragraph can be opened on the last line and it can lack a newline.
        failure_case!(should_reject_missing_ending_newline, "Hello".line());

        success_case!(should_work_with_valid_phrase, "Hello\n".line());
        success_case!(should_work_with_3_indents, "   Hello\n".line());
    }

    mod continuation {
        use segment::SegmentStrExt;

        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        ParagraphContinuationSegment::try_from($segment.clone()),
                        Err($segment)
                    );
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        ParagraphContinuationSegment::try_from($segment.clone()),
                        Ok(ParagraphContinuationSegment::new($segment))
                    );
                }
            };
        }

        failure_case!(should_reject_empty, LineSegment::default());
        failure_case!(should_reject_blank_line, "\n".line());
        failure_case!(should_reject_missing_ending_newline, "Hello".line());

        success_case!(should_work_with_valid_phrase, "Hello\n".line());
        success_case!(should_work_with_4_indents, "    Hello\n".line());
        success_case!(should_work_with_tab_indent, "\tHello\n".line());
    }
}
