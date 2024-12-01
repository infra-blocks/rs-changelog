use std::sync::LazyLock;

use crate::Segment;

/// Represents an ATX heading segment.
///
/// ATX Headings only have one segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtxHeadingSegment<'a> {
    /// The source segment from which this struct was constructed.
    pub segment: Segment<'a>,
    /// The title of the heading, if any.
    pub title: Option<&'a str>,
    /// The level of the heading, from 1 to 6.
    pub level: u8,
}

impl<'a> AtxHeadingSegment<'a> {
    fn new(segment: Segment<'a>, title: Option<&'a str>, level: u8) -> Self {
        Self {
            segment,
            title,
            level,
        }
    }
}

impl<'a> From<AtxHeadingSegment<'a>> for Segment<'a> {
    fn from(heading: AtxHeadingSegment<'a>) -> Segment<'a> {
        heading.segment
    }
}

// This regex reads as such:
// - Up to 3 whitespaces are allowed as indentation
// - Between 1 and 6 '#' characters must follow
// - The rest of the line is either a closing sequence or
// - Raw content followed by an optional closing sequence.
// The closing sequence is (?:\s+#*) and reads like this: at least one whitespace char followed by
// zero or more hash chars. For it to be a closing sequence, it can only be followed by whitespaces
// and tabs before the end of the line. This is a valid heading that matches that scenario:
// "## ###\n". The raw content should be empty here.
// In the second case, the raw content precedes the optional closing sequence. Typically, the raw content
// is made of at least one non whitespace character (like in "# Heading"). However, the raw content can also look exactly
// like a closing sequence (indeed, this is possible "# ### #\n")!
// Hence why the ordering is important and we try the closing sequence first, otherwise we could end
// up with a scenario where "## ###\n" means a raw content of "###", which is an invalid interpretation.
static REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(^[ ]{0,3}(#{1,6}))(?:(?:\s+#*)|(?:(\s+)(\S.*?))(?:\s+#*)?)?\s*$").unwrap()
});

impl<'a> TryFrom<Segment<'a>> for AtxHeadingSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        let Some(captures) = REGEX.captures(segment.text()) else {
            return Err(segment);
        };

        // The second capture corresonds to the hashes. Here, because we know the hashes
        // will never contain more than 6 symbols, we can safely convert the length of those to a u8.
        let level = captures.get(2).unwrap().as_str().len().try_into().unwrap();
        let title = match captures.get(4) {
            Some(title_match) => Some(title_match.as_str()),
            None => None,
        };

        Ok(AtxHeadingSegment::new(segment, title, level))
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
                    assert_eq!(AtxHeadingSegment::try_from($segment.clone()), Err($segment))
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(AtxHeadingSegment::try_from($segment), Ok($expected))
                }
            };
        }

        failure_case!(should_reject_empty_segment, Segment::first(""));
        failure_case!(should_reject_blank_line, Segment::first("\n"));
        failure_case!(should_reject_tab_indent, Segment::first("\t# Heading\n"));
        failure_case!(
            should_reject_4_whitespaces_prefix,
            Segment::first("    # Heading\n")
        );
        failure_case!(
            should_reject_missing_whitespace_before_content,
            Segment::first("#hashtag\n")
        );
        failure_case!(
            should_reject_if_not_just_hash_before_content,
            Segment::first("#5 Heading\n")
        );
        failure_case!(should_reject_7_hashes, Segment::first("####### Heading\n"));
        failure_case!(should_reject_escaped_hash, Segment::first(r"\## Heading\n"));

        success_case!(
            should_work_with_simple_case,
            Segment::first("# Heading\n"),
            AtxHeadingSegment::new(Segment::first("# Heading\n"), Some("Heading"), 1)
        );
        success_case!(
            should_work_with_2_hashes,
            Segment::first("## Heading\n"),
            AtxHeadingSegment::new(Segment::first("## Heading\n"), Some("Heading"), 2)
        );
        success_case!(
            should_work_with_3_hashes,
            Segment::first("### Heading\n"),
            AtxHeadingSegment::new(Segment::first("### Heading\n"), Some("Heading"), 3)
        );
        success_case!(
            should_work_with_4_hashes,
            Segment::first("#### Heading\n"),
            AtxHeadingSegment::new(Segment::first("#### Heading\n"), Some("Heading"), 4)
        );
        success_case!(
            should_work_with_5_hashes,
            Segment::first("##### Heading\n"),
            AtxHeadingSegment::new(Segment::first("##### Heading\n"), Some("Heading"), 5)
        );
        success_case!(
            should_work_with_6_hashes,
            Segment::first("###### Heading\n"),
            AtxHeadingSegment::new(Segment::first("###### Heading\n"), Some("Heading"), 6)
        );
        success_case!(
            should_work_with_3_spaces_indent,
            Segment::first("   # Heading\n"),
            AtxHeadingSegment::new(Segment::first("   # Heading\n"), Some("Heading"), 1)
        );
        success_case!(
            should_work_with_trailing_hashes,
            Segment::first("# Heading ###  \t  \n"),
            AtxHeadingSegment::new(Segment::first("# Heading ###  \t  \n"), Some("Heading"), 1)
        );
        success_case!(
            should_include_trailing_hash_in_content_if_missing_whitespace,
            Segment::first("# Heading#\n"),
            AtxHeadingSegment::new(Segment::first("# Heading#\n"), Some("Heading#"), 1)
        );
        success_case!(
            should_work_with_empty_heading,
            Segment::first("#\n"),
            AtxHeadingSegment::new(Segment::first("#\n"), None, 1)
        );
        success_case!(
            should_work_with_blank_heading,
            Segment::first("#       \n"),
            AtxHeadingSegment::new(Segment::first("#       \n"), None, 1)
        );
        success_case!(
            should_work_with_empty_heading_and_trailing_hashes,
            Segment::first("## ###\n"),
            AtxHeadingSegment::new(Segment::first("## ###\n"), None, 2)
        );
        success_case!(
            should_work_with_hash_content,
            Segment::first("# ### #\n"),
            AtxHeadingSegment::new(Segment::first("# ### #\n"), Some("###"), 1)
        );
        success_case!(
            should_work_with_characters_after_what_appears_to_be_a_closing_sequence,
            Segment::first("### foo ### b\n"),
            AtxHeadingSegment::new(Segment::first("### foo ### b\n"), Some("foo ### b"), 3)
        );
        success_case!(
            should_work_with_escaped_hash_as_content,
            Segment::first("# Heading #\\##\n"),
            AtxHeadingSegment::new(
                Segment::first("# Heading #\\##\n"),
                Some("Heading #\\##"),
                1
            )
        );
        success_case!(
            should_work_with_missing_eol,
            Segment::first("# Heading"),
            AtxHeadingSegment::new(Segment::first("# Heading"), Some("Heading"), 1)
        );
    }
}
