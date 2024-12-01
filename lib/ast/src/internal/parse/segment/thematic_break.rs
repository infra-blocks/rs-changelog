use std::sync::LazyLock;

use crate::Segment;

/// A thematic break segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreakSegment<'a>(pub Segment<'a>);

impl<'a> ThematicBreakSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> From<ThematicBreakSegment<'a>> for Segment<'a> {
    fn from(thematic_break: ThematicBreakSegment<'a>) -> Segment<'a> {
        thematic_break.0
    }
}

// Thematic breaks are three or more matching -, _, or * characters. They can be preceded by up to 3 spaces
// and followed by any amount. They can also be interspersed with spaces. No other characters are allowed on the same line.
static REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^[ ]{0,3}(?:(?:_[ \t]*){3,}|(?:-[ \t]*){3,}|(?:\*[ \t]*){3,})\n?$").unwrap()
});

impl<'a> TryFrom<Segment<'a>> for ThematicBreakSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if REGEX.is_match(&segment.text()) {
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

        failure_case!(should_reject_empty, Segment::first(""));
        failure_case!(should_reject_blank_line, Segment::first("  \n"));
        failure_case!(should_reject_tab_indent, Segment::first("\t---\n"));
        failure_case!(
            should_reject_four_spaces_indent,
            Segment::first("    ---\n")
        );
        failure_case!(
            should_reject_non_consecutive_tokens,
            Segment::first(" -_*\n")
        );
        failure_case!(
            should_reject_with_presence_of_other_characters,
            Segment::first("---a\n")
        );

        success_case!(should_work_with_three_underscores, Segment::first("___\n"));
        success_case!(should_work_with_four_underscores, Segment::first("____\n"));
        success_case!(should_work_with_three_hyphens, Segment::first("---\n"));
        success_case!(should_work_with_four_hyphens, Segment::first("----\n"));
        success_case!(should_work_with_three_asterisks, Segment::first("***\n"));
        success_case!(should_work_with_four_asterisks, Segment::first("****\n"));
        success_case!(
            should_work_with_three_spaces_indent,
            Segment::first("   ---\n")
        );
        success_case!(
            should_work_with_trailing_whitespace,
            Segment::first("--- \n")
        );
        success_case!(
            should_work_with_spaces_interspersed,
            Segment::first(" - - -\n")
        );
        success_case!(should_work_without_eol, Segment::first("---"));
    }
}
