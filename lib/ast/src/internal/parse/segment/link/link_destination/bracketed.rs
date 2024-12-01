use std::sync::LazyLock;

use crate::{
    internal::parse::try_extract::{Extraction, TryExtract},
    Segment,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BracketedLinkDestinationSegment<'a> {
    pub segment: Segment<'a>,
}

// The bracketed variation is encased in angle brackets and can contain any
// character except for unescaped angle brackets and new lines. New lines shouldn't
// occur in the normal flow of the program but still, let's crack down on them.
static INLINE_BRACKETED_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"<(?:(?:\\<)|(?:\\>)|[^\n\\<>])*>").unwrap());
static BRACKETED_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_BRACKETED_REGEX.as_str()).as_str()).unwrap()
});

impl<'a> BracketedLinkDestinationSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }

    pub fn is_opening_char(character: char) -> bool {
        character == '<'
    }

    pub fn is_closing_sequence(previous: char, current: char) -> bool {
        previous != '\\' && current == '>'
    }
}

impl<'a> TryFrom<Segment<'a>> for BracketedLinkDestinationSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if BRACKETED_REGEX.is_match(&segment.text()) {
            Ok(Self::new(segment))
        } else {
            Err(segment)
        }
    }
}

impl<'a> TryExtract<Segment<'a>> for BracketedLinkDestinationSegment<'a> {
    type Remaining = Segment<'a>;
    type Error = Segment<'a>;

    fn try_extract(segment: Segment<'a>) -> Result<Extraction<Self, Segment<'a>>, Segment<'a>> {
        let mut char_indices = segment.text().char_indices();
        let Some(first_char) = char_indices.next() else {
            return Err(segment);
        };

        if !BracketedLinkDestinationSegment::is_opening_char(first_char.1) {
            return Err(segment);
        }

        let mut previous_char = first_char;
        while let Some(char_index) = char_indices.next() {
            if BracketedLinkDestinationSegment::is_closing_sequence(previous_char.1, char_index.1) {
                let next_char_index = char_index.0 + char_index.1.len_utf8();
                let (maybe_link_destination, remaining) = if next_char_index == segment.len() {
                    // If the closing sequence is the end of the segment, there will be no remaining segment.
                    (segment, Segment::empty_at(segment.end()))
                } else {
                    // Otherwise, it will be the result of splitting at the next index.
                    segment.split_at(next_char_index)
                };
                let Ok(bracketed_destination_segment) =
                    BracketedLinkDestinationSegment::try_from(maybe_link_destination)
                else {
                    return Err(segment);
                };

                return Ok(Extraction::new(bracketed_destination_segment, remaining));
            }
            previous_char = char_index;
        }

        Err(segment)
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
                        BracketedLinkDestinationSegment::try_from($segment.clone()),
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
                        BracketedLinkDestinationSegment::try_from($segment.clone()),
                        Ok(BracketedLinkDestinationSegment::new($segment))
                    );
                }
            };
        }

        failure_case!(should_reject_single_opening_bracket, Segment::first("<"));
        failure_case!(
            should_reject_missing_unescaped_closing_bracket,
            Segment::first(r"<\>")
        );
        failure_case!(
            should_reject_duplicate_closing_bracket,
            Segment::first(r"<>>")
        );
        failure_case!(
            should_reject_duplicate_opening_bracket,
            Segment::first(r"<<>")
        );

        success_case!(should_work_with_empty_brackets, Segment::first("<>"));
        success_case!(should_work_with_a_parenthesis, Segment::first("<)>"));
        success_case!(
            should_work_with_several_parentheses,
            Segment::first("<()(()))>")
        );
    }

    mod try_extract {
        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        BracketedLinkDestinationSegment::try_extract($segment.clone()),
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
                        BracketedLinkDestinationSegment::try_extract($segment.clone()),
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

        failure_case!(should_fail_for_single_opening_bracket, Segment::first("<"));
        failure_case!(
            should_fail_if_the_closing_sequence_is_not_present_at_the_end,
            Segment::first("<hello")
        );
        failure_case!(
            should_fail_if_the_closing_sequence_is_escaped,
            Segment::first("<hello\\>")
        );
        failure_case!(
            should_fail_if_the_content_is_not_a_valid_link_destination,
            Segment::first("<\n>")
        );

        success_case!(
            should_work_with_valid_link_destination_and_no_remaining,
            Segment::first("<hello>"),
            BracketedLinkDestinationSegment::try_from(Segment::first("<hello>")).unwrap(),
            Segment::empty_at(location::Position::new(1, 8, 7))
        );
        success_case!(
            should_work_with_valid_link_destination_and_remaining_text,
            Segment::first("<hello> 'this is a link title'"),
            BracketedLinkDestinationSegment::try_from(Segment::first("<hello>")).unwrap(),
            Segment::new(location::Position::new(1, 8, 7), " 'this is a link title'")
        );
    }
}
