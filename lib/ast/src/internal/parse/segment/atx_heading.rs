use std::sync::LazyLock;

use segment::{LineSegment, SegmentLike};

/// Represents an ATX heading segment.
///
/// ATX Headings only have one segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtxHeadingSegment<'a> {
    /// The source segment from which this struct was constructed.
    pub segment: LineSegment<'a>,
    /// The title of the heading, if any.
    pub title: Option<&'a str>,
    /// The level of the heading, from 1 to 6.
    pub level: u8,
}

impl<'a> AtxHeadingSegment<'a> {
    fn new(segment: LineSegment<'a>, title: Option<&'a str>, level: u8) -> Self {
        Self {
            segment,
            title,
            level,
        }
    }
}

impl<'a> From<AtxHeadingSegment<'a>> for LineSegment<'a> {
    fn from(heading: AtxHeadingSegment<'a>) -> LineSegment<'a> {
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

impl<'a> TryFrom<LineSegment<'a>> for AtxHeadingSegment<'a> {
    type Error = LineSegment<'a>;

    fn try_from(segment: LineSegment<'a>) -> Result<Self, Self::Error> {
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
        use segment::SegmentStrExt;

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

        failure_case!(should_reject_empty_segment, "".line());
        failure_case!(should_reject_blank_line, "\n".line());
        failure_case!(should_reject_tab_indent, "\t# Heading\n".line());
        failure_case!(
            should_reject_4_whitespaces_prefix,
            "    # Heading\n".line()
        );
        failure_case!(
            should_reject_missing_whitespace_before_content,
            "#hashtag\n".line()
        );
        failure_case!(
            should_reject_if_not_just_hash_before_content,
            "#5 Heading\n".line()
        );
        failure_case!(should_reject_7_hashes, "####### Heading\n".line());
        failure_case!(should_reject_escaped_hash, r"\## Heading\n".line());

        success_case!(
            should_work_with_simple_case,
            "# Heading\n".line(),
            AtxHeadingSegment::new("# Heading\n".line(), Some("Heading"), 1)
        );
        success_case!(
            should_work_with_2_hashes,
            "## Heading\n".line(),
            AtxHeadingSegment::new("## Heading\n".line(), Some("Heading"), 2)
        );
        success_case!(
            should_work_with_3_hashes,
            "### Heading\n".line(),
            AtxHeadingSegment::new("### Heading\n".line(), Some("Heading"), 3)
        );
        success_case!(
            should_work_with_4_hashes,
            "#### Heading\n".line(),
            AtxHeadingSegment::new("#### Heading\n".line(), Some("Heading"), 4)
        );
        success_case!(
            should_work_with_5_hashes,
            "##### Heading\n".line(),
            AtxHeadingSegment::new("##### Heading\n".line(), Some("Heading"), 5)
        );
        success_case!(
            should_work_with_6_hashes,
            "###### Heading\n".line(),
            AtxHeadingSegment::new("###### Heading\n".line(), Some("Heading"), 6)
        );
        success_case!(
            should_work_with_3_spaces_indent,
            "   # Heading\n".line(),
            AtxHeadingSegment::new("   # Heading\n".line(), Some("Heading"), 1)
        );
        success_case!(
            should_work_with_trailing_hashes,
            "# Heading ###  \t  \n".line(),
            AtxHeadingSegment::new("# Heading ###  \t  \n".line(), Some("Heading"), 1)
        );
        success_case!(
            should_include_trailing_hash_in_content_if_missing_whitespace,
            "# Heading#\n".line(),
            AtxHeadingSegment::new("# Heading#\n".line(), Some("Heading#"), 1)
        );
        success_case!(
            should_work_with_empty_heading_without_newline,
            "#".line(),
            AtxHeadingSegment::new("#".line(), None, 1)
        );
        success_case!(
            should_work_with_blank_heading,
            "#       \n".line(),
            AtxHeadingSegment::new("#       \n".line(), None, 1)
        );
        success_case!(
            should_work_with_empty_heading_and_trailing_hashes,
            "## ###\n".line(),
            AtxHeadingSegment::new("## ###\n".line(), None, 2)
        );
        success_case!(
            should_work_with_hash_content,
            "# ### #\n".line(),
            AtxHeadingSegment::new("# ### #\n".line(), Some("###"), 1)
        );
        success_case!(
            should_work_with_characters_after_what_appears_to_be_a_closing_sequence,
            "### foo ### b\n".line(),
            AtxHeadingSegment::new("### foo ### b\n".line(), Some("foo ### b"), 3)
        );
        success_case!(
            should_work_with_escaped_hash_as_content,
            "# Heading #\\##\n".line(),
            AtxHeadingSegment::new("# Heading #\\##\n".line(), Some("Heading #\\##"), 1)
        );
        success_case!(
            should_work_with_missing_eol,
            "# Heading".line(),
            AtxHeadingSegment::new("# Heading".line(), Some("Heading"), 1)
        );
    }
}
