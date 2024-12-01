use std::sync::LazyLock;

use crate::Segment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetextHeadingEqualsUnderlineSegment<'a> {
    pub segment: Segment<'a>,
}

impl<'a> SetextHeadingEqualsUnderlineSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }

    pub fn level(&self) -> u8 {
        1
    }
}

static REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[ ]{0,3}=+\s*?\n$").unwrap());

impl<'a> TryFrom<Segment<'a>> for SetextHeadingEqualsUnderlineSegment<'a> {
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

    macro_rules! failure_case {
        ($test:ident, $segment:expr) => {
            #[test]
            fn $test() {
                assert_eq!(
                    SetextHeadingEqualsUnderlineSegment::try_from($segment.clone()),
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
                    SetextHeadingEqualsUnderlineSegment::try_from($segment.clone()),
                    Ok(SetextHeadingEqualsUnderlineSegment::new($segment))
                );
            }
        };
    }

    failure_case!(should_fail_with_empty, Segment::default());
    failure_case!(should_fail_with_blank_line, Segment::first("\n"));
    failure_case!(should_fail_with_4_idents, Segment::first("    =\n"));
    failure_case!(should_fail_for_missing_eol, Segment::first("="));
    failure_case!(should_reject_trailing_characters, Segment::first("=a\n"));
    failure_case!(should_fail_for_hyphens, Segment::first("-\n"));

    success_case!(should_work_with_single_equal, Segment::first("=\n"));
    success_case!(should_work_with_3_equals, Segment::first("===\n"));
    success_case!(should_work_with_3_idents, Segment::first("   =\n"));
    success_case!(
        should_work_with_trailing_whitespace,
        Segment::first("=  \n")
    );
}
