use std::{
    iter::{once, Chain, Once},
    sync::LazyLock,
};

use crate::{
    internal::parse::try_extract::{Extraction, TryExtract},
    IntoSegments, Segment,
};

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
    type Item = Segment<'a>;

    fn next(&mut self) -> Option<Segment<'a>> {
        match self.opening_segment.take() {
            Some(opening) => Some(opening.into()),
            None => self.continuations.next().map(|c| c.into()),
        }
    }
}

// TODO: into_iter?
impl<'a> IntoSegments<'a> for ParagraphSegments<'a> {
    type IntoIter = ParagraphSegmentsIter<'a>;

    fn into_segments(self) -> Self::IntoIter {
        self.into()
    }
}

impl<'a> From<ParagraphOpeningSegment<'a>> for ParagraphSegments<'a> {
    fn from(opening: ParagraphOpeningSegment<'a>) -> Self {
        Self::new(opening, Vec::new())
    }
}

impl<'a, I> TryExtract<I> for ParagraphSegments<'a>
where
    I: Iterator<Item = Segment<'a>>,
{
    type Remaining = Option<Chain<Once<Segment<'a>>, I>>;
    type Error = Option<Segment<'a>>;

    fn try_extract(
        mut segments: I,
    ) -> Result<Extraction<Self, Option<Chain<Once<Segment<'a>>, I>>>, Self::Error> {
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
pub struct ParagraphOpeningSegment<'a>(pub Segment<'a>);

impl<'a> ParagraphOpeningSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> From<ParagraphOpeningSegment<'a>> for Segment<'a> {
    fn from(paragraph_opening: ParagraphOpeningSegment<'a>) -> Self {
        paragraph_opening.0
    }
}

// Paragraphs require at least one non whitespace character.
static OPENING_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[ ]{0,3}\S+.*?\s+$").unwrap());

impl<'a> TryFrom<Segment<'a>> for ParagraphOpeningSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
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
pub struct ParagraphContinuationSegment<'a>(pub Segment<'a>);

impl<'a> ParagraphContinuationSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> From<ParagraphContinuationSegment<'a>> for Segment<'a> {
    fn from(paragraph_continuation: ParagraphContinuationSegment<'a>) -> Self {
        paragraph_continuation.0
    }
}

static CONTINUATION_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^\s*\S.*\n$").unwrap());

impl<'a> TryFrom<Segment<'a>> for ParagraphContinuationSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
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

        failure_case!(should_reject_empty, Segment::default());
        failure_case!(should_reject_blank_line, Segment::first("\n"));
        failure_case!(should_reject_4_indents, Segment::first("    Hello\n"));
        failure_case!(
            should_reject_missing_ending_newline,
            Segment::first("Hello")
        );

        success_case!(should_work_with_valid_phrase, Segment::first("Hello\n"));
        success_case!(should_work_with_3_indents, Segment::first("   Hello\n"));
    }

    mod continuation {
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

        failure_case!(should_reject_empty, Segment::default());
        failure_case!(should_reject_blank_line, Segment::first("\n"));
        failure_case!(
            should_reject_missing_ending_newline,
            Segment::first("Hello")
        );

        success_case!(should_work_with_valid_phrase, Segment::first("Hello\n"));
        success_case!(should_work_with_4_indents, Segment::first("    Hello\n"));
        success_case!(should_work_with_tab_indent, Segment::first("\tHello\n"));
    }
}
