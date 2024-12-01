use std::sync::LazyLock;

use crate::{
    internal::parse::try_extract::{Extraction, TryExtract},
    Segment,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LooseLinkDestinationSegment<'a> {
    pub segment: Segment<'a>,
}

impl<'a> LooseLinkDestinationSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }

    fn parentheseses_balance(segment: Segment<'a>) -> bool {
        // Ignore escaped parentheseses.
        let sanitized = segment.text().replace(r"\(", "").replace(r"\)", "");
        // Ensure the count of opening and closing parentheseses is equal.
        sanitized.chars().filter(|&c| c == '(').count()
            == sanitized.chars().filter(|&c| c == ')').count()
    }

    pub fn is_opening_char(character: char) -> bool {
        // The segment cannot start with the '<' character.s
        Self::is_continuation_char(character) && character != '<'
    }

    pub fn is_continuation_char(character: char) -> bool {
        // ASCII control characters and spaces are not allowed
        character != ' ' && !character.is_ascii_control()
    }
}

// The loose variation does not include ASCII control characters,
// spaces and does not start with the '<' character.
static INLINE_LOOSE_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[^< \x00-\x1F\x7F][^ \x00-\x1F\x7F]*").unwrap());
static LOOSE_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_LOOSE_REGEX.as_str()).as_str()).unwrap()
});

impl<'a> TryFrom<Segment<'a>> for LooseLinkDestinationSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if LOOSE_REGEX.is_match(&segment.text())
            && LooseLinkDestinationSegment::parentheseses_balance(segment)
        {
            Ok(Self::new(segment))
        } else {
            Err(segment)
        }
    }
}

impl<'a> TryExtract<Segment<'a>> for LooseLinkDestinationSegment<'a> {
    type Remaining = Segment<'a>;
    type Error = Segment<'a>;

    fn try_extract(segment: Segment<'a>) -> Result<Extraction<Self, Segment<'a>>, Self::Error> {
        let mut char_indices = segment.text().char_indices();
        let Some(first_char) = char_indices.next() else {
            return Err(segment);
        };

        if !LooseLinkDestinationSegment::is_opening_char(first_char.1) {
            return Err(segment);
        }

        while let Some(char_index) = char_indices.next() {
            if !LooseLinkDestinationSegment::is_continuation_char(char_index.1) {
                let (maybe_link_destination, remaining) = segment.split_at(char_index.0);
                let Ok(loose_destination_segment) =
                    LooseLinkDestinationSegment::try_from(maybe_link_destination)
                else {
                    return Err(segment);
                };

                return Ok(Extraction::new(loose_destination_segment, remaining));
            }
        }

        let Ok(loose_link_destination) = LooseLinkDestinationSegment::try_from(segment) else {
            return Err(segment);
        };
        Ok(Extraction::new(
            loose_link_destination,
            Segment::empty_at(segment.end()),
        ))
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
                    assert_eq!(
                        LooseLinkDestinationSegment::try_from($segment.clone()),
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
                        LooseLinkDestinationSegment::try_from($segment.clone()),
                        Ok(LooseLinkDestinationSegment::new($segment))
                    );
                }
            };
        }

        failure_case!(should_reject_space, Segment::first(" "));
        failure_case!(should_reject_leading_whitespace, Segment::first(" a"));
        failure_case!(should_reject_trailing_whitespace, Segment::first("a "));
        failure_case!(
            should_reject_missing_closing_parenthesis,
            Segment::first("(")
        );
        failure_case!(
            should_reject_missing_opening_parenthesis,
            Segment::first(")")
        );
        failure_case!(
            should_reject_ascii_control_character,
            Segment::first("\x00")
        );

        success_case!(should_work_with_character, Segment::first("a"));
        success_case!(should_work_with_several_characters, Segment::first("abc"));
        success_case!(should_work_with_slash, Segment::first("/"));
        success_case!(
            should_work_with_relative_path,
            Segment::first("./relative/path.sftu")
        );
        success_case!(
            should_work_with_fragment_identifier,
            Segment::first("#fragment")
        );
        success_case!(
            should_work_with_full_uri,
            Segment::first("https://example.com?query=value#head-wallet")
        );
        success_case!(
            should_work_with_escaped_opening_parenthesis,
            Segment::first(r"\(")
        );
        success_case!(
            should_work_with_escaped_closing_parenthesis,
            Segment::first(r"\)")
        );
        success_case!(
            should_work_with_balanced_parentheses,
            Segment::first("(foo(and(bar)))")
        );
    }

    mod try_extract {
        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        LooseLinkDestinationSegment::try_extract($segment.clone()),
                        Err($segment)
                    );
                }
            };
        }

        macro_rules! success_case {
            (
                $test:ident,
                $segment:expr,
                $expected_link_destination:expr,
                $expected_remaining:expr
            ) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        LooseLinkDestinationSegment::try_extract($segment.clone()),
                        Ok(Extraction::new(
                            $expected_link_destination,
                            $expected_remaining
                        ))
                    );
                }
            };
        }

        failure_case!(should_fail_for_empty_segment, Segment::default());
        failure_case!(should_fail_for_newline, Segment::first("\n"));

        success_case!(
            should_work_with_valid_link_destination_and_no_remaining,
            Segment::first("hello/toto"),
            LooseLinkDestinationSegment::try_from(Segment::first("hello/toto")).unwrap(),
            Segment::empty_at(location::Position::new(1, 11, 10))
        );
        success_case!(
            should_work_with_valid_link_destination_and_remaining_text,
            Segment::first("hello/toto 'this is a link title'"),
            LooseLinkDestinationSegment::try_from(Segment::first("hello/toto")).unwrap(),
            Segment::new(
                location::Position::new(1, 11, 10),
                " 'this is a link title'"
            )
        );
    }
}
