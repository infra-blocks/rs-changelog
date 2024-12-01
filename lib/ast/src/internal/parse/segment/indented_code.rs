use std::sync::LazyLock;

use crate::Segment;

use super::BlankLineSegment;

/// An indented code segment.
///
/// An indented code segment is one that starts with 4 spaces or a tab and
/// isn't a blank line segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndentedCodeSegment<'a>(pub Segment<'a>);

impl<'a> IndentedCodeSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> From<IndentedCodeSegment<'a>> for Segment<'a> {
    fn from(segment: IndentedCodeSegment<'a>) -> Self {
        segment.0
    }
}

// Every line needs to start with at least 4 spaces.
static REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^(?:(?:[ ]{4})|(?:\t))(?:\s*\S.*\n?)$").unwrap());

impl<'a> TryFrom<Segment<'a>> for IndentedCodeSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if REGEX.is_match(segment.text()) {
            Ok(Self::new(segment))
        } else {
            Err(segment)
        }
    }
}

/// An enum representing either an indented code segment or a blank line segment.
///
/// This is useful in the context of building an indented code block, as it can
/// contain blank lines.
///
/// # Note
/// Only non trailing blank lines should be kept in the block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndentedCodeOrBlankLineSegment<'a> {
    IndentedCode(IndentedCodeSegment<'a>),
    BlankLine(BlankLineSegment<'a>),
}

impl<'a> From<IndentedCodeSegment<'a>> for IndentedCodeOrBlankLineSegment<'a> {
    fn from(segment: IndentedCodeSegment<'a>) -> Self {
        Self::IndentedCode(segment)
    }
}

impl<'a> From<BlankLineSegment<'a>> for IndentedCodeOrBlankLineSegment<'a> {
    fn from(segment: BlankLineSegment<'a>) -> Self {
        Self::BlankLine(segment)
    }
}

impl<'a> TryFrom<Segment<'a>> for IndentedCodeOrBlankLineSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if let Ok(segment) = IndentedCodeSegment::try_from(segment) {
            Ok(IndentedCodeOrBlankLineSegment::IndentedCode(segment))
        } else if let Ok(segment) = BlankLineSegment::try_from(segment) {
            Ok(IndentedCodeOrBlankLineSegment::BlankLine(segment))
        } else {
            Err(segment)
        }
    }
}

impl<'a> From<IndentedCodeOrBlankLineSegment<'a>> for Segment<'a> {
    fn from(value: IndentedCodeOrBlankLineSegment<'a>) -> Self {
        match value {
            IndentedCodeOrBlankLineSegment::IndentedCode(segment) => segment.into(),
            IndentedCodeOrBlankLineSegment::BlankLine(segment) => segment.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod indented_code_segment {
        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        IndentedCodeSegment::try_from($segment.clone()),
                        Err($segment)
                    )
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        IndentedCodeSegment::try_from($segment.clone()),
                        Ok(IndentedCodeSegment::new($segment))
                    )
                }
            };
        }

        failure_case!(should_reject_empty_segment, Segment::default());
        failure_case!(should_reject_blank_line, Segment::first(" \n"));
        failure_case!(
            should_reject_3_whitespaces_indent,
            Segment::first("   Missing one space\n")
        );

        success_case!(
            should_work_with_4_whitespaces_indent,
            Segment::first("    This is indented code. Finally.\n")
        );
        success_case!(
            should_work_with_tab_indent,
            Segment::first("\tThis is indented code. Finally.\n")
        );
        success_case!(
            should_work_with_missing_eol,
            Segment::first("    This is indented code. Finally.")
        );
    }

    // Test that it can accept an indented code or a blank line.
    mod indented_code_or_blank_line_segment {
        use super::*;

        #[test]
        fn should_reject_empty_segment() {
            assert_eq!(
                IndentedCodeOrBlankLineSegment::try_from(Segment::default()),
                Err(Segment::default())
            )
        }

        #[test]
        fn should_work_with_single_char_blank_line() {
            let segment = Segment::first(" \n");
            assert_eq!(
                IndentedCodeOrBlankLineSegment::try_from(segment),
                Ok(IndentedCodeOrBlankLineSegment::BlankLine(
                    segment.try_into().unwrap()
                ))
            )
        }

        #[test]
        fn should_work_with_indented_code() {
            let segment = Segment::first("    This is indented code.\n");
            assert_eq!(
                IndentedCodeOrBlankLineSegment::try_from(segment),
                Ok(IndentedCodeOrBlankLineSegment::IndentedCode(
                    segment.try_into().unwrap()
                ))
            )
        }
    }
}
