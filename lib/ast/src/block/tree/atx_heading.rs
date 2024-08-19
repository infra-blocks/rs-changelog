use std::sync::LazyLock;

use crate::Segment;

use super::{
    error::Error,
    ingestion::{Consume, NodeState},
    BlockNode,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtxHeadingNode<'a> {
    segment: Segment<'a>,
    raw_content: Option<Segment<'a>>,
    level: u8,
}

impl<'a> AtxHeadingNode<'a> {
    pub fn segment(&self) -> &Segment<'a> {
        &self.segment
    }
    pub fn level(&self) -> u8 {
        self.level
    }
}

// This regex is quite complicated to understand. Besides the trivial part of the leading spaces
// and hashes, it reads as such:
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
    regex::Regex::new(r"(^[ ]{0,3}(#{1,6}))(?:(?:\s+#*)|(?:(\s+)(\S.*?))(?:\s+#*)?)?\s*?\n$")
        .unwrap()
});

impl<'a> TryFrom<Segment<'a>> for AtxHeadingNode<'a> {
    type Error = Error;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        match REGEX.captures(segment.text()) {
            Some(capture) => {
                let level = capture.get(2).unwrap().as_str().len().try_into().unwrap();
                let raw_content = match capture.get(4) {
                    Some(raw_content_text_match) => {
                        // This is the prefix including the first whitespaces and the leading hashes.
                        let heading_prefix = capture.get(1).unwrap().as_str();
                        // This is the mandatory whitespace between the hashes and the content.
                        let whitespace_prefix = capture.get(3).unwrap().as_str();
                        let raw_content_start =
                            segment.start().walk(heading_prefix).walk(whitespace_prefix);
                        Some(Segment::new(
                            raw_content_start,
                            raw_content_text_match.as_str(),
                        ))
                    }
                    None => None,
                };

                Ok(Self {
                    segment,
                    raw_content,
                    level,
                })
            }
            None => Err(Error::invalid_segment()),
        }
    }
}

