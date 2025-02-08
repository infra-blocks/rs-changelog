use std::sync::LazyLock;

use segment::{LineSegment, SegmentLike};

/// A thematic break segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThematicBreakSegment<'a>(pub LineSegment<'a>);

impl<'a> ThematicBreakSegment<'a> {
    fn new(segment: LineSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> From<ThematicBreakSegment<'a>> for LineSegment<'a> {
    fn from(thematic_break: ThematicBreakSegment<'a>) -> LineSegment<'a> {
        thematic_break.0
    }
}

// Thematic breaks are three or more matching -, _, or * characters. They can be preceded by up to 3 spaces
// and followed by any amount. They can also be interspersed with spaces. No other characters are allowed on the same line.
static REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^[ ]{0,3}(?:(?:_[ \t]*){3,}|(?:-[ \t]*){3,}|(?:\*[ \t]*){3,})\n?$").unwrap()
});

impl<'a> TryFrom<LineSegment<'a>> for ThematicBreakSegment<'a> {
    type Error = LineSegment<'a>;

    fn try_from(segment: LineSegment<'a>) -> Result<Self, Self::Error> {
        if REGEX.is_match(segment.text()) {
            Ok(ThematicBreakSegment::new(segment))
        } else {
            Err(segment)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Test segment validity.
    mod try_from {
        use segment::SegmentStrExt;

        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        ThematicBreakSegment::try_from($segment.clone()),
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
                        ThematicBreakSegment::try_from($segment.clone()),
                        Ok(ThematicBreakSegment::new($segment))
                    )
                }
            };
        }

        failure_case!(should_reject_empty, LineSegment::default());
        failure_case!(should_reject_blank_line, "  \n".line());
        failure_case!(should_reject_tab_indent, "\t---\n".line());
        failure_case!(should_reject_four_spaces_indent, "    ---\n".line());
        failure_case!(
            should_reject_non_consecutive_tokens,
            " -_*\n".line()
        );
        failure_case!(
            should_reject_with_presence_of_other_characters,
            "---a\n".line()
        );

        success_case!(should_work_with_three_underscores, "___\n".line());
        success_case!(should_work_with_four_underscores, "____\n".line());
        success_case!(should_work_with_three_hyphens, "---\n".line());
        success_case!(should_work_with_four_hyphens, "----\n".line());
        success_case!(should_work_with_three_asterisks, "***\n".line());
        success_case!(should_work_with_four_asterisks, "****\n".line());
        success_case!(
            should_work_with_three_spaces_indent,
            "   ---\n".line()
        );
        success_case!(
            should_work_with_trailing_whitespace,
            "--- \n".line()
        );
        success_case!(
            should_work_with_spaces_interspersed,
            " - - -\n".line()
        );
        success_case!(should_work_without_eol, "---".line());
    }
}
