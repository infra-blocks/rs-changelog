use std::sync::LazyLock;

use segment::{LineSegment, Segment, SegmentLike};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TildesFencedCodeOpeningSegment<'a> {
    pub segment: LineSegment<'a>,
    pub indent: usize,
    // The amount of tildes used, minimally 3.
    pub fence_length: usize,
    pub info_string: Option<Segment<'a>>,
}

impl<'a> TildesFencedCodeOpeningSegment<'a> {
    fn new(
        segment: LineSegment<'a>,
        indent: usize,
        fence_length: usize,
        info_string: Option<Segment<'a>>,
    ) -> Self {
        Self {
            segment,
            indent,
            fence_length,
            info_string,
        }
    }
}

// In the tilde case, the info string can contain tildes, and tildes.
static TILDE_OPENING_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^([ ]{0,3})(~{3,})\s*(.+?)?\s*$").unwrap());

impl<'a> TryFrom<LineSegment<'a>> for TildesFencedCodeOpeningSegment<'a> {
    type Error = LineSegment<'a>;

    fn try_from(segment: LineSegment<'a>) -> Result<Self, Self::Error> {
        let captures = TILDE_OPENING_REGEX
            .captures(segment.text())
            .ok_or(segment)?;

        // Indent is first group.
        let indent = captures.get(1).unwrap().as_str().len();
        // The fence is the second group.
        let fence_length = captures.get(2).unwrap().as_str().len();
        // The info string is the optional third group.
        let info_string = captures.get(3).map(|info_string_match| {
            Segment::new(
                segment
                    .start()
                    .walk(&segment.text()[..info_string_match.start()]),
                info_string_match.as_str(),
            )
        });
        Ok(Self::new(segment, indent, fence_length, info_string))
    }
}

// Closing segments don't have info strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TildesFencedCodeClosingSegment<'a> {
    pub segment: LineSegment<'a>,
    pub indent: usize,
    pub fence_length: usize,
}

impl<'a> TildesFencedCodeClosingSegment<'a> {
    fn new(segment: LineSegment<'a>, indent: usize, fence_length: usize) -> Self {
        Self {
            segment,
            indent,
            fence_length,
        }
    }
}

// No info strings on closing segments.
static TILDE_CLOSING_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^([ ]{0,3})(~{3,})\s*$").unwrap());

impl<'a> TryFrom<LineSegment<'a>> for TildesFencedCodeClosingSegment<'a> {
    type Error = LineSegment<'a>;

    fn try_from(segment: LineSegment<'a>) -> Result<Self, Self::Error> {
        let captures = TILDE_CLOSING_REGEX
            .captures(segment.text())
            .ok_or(segment)?;

        // Indent is first group.
        let indent = captures.get(1).unwrap().as_str().len();
        // The fence is the second group.
        let fence_length = captures.get(2).unwrap().as_str().len();
        Ok(Self::new(segment, indent, fence_length))
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
                        TildesFencedCodeOpeningSegment::try_from($segment.clone()),
                        Err($segment)
                    );
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        TildesFencedCodeOpeningSegment::try_from($segment.clone()),
                        Ok($expected)
                    );
                }
            };
        }

        failure_case!(should_reject_empy, LineSegment::default());
        failure_case!(should_reject_blank_line, "\n".line());
        failure_case!(should_reject_2_tildes, "~~\n".line());
        failure_case!(
            should_reject_4_whitespace_indent,
            "    ~~~\n".line()
        );
        failure_case!(should_reject_tab_indent, "\t~~~\n".line());

        success_case!(
            should_work_with_3_tildes,
            "~~~\n".line(),
            TildesFencedCodeOpeningSegment::new("~~~\n".line(), 0, 3, None)
        );
        success_case!(
            should_work_with_3_tildes_and_3_whitespace_ident,
            "   ~~~\n".line(),
            TildesFencedCodeOpeningSegment::new("   ~~~\n".line(), 3, 3, None)
        );
        success_case!(
            should_work_with_info_string,
            "~~~rust\n".line(),
            TildesFencedCodeOpeningSegment::new(
                "~~~rust\n".line(),
                0,
                3,
                Some(Segment::new(location::Position::new(1, 4, 3), "rust"))
            )
        );
        success_case!(
            should_work_tildes_in_info_string,
            "~~~rust~\n".line(),
            TildesFencedCodeOpeningSegment::new(
                "~~~rust~\n".line(),
                0,
                3,
                Some(Segment::new(location::Position::new(1, 4, 3), "rust~"))
            )
        );
        success_case!(
            should_work_backticks_in_info_string,
            "~~~rust`\n".line(),
            TildesFencedCodeOpeningSegment::new(
                "~~~rust`\n".line(),
                0,
                3,
                Some(Segment::new(location::Position::new(1, 4, 3), "rust`"))
            )
        );
        success_case!(
            should_work_with_padded_info_string,
            "~~~   rust is kind of fucking cool   \n".line(),
            TildesFencedCodeOpeningSegment::new(
                "~~~   rust is kind of fucking cool   \n".line(),
                0,
                3,
                Some(Segment::new(
                    location::Position::new(1, 7, 6),
                    "rust is kind of fucking cool"
                ))
            )
        );
    }

    mod closing {
        use segment::SegmentStrExt;

        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        TildesFencedCodeClosingSegment::try_from($segment.clone()),
                        Err($segment)
                    );
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        TildesFencedCodeClosingSegment::try_from($segment.clone()),
                        Ok($expected)
                    );
                }
            };
        }

        failure_case!(should_reject_empy, LineSegment::default());
        failure_case!(should_reject_blank_line, "\n".line());
        failure_case!(should_reject_2_tildes, "~~\n".line());
        failure_case!(should_reject_info_string, "~~~rust\n".line());
        failure_case!(
            should_reject_4_whitespace_indent,
            "    ~~~\n".line()
        );
        failure_case!(should_reject_tab_indent, "\t~~~\n".line());

        success_case!(
            should_work_with_3_tildes,
            "~~~\n".line(),
            TildesFencedCodeClosingSegment::new("~~~\n".line(), 0, 3)
        );
        success_case!(
            should_work_with_4_tildes,
            "~~~~\n".line(),
            TildesFencedCodeClosingSegment::new("~~~~\n".line(), 0, 4)
        );
        success_case!(
            should_work_with_trailing_whitespaces,
            "~~~   \t\n".line(),
            TildesFencedCodeClosingSegment::new("~~~   \t\n".line(), 0, 3)
        );
        success_case!(
            should_work_with_3_whitespaces_indent,
            "   ~~~\n".line(),
            TildesFencedCodeClosingSegment::new("   ~~~\n".line(), 3, 3)
        );
    }
}