impl<'a> Consume<'a> for AtxHeadingNode<'a> {
    fn consume(&mut self, segment: Segment<'a>) -> NodeState<'a> {
        NodeState::InterruptedBy(BlockNode::from(segment))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod try_from {
        use super::*;

        #[test]
        fn should_reject_empty_segment() {
            let segment = Segment::first("");
            assert_eq!(
                AtxHeadingNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_blank_line() {
            let segment = Segment::first("\n");
            assert_eq!(
                AtxHeadingNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_tab_indent() {
            let segment = Segment::first("\t# Heading\n");
            assert_eq!(
                AtxHeadingNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_4_whitespaces_prefix() {
            let segment = Segment::first("    # Heading\n");
            assert_eq!(
                AtxHeadingNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_missing_whitespace_before_content() {
            let segment = Segment::first("#hashtag\n");
            assert_eq!(
                AtxHeadingNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_if_not_just_hash_before_content() {
            let segment = Segment::first("#5 Heading\n");
            assert_eq!(
                AtxHeadingNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_7_hashes() {
            let segment = Segment::first("####### Heading\n");
            assert_eq!(
                AtxHeadingNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_escaped_hash() {
            let segment = Segment::first(r"\## Heading\n");
            assert_eq!(
                AtxHeadingNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_work_with_simple_case() {
            let segment = Segment::first("# Heading\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 1);
            assert_eq!(
                node.raw_content,
                Some(Segment::new(location::Position::new(1, 3, 2), "Heading"))
            );
        }

        #[test]
        fn should_work_with_2_hashes() {
            let segment = Segment::first("## Heading\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 2);
            assert_eq!(
                node.raw_content,
                Some(Segment::new(location::Position::new(1, 4, 3), "Heading"))
            );
        }

        #[test]
        fn should_work_with_3_hashes() {
            let segment = Segment::first("### Heading\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 3);
            assert_eq!(
                node.raw_content,
                Some(Segment::new(location::Position::new(1, 5, 4), "Heading"))
            );
        }

        #[test]
        fn should_work_with_4_hashes() {
            let segment = Segment::first("#### Heading\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 4);
            assert_eq!(
                node.raw_content,
                Some(Segment::new(location::Position::new(1, 6, 5), "Heading"))
            );
        }

        #[test]
        fn should_work_with_5_hashes() {
            let segment = Segment::first("##### Heading\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 5);
            assert_eq!(
                node.raw_content,
                Some(Segment::new(location::Position::new(1, 7, 6), "Heading"))
            );
        }

        #[test]
        fn should_work_with_6_hashes() {
            let segment = Segment::first("###### Heading\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 6);
            assert_eq!(
                node.raw_content,
                Some(Segment::new(location::Position::new(1, 8, 7), "Heading"))
            );
        }

        #[test]
        fn should_work_with_3_spaces_indent() {
            let segment = Segment::first("   # Heading\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 1);
            assert_eq!(
                node.raw_content,
                Some(Segment::new(location::Position::new(1, 6, 5), "Heading"))
            );
        }

        #[test]
        fn should_work_with_trailing_hashes() {
            // Whitespaces are also allowed after trailing hashes
            let segment = Segment::first("# Heading ###  \t  \n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 1);
            assert_eq!(
                node.raw_content,
                Some(Segment::new(location::Position::new(1, 3, 2), "Heading"))
            );
        }

        #[test]
        fn should_include_trailing_hash_in_content_if_missing_whitespace() {
            let segment = Segment::first("# Heading#\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 1);
            assert_eq!(
                node.raw_content,
                Some(Segment::new(location::Position::new(1, 3, 2), "Heading#"))
            );
        }

        #[test]
        fn should_work_with_empty_heading() {
            let segment = Segment::first("#\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 1);
            assert_eq!(node.raw_content, None);
        }

        #[test]
        fn should_work_with_blank_heading() {
            let segment = Segment::first("#       \n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 1);
            assert_eq!(node.raw_content, None);
        }

        #[test]
        fn should_work_with_empty_heading_and_trailing_hashes() {
            let segment = Segment::first("## ###\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 2);
            assert_eq!(node.raw_content, None);
        }

        #[test]
        fn should_work_with_hash_content() {
            let segment = Segment::first("# ### #\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 1);
            assert_eq!(
                node.raw_content,
                Some(Segment::new(location::Position::new(1, 3, 2), "###"))
            );
        }

        #[test]
        fn should_work_with_characters_after_what_appears_to_be_a_closing_sequence() {
            let segment = Segment::first("### foo ### b\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 3);
            assert_eq!(
                node.raw_content,
                Some(Segment::new(location::Position::new(1, 5, 4), "foo ### b"))
            );
        }

        #[test]
        fn should_treat_escaped_hash_as_content() {
            let segment = Segment::first("# Heading #\\##\n");
            let node = AtxHeadingNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
            assert_eq!(node.level, 1);
            assert_eq!(
                node.raw_content,
                Some(Segment::new(
                    location::Position::new(1, 3, 2),
                    // TODO: this should render as just a plain # when displaying.
                    "Heading #\\##"
                ))
            );
        }
    }

    mod consume {
        use crate::block::tree::{
            blank_line::BlankLineNode, FencedCodeNode, IndentedCodeNode, ParagraphNode,
            ThematicBreakNode,
        };

        use super::*;

        #[test]
        fn should_be_interrupted_by_another_atx_heading() {
            let first_segment = Segment::first("# Heading\n");
            let second_segment = Segment::new(first_segment.end(), "## Subsequent Heading\n");
            let mut node = AtxHeadingNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(node.segment, first_segment);
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_atx_heading(),
                AtxHeadingNode::try_from(second_segment).unwrap()
            );
        }

        #[test]
        fn should_be_interrupted_by_blank_line() {
            let first_segment = Segment::first("# Heading\n");
            let second_segment = Segment::new(first_segment.end(), "\n");
            let mut node = AtxHeadingNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(node.segment, first_segment);
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_blank_line(),
                BlankLineNode::try_from(second_segment).unwrap()
            );
        }

        #[test]
        fn should_be_interrupted_by_fenced_code_block() {
            let first_segment = Segment::first("# Heading\n");
            let second_segment = Segment::new(first_segment.end(), "```\n");
            let mut node = AtxHeadingNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(node.segment, first_segment);
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_fenced_code(),
                FencedCodeNode::try_from(second_segment).unwrap()
            );
        }

        #[test]
        fn should_be_interrupted_by_indented_code() {
            let first_segment = Segment::first("# Heading\n");
            let second_segment = Segment::new(first_segment.end(), "    code\n");
            let mut node = AtxHeadingNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(node.segment, first_segment);
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_indented_code(),
                IndentedCodeNode::try_from(second_segment).unwrap()
            );
        }

        #[test]
        fn should_be_interrupted_by_paragraph() {
            let first_segment = Segment::first("# Heading\n");
            let second_segment = Segment::new(first_segment.end(), "Paragraph\n");
            let mut node = AtxHeadingNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(node.segment, first_segment);
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_paragraph(),
                ParagraphNode::try_from(second_segment).unwrap()
            );
        }

        // This test also showcases that atx headings have precedence over setext headings.
        // If an atx heading can be made, if it is followed by a valid setext underline, it still
        // remains an atx heading, and the next line is parsed as thematic break.
        #[test]
        fn should_be_interrupted_by_thematic_break() {
            let first_segment = Segment::first("# Heading\n");
            let second_segment = Segment::new(first_segment.end(), "---\n");
            let mut node = AtxHeadingNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(node.segment, first_segment);
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_thematic_break(),
                ThematicBreakNode::try_from(second_segment).unwrap()
            );
        }
    }
}
