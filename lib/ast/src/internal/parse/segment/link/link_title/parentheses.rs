use std::sync::LazyLock;

use crate::{internal::parse::segment::BlankLineSegment, Segment};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesesLinkTitleOpeningSegment<'a> {
    pub segment: Segment<'a>,
}

impl<'a> ParenthesesLinkTitleOpeningSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }

    pub fn is_closing(&self) -> bool {
        self.segment.text().ends_with(')')
    }
}

static INLINE_OPENING_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"\((?:(?:\\.)|(?:[^\\()]))*(?:\)|\n)").unwrap());
static OPENING_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_OPENING_REGEX.as_str()).as_str()).unwrap()
});

impl<'a> TryFrom<Segment<'a>> for ParenthesesLinkTitleOpeningSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if OPENING_REGEX.is_match(&segment.text()) {
            Ok(Self::new(segment))
        } else {
            Err(segment)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesesLinkTitleContinuationSegment<'a> {
    pub segment: Segment<'a>,
}

impl<'a> ParenthesesLinkTitleContinuationSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }

    pub fn is_closing(&self) -> bool {
        self.segment.text().ends_with(')')
    }
}

// TODO: new line or parenthesis at the end, or die.
static INLINE_CONTINUATION_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?:(?:\\.)|(?:[^\\()]))*(?:\)|\n)").unwrap());
static CONTINUATION_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_CONTINUATION_REGEX.as_str()).as_str()).unwrap()
});

impl<'a> TryFrom<Segment<'a>> for ParenthesesLinkTitleContinuationSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if CONTINUATION_REGEX.is_match(&segment.text())
            && BlankLineSegment::try_from(segment.clone()).is_err()
        {
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
                        ParenthesesLinkTitleOpeningSegment::try_from($segment.clone()),
                        Err($segment)
                    );
                }
            };
        }

        failure_case!(should_reject_empty, Segment::default());
        failure_case!(should_reject_single_newline, Segment::first("\n"));
        failure_case!(should_reject_blank_line, Segment::first(" \t\n"));
        failure_case!(should_reject_leading_whitespace, Segment::first(" ()"));
        failure_case!(should_reject_trailing_whitespace, Segment::first("() "));
        failure_case!(should_reject_opening_without_newline, Segment::first("("));

        // The absence of the closing parenthesis signifies that the segment is not closed.
        // In that case, a new line character should end the segment.
        mod not_closed {
            use super::*;

            macro_rules! success_case {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        let result =
                            ParenthesesLinkTitleOpeningSegment::try_from($segment.clone()).unwrap();
                        assert_eq!(result.segment, $segment);
                        assert_eq!(result.is_closing(), false);
                    }
                };
            }

            failure_case!(should_reject_missing_newline, Segment::first("("));
            success_case!(should_work_with_single_opening_quote, Segment::first("(\n"));
            success_case!(should_work_with_some_text, Segment::first("(Hello,\n"));
            success_case!(
                should_work_with_escaped_parentheses,
                Segment::first("(Hello, \\(Bro\\)\n")
            );
            success_case!(
                should_work_with_any_escape,
                Segment::first("(Hello, \\;World!\n")
            );
        }

        // The presence of the ending quote is what makes them closed.
        mod closed {
            use super::*;

            macro_rules! success_case {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        let result =
                            ParenthesesLinkTitleOpeningSegment::try_from($segment.clone()).unwrap();
                        assert_eq!(result.segment, $segment);
                        assert_eq!(result.is_closing(), true);
                    }
                };
            }

            success_case!(should_work_without_content, Segment::first("()"));
            success_case!(should_work_with_some_text, Segment::first("(Hello)"));
            success_case!(
                should_work_with_escaped_parentheses,
                Segment::first("(Hello, \\(Bro\\))")
            );
            success_case!(
                should_work_with_any_escape,
                Segment::first("(Hello, \\;World!)")
            );
        }
    }

    mod continuation {
        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        ParenthesesLinkTitleContinuationSegment::try_from($segment.clone()),
                        Err($segment)
                    );
                }
            };
        }

        // Either it ends with a newline or a closing parenthesis.
        // Blank lines are always rejected as continuation segments.
        failure_case!(should_reject_empty, Segment::default());
        failure_case!(should_reject_single_newline, Segment::first("\n"));
        failure_case!(should_reject_blank_line, Segment::first(" \t\n"));
        failure_case!(should_reject_2_closing_parenthesis, Segment::first("))"));
        failure_case!(should_reject_trailing_whitespace, Segment::first(") "));

        mod not_closed {
            use super::*;

            macro_rules! success_case {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        let result =
                            ParenthesesLinkTitleContinuationSegment::try_from($segment.clone())
                                .unwrap();
                        assert_eq!(result.segment, $segment);
                        assert_eq!(result.is_closing(), false);
                    }
                };
            }

            success_case!(should_work_with_a_single_character, Segment::first("a\n"));
            success_case!(
                should_work_with_leading_whitespace,
                Segment::first(" \ta\n")
            );
            success_case!(should_work_with_trailing_whitespace, Segment::first("a \n"));
            success_case!(should_work_with_single_quotes, Segment::first("a'\n"));
            success_case!(should_work_with_double_quotes, Segment::first("a\"b\n"));
            success_case!(
                should_work_with_escaped_parenthesis,
                Segment::first("a\\)b\n")
            );
        }

        mod closed {
            use super::*;

            macro_rules! success_case {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        let result =
                            ParenthesesLinkTitleContinuationSegment::try_from($segment.clone())
                                .unwrap();
                        assert_eq!(result.segment, $segment);
                        assert_eq!(result.is_closing(), true);
                    }
                };
            }

            success_case!(should_work_with_a_single_character, Segment::first("a)"));
            success_case!(should_work_with_leading_whitespace, Segment::first(" a)"));
            success_case!(should_work_with_single_quote, Segment::first("a')"));
            success_case!(should_work_with_double_quotes, Segment::first("a\")"));
            success_case!(
                should_work_with_escaped_parenthesis,
                Segment::first("a\\)b)")
            );
        }
    }
}
