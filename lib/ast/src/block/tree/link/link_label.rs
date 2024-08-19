use std::sync::LazyLock;

use crate::{block::tree::error::Error, Segment};

/// This struct represents a link lable as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#link-label).
///
/// It can be constructed from a [Segment] using the [TryFrom] trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkLabel<'a> {
    pub segment: Segment<'a>,
}

impl<'a> LinkLabel<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }

    /// A link label can have at most 999 characters inside the square brackets.
    fn valid_character_count(segment: Segment<'a>) -> bool {
        // Including the square brackets for simplicity: 999 + 2 = 1001
        segment.text().chars().count() <= 1001
    }

    // TODO: make sure this isn't available in the public API.
    /// Returns an embeddable regex string that matches a link label. No capture groups are used,
    /// but the caller shouldn't assume that. Always use named capture groups when embedding regexes.
    pub(crate) fn inline_regex_str() -> &'static str {
        INLINE_REGEX.as_str()
    }
}

/// This regex is used to parse and validate that a segment is a link label.
/// It embeds the "inline regex" and makes sure the entirety of the segment is a match.
static REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", LinkLabel::inline_regex_str()).as_str()).unwrap()
});

/// This regex contains the logic of whether text is a match or not, and it is meant to be embedded
/// in another regex that validates the rest of the context.
static INLINE_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"\[(?:\s*(?:(?:\\.)|(?:[^\s\\\[\]])))+\s*]").unwrap());

impl<'a> TryFrom<Segment<'a>> for LinkLabel<'a> {
    type Error = Error;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if REGEX.is_match(segment.text()) && Self::valid_character_count(segment) {
            return Ok(Self::new(segment));
        }
        return Err(Error::invalid_segment());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from {
        use super::*;

        #[test]
        fn should_reject_empty_segment() {
            let segment = Segment::default();
            assert_eq!(LinkLabel::try_from(segment), Err(Error::invalid_segment()));
        }

        #[test]
        fn should_reject_blank_line() {
            let segment = Segment::first("\n");
            assert_eq!(LinkLabel::try_from(segment), Err(Error::invalid_segment()));
        }

        #[test]
        fn should_reject_missing_closing_bracket() {
            let segment = Segment::first("[a");
            assert_eq!(LinkLabel::try_from(segment), Err(Error::invalid_segment()));
        }

        #[test]
        fn should_reject_missing_opening_backet() {
            let segment = Segment::first("a]");
            assert_eq!(LinkLabel::try_from(segment), Err(Error::invalid_segment()));
        }

        #[test]
        fn should_reject_leading_whitespace() {
            let segment = Segment::first(" [a]");
            assert_eq!(LinkLabel::try_from(segment), Err(Error::invalid_segment()));
        }

        #[test]
        fn should_reject_trailing_whitespace() {
            let segment = Segment::first("[a] ");
            assert_eq!(LinkLabel::try_from(segment), Err(Error::invalid_segment()));
        }

        #[test]
        fn should_reject_empty_link_label() {
            let segment = Segment::first("[]");
            assert_eq!(LinkLabel::try_from(segment), Err(Error::invalid_segment()));
        }

        #[test]
        fn should_reject_whitespace_link_label() {
            let segment = Segment::first("[ \t ]");
            assert_eq!(LinkLabel::try_from(segment), Err(Error::invalid_segment()));
        }

        #[test]
        fn should_reject_multiple_closing_brackets() {
            let segment = Segment::first("[a]]");
            assert_eq!(LinkLabel::try_from(segment), Err(Error::invalid_segment()));
        }

        #[test]
        fn should_reject_multiple_opening_brackets() {
            let segment = Segment::first("[[a]");
            assert_eq!(LinkLabel::try_from(segment), Err(Error::invalid_segment()));
        }

        #[test]
        fn should_reject_verbose_label() {
            // This segment has 1000 internal characters, spread over 10 lines of 100 characters for readability.
            let segment = Segment::first(concat!("[",
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
             "]"));
            assert_eq!(LinkLabel::try_from(segment), Err(Error::invalid_segment()));
        }

        #[test]
        fn should_work_with_a_simple_link_label() {
            let segment = Segment::first("[a]");
            let node = LinkLabel::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
        }

        #[test]
        fn should_work_with_included_whitespace() {
            let segment = Segment::first("[ a ]");
            let node = LinkLabel::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
        }

        #[test]
        fn should_work_with_backslash() {
            let segment = Segment::first(r"[\\]");
            let node = LinkLabel::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
        }

        #[test]
        fn should_work_with_escaped_closing_bracket() {
            let segment = Segment::first(r"[\]]");
            let node = LinkLabel::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
        }

        #[test]
        fn should_work_with_escaped_opening_bracket() {
            let segment = Segment::first(r"[\[]");
            let node = LinkLabel::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
        }

        #[test]
        fn should_work_with_several_words() {
            let segment = Segment::first("[a b c]");
            let node = LinkLabel::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
        }

        #[test]
        fn should_work_with_999_characters() {
            // This segment has 999 internal characters, one less than the test case above.
            let segment = Segment::first(concat!("[",
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
             "]"));
            let node = LinkLabel::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
        }
    }
}
