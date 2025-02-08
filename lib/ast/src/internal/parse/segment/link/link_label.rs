use std::{char, sync::LazyLock};

use crate::internal::parse::try_extract::{Extraction, TryExtract};
use segment::{Segment, SegmentLike};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkLabelSegment<'a>(pub Segment<'a>);

impl<'a> LinkLabelSegment<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self(segment)
    }

    /// A link label can have at most 999 characters inside the square brackets.
    fn valid_character_count(segment: Segment<'a>) -> bool {
        // Including the square brackets for simplicity: 999 + 2 = 1001
        segment.text().chars().count() <= 1001
    }

    /// Returns true if the character is the opening character for this segment.
    pub fn is_opening_char(character: char) -> bool {
        character == '['
    }

    /// Returns true if the last two characters are an unescaped closing bracket.
    pub fn is_closing_sequence(second_to_last: char, last: char) -> bool {
        return second_to_last != '\\' && last == ']';
    }
}

/// This regex is used to parse and validate that a segment is a link label.
/// It embeds the "inline regex" and makes sure the entirety of the segment is a match.
static REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(format!("^{}$", INLINE_REGEX.as_str()).as_str()).unwrap());

/// This regex contains the logic of whether text is a match or not, and it is meant to be embedded
/// in another regex that validates the rest of the context.
static INLINE_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"\[(?:\s*(?:(?:\\.)|(?:[^\s\\\[\]])))+\s*]").unwrap());

impl<'a> TryFrom<Segment<'a>> for LinkLabelSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if REGEX.is_match(segment.text()) && Self::valid_character_count(segment) {
            Ok(Self::new(segment))
        } else {
            Err(segment)
        }
    }
}

impl<'a> TryExtract<Segment<'a>> for LinkLabelSegment<'a> {
    type Remaining = Segment<'a>;
    type Error = Segment<'a>;

    fn try_extract(segment: Segment<'a>) -> Result<Extraction<Self, Segment<'a>>, Self::Error> {
        let mut char_indices = segment.text().char_indices();
        let Some(first_char) = char_indices.next() else {
            return Err(segment);
        };

        if !LinkLabelSegment::is_opening_char(first_char.1) {
            return Err(segment);
        }

        let mut previous_char = first_char;
        while let Some(char_index) = char_indices.next() {
            if LinkLabelSegment::is_closing_sequence(previous_char.1, char_index.1) {
                let next_char_index = char_index.0 + char_index.1.len_utf8();
                let (maybe_link_label, remaining) = if next_char_index == segment.len() {
                    // If the closing sequence is the end of the segment, there will be no remaining segment.
                    (segment, Segment::empty_at(segment.end()))
                } else {
                    // Otherwise, it will be the result of splitting at the next index.
                    segment.split_at(next_char_index)
                };
                let Ok(link_label_segment) = LinkLabelSegment::try_from(maybe_link_label) else {
                    return Err(segment);
                };

                return Ok(Extraction::new(link_label_segment, remaining));
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
                    assert_eq!(LinkLabelSegment::try_from($segment.clone()), Err($segment));
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        LinkLabelSegment::try_from($segment.clone()),
                        Ok(LinkLabelSegment::new($segment))
                    );
                }
            };
        }

        failure_case!(should_reject_empty_segment, Segment::default());
        failure_case!(should_reject_blank_line, Segment::first("\n"));
        failure_case!(should_reject_missing_closing_bracket, Segment::first("[a"));
        failure_case!(should_reject_missing_opening_backet, Segment::first("a]"));
        failure_case!(should_reject_leading_whitespace, Segment::first(" [a]"));
        failure_case!(should_reject_trailing_whitespace, Segment::first("[a] "));
        failure_case!(should_reject_empty_link_label, Segment::first("[]"));
        failure_case!(
            should_reject_whitespace_link_label,
            Segment::first("[ \t ]")
        );
        failure_case!(
            should_reject_multiple_closing_brackets,
            Segment::first("[a]]")
        );
        failure_case!(
            should_reject_multiple_opening_brackets,
            Segment::first("[[a]")
        );
        failure_case!(
            should_reject_verbose_label,
            Segment::first(concat!("[",
        "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean m",
        "assa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec qu",
        "am felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec ",
        "pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a",
        ", venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibu",
        "s. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus. Aenean leo ligula, porttitor eu,",
        " consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus",
        ". Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum. Aenean imperdiet. Etiam ultricies",
        " nisi vel augue. Curabitur ullamcorper ultricies nisi. Nam eget dui. Etiam rhoncus. Maecenas tempus,",
        " tellus eget condimentum rhoncus, sem quam semper libero, sit amet adipiscing sem neque sed ipsum. N",
     "]")));

        success_case!(should_work_with_a_simple_link_label, Segment::first("[a]"));
        success_case!(
            should_work_with_included_whitespace,
            Segment::first("[ a ]")
        );
        success_case!(should_work_with_backslash, Segment::first(r"[\\]"));
        success_case!(
            should_work_with_escaped_closing_bracket,
            Segment::first(r"[\]]")
        );
        success_case!(
            should_work_with_escaped_opening_bracket,
            Segment::first(r"[\[]")
        );
        success_case!(should_work_with_several_words, Segment::first("[a b c]"));
        success_case!(
            should_work_with_999_characters,
            Segment::first(concat!("[",
                "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean m",
                "assa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec qu",
                "am felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec ",
                "pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a",
                ", venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibu",
                "s. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus. Aenean leo ligula, porttitor eu,",
                " consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus",
                ". Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum. Aenean imperdiet. Etiam ultricies",
                " nisi vel augue. Curabitur ullamcorper ultricies nisi. Nam eget dui. Etiam rhoncus. Maecenas tempus,",
                " tellus eget condimentum rhoncus, sem quam semper libero, sit amet adipiscing sem neque sed ipsum. ",
            "]"))
        );
    }

    mod try_extract {
        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        LinkLabelSegment::try_extract($segment.clone()),
                        Err($segment)
                    );
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected_link_label:expr, $expected_remaining:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        LinkLabelSegment::try_extract($segment.clone()),
                        Ok(Extraction::new($expected_link_label, $expected_remaining))
                    );
                }
            };
        }

        failure_case!(should_fail_for_empty_segment, Segment::default());
        failure_case!(should_fail_for_newline, Segment::first("\n"));
        failure_case!(
            should_fail_if_the_first_character_is_not_an_opening_bracket,
            Segment::first("(")
        );
        failure_case!(should_fail_for_single_opening_bracket, Segment::first("["));
        failure_case!(
            should_fail_if_the_closing_sequence_is_not_present_at_the_end,
            Segment::first("[hello")
        );
        failure_case!(
            should_fail_if_the_closing_sequence_is_escaped,
            Segment::first("[hello\\]")
        );
        failure_case!(
            should_fail_if_the_content_is_not_a_valid_link_label,
            Segment::first("[     ]")
        );

        success_case!(
            should_work_with_valid_link_label_and_no_remaining,
            Segment::first("[hello]"),
            LinkLabelSegment::try_from(Segment::first("[hello]")).unwrap(),
            Segment::empty_at(location::Position::new(1, 8, 7))
        );
        success_case!(
            should_work_with_valid_link_label_and_remaining_text,
            Segment::first("[hello]: extract/me"),
            LinkLabelSegment::try_from(Segment::first("[hello]")).unwrap(),
            Segment::new(location::Position::new(1, 8, 7), ": extract/me")
        );
    }
}
