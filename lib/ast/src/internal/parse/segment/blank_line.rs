use std::sync::LazyLock;

use crate::Segment;

/// Represents a blank line segment.
///
/// A blank line contains at least one whitespace character, and only whitespace characters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlankLineSegment<'a>(pub Segment<'a>);

impl<'a> BlankLineSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> From<BlankLineSegment<'a>> for Segment<'a> {
    fn from(blank_line: BlankLineSegment<'a>) -> Segment<'a> {
        blank_line.0
    }
}

// Just a blank line y'all. Still requires at least one whitespace character.
static REGEX: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"^\s+$").unwrap());

impl<'a> TryFrom<Segment<'a>> for BlankLineSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
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

        failure_case!(should_reject_empty, Segment::first(""));
        failure_case!(
            should_reject_line_with_a_char,
            Segment::first("    a     \n")
        );

        success_case!(should_work_with_one_whitespace, Segment::first(" "));
        success_case!(should_work_with_a_single_newline, Segment::first("\n"));
        success_case!(should_work_with_a_single_tab, Segment::first("\t"));
        success_case!(should_work_with_any_whitespace, Segment::first("\t\r\n"));
    }
}
