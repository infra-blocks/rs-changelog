use std::sync::LazyLock;

use segment::{LineSegment, SegmentLike};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetextHeadingHyphensUnderlineSegment<'a>(pub LineSegment<'a>);

impl<'a> SetextHeadingHyphensUnderlineSegment<'a> {
    fn new(segment: LineSegment<'a>) -> Self {
        Self(segment)
    }

    pub fn level(&self) -> u8 {
        2
    }
}

static REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[ ]{0,3}-+\s*?\n?$").unwrap());

impl<'a> TryFrom<LineSegment<'a>> for SetextHeadingHyphensUnderlineSegment<'a> {
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
    use segment::SegmentStrExt;

    use super::*;

    macro_rules! failure_case {
        ($test:ident, $segment:expr) => {
            #[test]
            fn $test() {
                assert_eq!(
                    SetextHeadingHyphensUnderlineSegment::try_from($segment.clone()),
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
                    SetextHeadingHyphensUnderlineSegment::try_from($segment.clone()),
                    Ok(SetextHeadingHyphensUnderlineSegment::new($segment))
                );
            }
        };
    }

    failure_case!(should_fail_with_empty, LineSegment::default());
    failure_case!(should_fail_with_blank_line, "\n".line());
    failure_case!(should_fail_with_4_idents, "    -\n".line());
    failure_case!(should_fail_for_missing_eol, "-".line());
    failure_case!(should_reject_trailing_characters, "-a\n".line());
    failure_case!(should_fail_for_equals, "=\n".line());

    success_case!(should_work_with_single_hyphen, "-\n".line());
    success_case!(should_work_with_3_hyphens, "---\n".line());
    success_case!(should_work_with_3_idents, "   -\n".line());
    success_case!(should_work_with_trailing_whitespace, "-  \n".line());
}
