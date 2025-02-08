use std::sync::LazyLock;

use segment::{LineSegment, SegmentLike};

/// Represents a blank line segment.
///
/// A blank line contains at least one whitespace character, and only whitespace characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlankLineSegment<'a>(pub LineSegment<'a>);

impl<'a> BlankLineSegment<'a> {
    fn new(segment: LineSegment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> From<BlankLineSegment<'a>> for LineSegment<'a> {
    fn from(blank_line: BlankLineSegment<'a>) -> Self {
        blank_line.0
    }
}

// Just a blank line y'all. Still requires at least one whitespace character.
static REGEX: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"^\s+$").unwrap());

impl<'a> TryFrom<LineSegment<'a>> for BlankLineSegment<'a> {
    type Error = LineSegment<'a>;

    fn try_from(segment: LineSegment<'a>) -> Result<Self, Self::Error> {
        if REGEX.is_match(segment.text()) {
            Ok(Self::new(segment))
        } else {
            Err(segment)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from {
        use segment::SegmentStrExt;

        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(BlankLineSegment::try_from($segment.clone()), Err($segment))
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        BlankLineSegment::try_from($segment),
                        Ok(BlankLineSegment::new($segment))
                    )
                }
            };
        }

        failure_case!(should_reject_empty, "".line());
        failure_case!(should_reject_line_with_a_char, "    a     \n".line());

        success_case!(should_work_with_one_whitespace, " ".line());
        success_case!(should_work_with_a_single_newline, "\n".line());
        success_case!(should_work_with_a_single_tab, "\t".line());
        success_case!(should_work_with_any_whitespace, "\t\r\n".line());
    }
}
