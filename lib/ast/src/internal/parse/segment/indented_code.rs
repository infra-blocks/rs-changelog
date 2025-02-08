use std::sync::LazyLock;

use segment::{LineSegment, SegmentLike};

use super::BlankLineSegment;

/// An indented code segment.
///
/// An indented code segment is one that starts with 4 spaces or a tab and
/// isn't a blank line segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndentedCodeSegment<'a>(pub LineSegment<'a>);

impl<'a> IndentedCodeSegment<'a> {
    fn new(segment: LineSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> From<IndentedCodeSegment<'a>> for LineSegment<'a> {
    fn from(segment: IndentedCodeSegment<'a>) -> Self {
        segment.0
    }
}

impl<'a> From<&IndentedCodeSegment<'a>> for LineSegment<'a> {
    fn from(segment: &IndentedCodeSegment<'a>) -> Self {
        segment.0
    }
}

// Every line needs to start with at least 4 spaces.
static REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^(?:(?:[ ]{4})|(?:\t))(?:\s*\S.*)$").unwrap());

impl<'a> TryFrom<LineSegment<'a>> for IndentedCodeSegment<'a> {
    type Error = LineSegment<'a>;

    fn try_from(segment: LineSegment<'a>) -> Result<Self, Self::Error> {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl<'a> TryFrom<LineSegment<'a>> for IndentedCodeOrBlankLineSegment<'a> {
    type Error = LineSegment<'a>;

    fn try_from(segment: LineSegment<'a>) -> Result<Self, Self::Error> {
        if let Ok(segment) = IndentedCodeSegment::try_from(segment) {
            Ok(IndentedCodeOrBlankLineSegment::IndentedCode(segment))
        } else if let Ok(segment) = BlankLineSegment::try_from(segment) {
            Ok(IndentedCodeOrBlankLineSegment::BlankLine(segment))
        } else {
            Err(segment)
        }
    }
}

impl<'a> From<IndentedCodeOrBlankLineSegment<'a>> for LineSegment<'a> {
    fn from(value: IndentedCodeOrBlankLineSegment<'a>) -> Self {
        match value {
            IndentedCodeOrBlankLineSegment::IndentedCode(segment) => segment.into(),
            IndentedCodeOrBlankLineSegment::BlankLine(segment) => segment.into(),
        }
    }
}

impl<'a> From<&IndentedCodeOrBlankLineSegment<'a>> for LineSegment<'a> {
    fn from(value: &IndentedCodeOrBlankLineSegment<'a>) -> Self {
        (*value).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod indented_code_segment {
        use segment::SegmentStrExt;

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

        failure_case!(should_reject_empty_segment, LineSegment::default());
        failure_case!(should_reject_blank_line, " \n".line());
        failure_case!(
            should_reject_3_whitespaces_indent,
            "   Missing one space\n".line()
        );

        success_case!(
            should_work_with_4_whitespaces_indent,
            "    This is indented code. Finally.\n".line()
        );
        success_case!(
            should_work_with_tab_indent,
            "\tThis is indented code. Finally.\n".line()
        );
        success_case!(
            should_work_with_missing_eol,
            "    This is indented code. Finally.".line()
        );
    }

    // Test that it can accept an indented code or a blank line.
    mod indented_code_or_blank_line_segment {
        use segment::SegmentStrExt;

        use super::*;

        #[test]
        fn should_reject_empty_segment() {
            assert_eq!(
                IndentedCodeOrBlankLineSegment::try_from(LineSegment::default()),
                Err(LineSegment::default())
            )
        }

        #[test]
        fn should_work_with_single_char_blank_line() {
            let segment = " \n".line();
            assert_eq!(
                IndentedCodeOrBlankLineSegment::try_from(segment),
                Ok(IndentedCodeOrBlankLineSegment::BlankLine(
                    segment.try_into().unwrap()
                ))
            )
        }

        #[test]
        fn should_work_with_indented_code() {
            let segment = "    This is indented code.\n".line();
            assert_eq!(
                IndentedCodeOrBlankLineSegment::try_from(segment),
                Ok(IndentedCodeOrBlankLineSegment::IndentedCode(
                    segment.try_into().unwrap()
                ))
            )
        }
    }
}
