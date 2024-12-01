use std::sync::LazyLock;

use crate::{internal::parse::segment::BlankLineSegment, Segment};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleQuotesLinkTitleOpeningSegment<'a> {
    pub segment: Segment<'a>,
}

impl<'a> DoubleQuotesLinkTitleOpeningSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }

    pub fn is_closing(&self) -> bool {
        self.segment.text().ends_with('"')
    }
}

static INLINE_DOUBLE_QUOTE_ENTRY_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r#""(?:(?:\\.)|(?:[^\\"]))*(?:"|\n)"#).unwrap());
static DOUBLE_QUOTE_OPENING_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_DOUBLE_QUOTE_ENTRY_REGEX.as_str()).as_str()).unwrap()
});

impl<'a> TryFrom<Segment<'a>> for DoubleQuotesLinkTitleOpeningSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if DOUBLE_QUOTE_OPENING_REGEX.is_match(&segment.text()) {
            Ok(Self::new(segment))
        } else {
            Err(segment)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleQuotesLinkTitleContinuationSegment<'a> {
    pub segment: Segment<'a>,
}

impl<'a> DoubleQuotesLinkTitleContinuationSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }

    pub fn is_closing(&self) -> bool {
        self.segment.text().ends_with('"')
    }
}

static INLINE_CLOSING_DOUBLE_QUOTE_CONTINUATION_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r#"(?:(?:\\.)|(?:[^\\"]))*""#).unwrap());
static INLINE_CONTINUING_DOUBLE_QUOTE_CONTINUATION_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r#"(?:(?:\\.)|(?:[^\\"]))+\n"#).unwrap());
// TODO: we don't need that extra step if we don't need to export the "inline" stuff
static INLINE_DOUBLE_QUOTE_CONTINUATION_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        format!(
            "(?:(?:{})|(?:{}))",
            INLINE_CLOSING_DOUBLE_QUOTE_CONTINUATION_REGEX.as_str(),
            INLINE_CONTINUING_DOUBLE_QUOTE_CONTINUATION_REGEX.as_str()
        )
        .as_str(),
    )
    .unwrap()
});
static DOUBLE_QUOTE_CONTINUATION_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_DOUBLE_QUOTE_CONTINUATION_REGEX.as_str()).as_str())
        .unwrap()
});

impl<'a> TryFrom<Segment<'a>> for DoubleQuotesLinkTitleContinuationSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if DOUBLE_QUOTE_CONTINUATION_REGEX.is_match(&segment.text())
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
                        DoubleQuotesLinkTitleOpeningSegment::try_from($segment.clone()),
                        Err($segment)
                    );
                }
            };
        }

        failure_case!(should_reject_empty, Segment::default());
        failure_case!(should_reject_single_newline, Segment::first("\n"));
        failure_case!(should_reject_blank_line, Segment::first(" \t\n"));
        failure_case!(should_reject_leading_whitespace, Segment::first(r#" """#));

        failure_case!(should_reject_trailing_whitespace, Segment::first(r#""" "#));
        failure_case!(
            should_reject_opening_quote_without_newline,
            Segment::first(r#"""#)
        );

        // The absence of the closing quote signifies that the segment is not closed.
        // In that case, a new line character should end the segment.
        mod not_closed {
            use super::*;

            macro_rules! success_case {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        let result =
                            DoubleQuotesLinkTitleOpeningSegment::try_from($segment.clone())
                                .unwrap();
                        assert_eq!(result.segment, $segment);
                        assert_eq!(result.is_closing(), false);
                    }
                };
            }

            failure_case!(should_reject_missing_newline, Segment::first("\""));
            success_case!(
                should_work_with_single_opening_quote,
                Segment::first("\"\n")
            );
            success_case!(should_work_with_some_text, Segment::first("\"Hello,\n"));
            success_case!(
                should_work_with_escaped_quotes,
                Segment::first("\"Hello, \\\"Bro\\\"\n")
            );
            success_case!(
                should_work_with_any_escape,
                Segment::first("\"Hello, \\;World!\n")
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
                            DoubleQuotesLinkTitleOpeningSegment::try_from($segment.clone())
                                .unwrap();
                        assert_eq!(result.segment, $segment);
                        assert_eq!(result.is_closing(), true);
                    }
                };
            }

            success_case!(should_work_without_content, Segment::first("\"\""));
            success_case!(should_work_with_some_text, Segment::first("\"Hello\""));
            success_case!(
                should_work_with_escaped_quotes,
                Segment::first("\"Hello, \\\"Bro\\\"\"")
            );
            success_case!(
                should_work_with_any_escape,
                Segment::first("\"Hello, \\;World!\"")
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
                        DoubleQuotesLinkTitleContinuationSegment::try_from($segment.clone()),
                        Err($segment)
                    );
                }
            };
        }

        // Either it ends with a newline or a double quote.
        // Blank lines are always rejected as continuation segments.
        failure_case!(should_reject_empty, Segment::default());
        failure_case!(should_reject_single_newline, Segment::first("\n"));
        failure_case!(should_reject_blank_line, Segment::first(" \t\n"));
        failure_case!(should_reject_two_double_quotes, Segment::first("\"\""));
        failure_case!(should_reject_trailing_whitespace, Segment::first("\" "));

        mod not_closed {
            use super::*;

            macro_rules! success_case {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        let result =
                            DoubleQuotesLinkTitleContinuationSegment::try_from($segment.clone())
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
            success_case!(should_work_with_escaped_quotes, Segment::first("a\\\"b\n"));
        }

        mod closed {
            use super::*;

            macro_rules! success_case {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        let result =
                            DoubleQuotesLinkTitleContinuationSegment::try_from($segment.clone())
                                .unwrap();
                        assert_eq!(result.segment, $segment);
                        assert_eq!(result.is_closing(), true);
                    }
                };
            }

            success_case!(should_work_with_a_single_character, Segment::first("a\""));
            success_case!(should_work_with_leading_whitespace, Segment::first(" a\""));
            success_case!(should_work_with_double_quotes, Segment::first("a'\""));
            success_case!(should_work_with_escaped_quotes, Segment::first("a\\\"b\""));
        }
    }
}
