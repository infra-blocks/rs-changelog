use std::sync::LazyLock;

use crate::{block::tree::error::Error, Segment};

/// This struct represents a link destination as described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#link-destination).
///
/// It can be constructed from a [Segment] using the [TryFrom] trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkDestination<'a> {
    segment: Segment<'a>,
}

impl<'a> LinkDestination<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }

    fn is_bracketed(segment: Segment<'a>) -> bool {
        BRACKETED_REGEX.is_match(segment.text())
    }

    fn is_freeform(segment: Segment<'a>) -> bool {
        FREEFORM_REGEX.is_match(segment.text()) && Self::parentheseses_balance(segment)
    }

    fn parentheseses_balance(segment: Segment<'a>) -> bool {
        // Ignore escaped parentheseses.
        let sanitized = segment.text().replace(r"\(", "").replace(r"\)", "");
        // Ensure the count of opening and closing parentheseses is equal.
        sanitized.chars().filter(|&c| c == '(').count()
            == sanitized.chars().filter(|&c| c == ')').count()
    }

    // TODO: make sure this isn't available in the public API.
    /// Returns an embeddable regex string that matches a link label. No capture groups are used,
    /// but the caller shouldn't assume that. Always use named capture groups when embedding regexes.
    /// In addition, matching this regex does *not* guarantee that the text is valid. Some additional
    /// validations still need to occur. Using this regex is not a replacement for [TryFrom].
    pub(crate) fn inline_regex_str() -> &'static str {
        INLINE_REGEX.as_str()
    }
}

// The bracketed variation is encased in angle brackets and can contain any
// character except for unescaped angle brackets.
static INLINE_BRACKETED_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"<(?:(?:\\<)|(?:\\>)|[^\\<>])*>").unwrap());
static BRACKETED_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_BRACKETED_REGEX.as_str()).as_str()).unwrap()
});

// The freeform variation does not include ASCII control characters,
// spaces and does not start with the '<' character.
static INLINE_FREEFORM_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[^< \x00-\x1F\x7F][^ \x00-\x1F\x7F]*").unwrap());
static FREEFORM_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(format!("^{}$", INLINE_FREEFORM_REGEX.as_str()).as_str()).unwrap()
});

static INLINE_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        format!(
            "(?:(?:{})|(?:{}))",
            INLINE_BRACKETED_REGEX.as_str(),
            INLINE_FREEFORM_REGEX.as_str()
        )
        .as_str(),
    )
    .unwrap()
});

impl<'a> TryFrom<Segment<'a>> for LinkDestination<'a> {
    type Error = Error;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if Self::is_bracketed(segment) || Self::is_freeform(segment) {
            return Ok(Self::new(segment));
        }
        return Err(Error::invalid_segment());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_reject_empty_segment() {
        let segment = Segment::empty_at(location::Position::first());
        let link_destination = LinkDestination::try_from(segment);
        assert!(link_destination.is_err());
    }

    mod bracketed {
        use super::*;

        #[test]
        fn should_reject_single_opening_bracket() {
            let segment = Segment::first("<");
            let link_destination = LinkDestination::try_from(segment);
            assert!(link_destination.is_err());
        }

        #[test]
        fn should_reject_missing_unescaped_closing_bracket() {
            let segment = Segment::first(r"<\>");
            let link_destination = LinkDestination::try_from(segment);
            assert!(link_destination.is_err());
        }

        #[test]
        fn should_reject_duplicate_closing_bracket() {
            let segment = Segment::first(r"<>>");
            let link_destination = LinkDestination::try_from(segment);
            assert!(link_destination.is_err());
        }

        #[test]
        fn should_reject_duplicate_opening_bracket() {
            let segment = Segment::first(r"<<>");
            let link_destination = LinkDestination::try_from(segment);
            assert!(link_destination.is_err());
        }

        #[test]
        fn should_work_with_empty_brackets() {
            let segment = Segment::first("<>");
            let link_destination = LinkDestination::try_from(segment).unwrap();
            assert_eq!(link_destination.segment, segment);
        }

        #[test]
        fn should_work_with_a_parenthesis() {
            let segment = Segment::first("<)>");
            let link_destination = LinkDestination::try_from(segment).unwrap();
            assert_eq!(link_destination.segment, segment);
        }

        #[test]
        fn should_work_with_several_parentheses() {
            // They don't need to be balanced neither.
            let segment = Segment::first("<()(()))>");
            let link_destination = LinkDestination::try_from(segment).unwrap();
            assert_eq!(link_destination.segment, segment);
        }
    }

    mod freeform {
        use super::*;

        #[test]
        fn should_reject_space() {
            let segment = Segment::first(" ");
            let link_destination = LinkDestination::try_from(segment);
            assert!(link_destination.is_err());
        }

        #[test]
        fn should_reject_leading_whitespace() {
            let segment = Segment::first(" a");
            let link_destination = LinkDestination::try_from(segment);
            assert!(link_destination.is_err());
        }

        #[test]
        fn should_reject_trailing_whitespace() {
            let segment = Segment::first("a ");
            let link_destination = LinkDestination::try_from(segment);
            assert!(link_destination.is_err());
        }

        #[test]
        fn should_reject_missing_closing_parenthesis() {
            let segment = Segment::first("(");
            let link_destination = LinkDestination::try_from(segment);
            assert!(link_destination.is_err());
        }

        #[test]
        fn should_reject_missing_opening_parenthesis() {
            let segment = Segment::first(")");
            let link_destination = LinkDestination::try_from(segment);
            assert!(link_destination.is_err());
        }

        #[test]
        fn should_work_with_character() {
            let segment = Segment::first("a");
            let link_destination = LinkDestination::try_from(segment).unwrap();
            assert_eq!(link_destination.segment, segment);
        }

        #[test]
        fn should_work_with_slash() {
            let segment = Segment::first("/");
            let link_destination = LinkDestination::try_from(segment).unwrap();
            assert_eq!(link_destination.segment, segment);
        }

        #[test]
        fn should_work_with_relative_path() {
            let segment = Segment::first("./relative/path.sftu");
            let link_destination = LinkDestination::try_from(segment).unwrap();
            assert_eq!(link_destination.segment, segment);
        }

        #[test]
        fn should_work_with_fragment_identifier() {
            let segment = Segment::first("#fragment");
            let link_destination = LinkDestination::try_from(segment).unwrap();
            assert_eq!(link_destination.segment, segment);
        }

        #[test]
        fn should_work_with_full_uri() {
            let segment = Segment::first("https://example.com?query=value#head-wallet");
            let link_destination = LinkDestination::try_from(segment).unwrap();
            assert_eq!(link_destination.segment, segment);
        }

        #[test]
        fn should_work_with_escaped_opening_parenthesis() {
            let segment = Segment::first(r"\(");
            let link_destination = LinkDestination::try_from(segment).unwrap();
            assert_eq!(link_destination.segment, segment);
        }

        #[test]
        fn should_work_with_escaped_closing_parenthesis() {
            let segment = Segment::first(r"\)");
            let link_destination = LinkDestination::try_from(segment).unwrap();
            assert_eq!(link_destination.segment, segment);
        }

        #[test]
        fn should_work_wit_balenced_parentheses() {
            let segment = Segment::first("(foo(and(bar)))");
            let link_destination = LinkDestination::try_from(segment).unwrap();
            assert_eq!(link_destination.segment, segment);
        }
    }
}
