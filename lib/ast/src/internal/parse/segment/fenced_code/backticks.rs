use std::sync::LazyLock;

use crate::Segment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackticksFencedCodeOpeningSegment<'a> {
    pub segment: Segment<'a>,
    pub indent: usize,
    // The amount of backticks used, minimally 3.
    pub fence_length: usize,
    // TODO: this could be just a str.
    pub info_string: Option<Segment<'a>>,
}

impl<'a> BackticksFencedCodeOpeningSegment<'a> {
    fn new(
        segment: Segment<'a>,
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

/// Info string cannot contain backtick characters.
static BACKTICK_OPENING_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^([ ]{0,3})(`{3,})\s*([^`]+?)?\s*$").unwrap());

impl<'a> TryFrom<Segment<'a>> for BackticksFencedCodeOpeningSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        let captures = BACKTICK_OPENING_REGEX
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
pub struct BackticksFencedCodeClosingSegment<'a> {
    pub segment: Segment<'a>,
    pub indent: usize,
    pub fence_length: usize,
}

impl<'a> BackticksFencedCodeClosingSegment<'a> {
    fn new(segment: Segment<'a>, indent: usize, fence_length: usize) -> Self {
        Self {
            segment,
            indent,
            fence_length,
        }
    }
}

/// Info string cannot contain backtick characters.
static BACKTICK_CLOSING_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^([ ]{0,3})(`{3,})\s*$").unwrap());

impl<'a> TryFrom<Segment<'a>> for BackticksFencedCodeClosingSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        let captures = BACKTICK_CLOSING_REGEX
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
        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        BackticksFencedCodeOpeningSegment::try_from($segment.clone()),
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
                        BackticksFencedCodeOpeningSegment::try_from($segment.clone()),
                        Ok($expected)
                    );
                }
            };
        }

        failure_case!(should_reject_empy, Segment::default());
        failure_case!(should_reject_blank_line, Segment::first("\n"));
        failure_case!(should_reject_2_backticks, Segment::first("``\n"));
        failure_case!(
            should_reject_backticks_in_info_string,
            Segment::first("```rust`\n")
        );
        failure_case!(
            should_reject_4_whitespace_indent,
            Segment::first("    ```\n")
        );
        failure_case!(should_reject_tab_indent, Segment::first("\t```\n"));

        success_case!(
            should_work_with_3_backticks,
            Segment::first("```\n"),
            BackticksFencedCodeOpeningSegment::new(Segment::first("```\n"), 0, 3, None)
        );
        success_case!(
            should_work_with_3_backticks_and_3_whitespace_ident,
            Segment::first("   ```\n"),
            BackticksFencedCodeOpeningSegment::new(Segment::first("   ```\n"), 3, 3, None)
        );
        success_case!(
            should_work_with_info_string,
            Segment::first("```rust\n"),
            BackticksFencedCodeOpeningSegment::new(
                Segment::first("```rust\n"),
                0,
                3,
                Some(Segment::new(location::Position::new(1, 4, 3), "rust"))
            )
        );
        success_case!(
            should_work_with_padded_info_string,
            Segment::first("```   rust is kind of fucking cool   \n"),
            BackticksFencedCodeOpeningSegment::new(
                Segment::first("```   rust is kind of fucking cool   \n"),
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
        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        BackticksFencedCodeClosingSegment::try_from($segment.clone()),
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
                        BackticksFencedCodeClosingSegment::try_from($segment.clone()),
                        Ok($expected)
                    );
                }
            };
        }

        failure_case!(should_reject_empy, Segment::default());
        failure_case!(should_reject_blank_line, Segment::first("\n"));
        failure_case!(should_reject_2_backticks, Segment::first("``\n"));
        failure_case!(should_reject_info_string, Segment::first("```rust\n"));
        failure_case!(
            should_reject_4_whitespace_indent,
            Segment::first("    ```\n")
        );
        failure_case!(should_reject_tab_indent, Segment::first("\t```\n"));

        success_case!(
            should_work_with_3_backticks,
            Segment::first("```\n"),
            BackticksFencedCodeClosingSegment::new(Segment::first("```\n"), 0, 3)
        );
        success_case!(
            should_work_with_4_backticks,
            Segment::first("````\n"),
            BackticksFencedCodeClosingSegment::new(Segment::first("````\n"), 0, 4)
        );
        success_case!(
            should_work_with_trailing_whitespaces,
            Segment::first("```   \t\n"),
            BackticksFencedCodeClosingSegment::new(Segment::first("```   \t\n"), 0, 3)
        );
        success_case!(
            should_work_with_3_whitespaces_indent,
            Segment::first("   ```\n"),
            BackticksFencedCodeClosingSegment::new(Segment::first("   ```\n"), 3, 3)
        );
    }
}
