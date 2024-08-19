use std::{iter, sync::LazyLock};

use crate::Segment;

use super::{
    error::Error,
    ingestion::{Consume, NodeState},
    BlockNode, ParagraphNode,
};

/// This struct represents a [Setext](https://spec.commonmark.org/0.31.2/#setext-headings) heading node.
///
/// Unlike most nodes, Setext headings don't implement the [TryFrom] trait for a single segment. This is
/// because they always start off as paragraphs, and then that possibly morphs into a Setext heading
/// when we encounter an underline segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetextHeadingNode<'a> {
    /// Either 1 or 2, depending on if the underline is `=` or `-`, respectively
    level: u8,
    segments: Vec<Segment<'a>>,
}

impl<'a> SetextHeadingNode<'a> {
    /// Returns the level of this Setext heading.
    pub fn level(&self) -> u8 {
        self.level
    }

    /// Returns the segments of this Setext heading.
    pub fn segments(&self) -> &[Segment<'a>] {
        &self.segments
    }
}

static UNDERLINE_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[ ]{0,3}(?:(=)+|(-)+)\s*?\n$").unwrap());

impl<'a> TryFrom<(&ParagraphNode<'a>, Segment<'a>)> for SetextHeadingNode<'a> {
    type Error = Error;

    fn try_from(
        (paragraph, segment): (&ParagraphNode<'a>, Segment<'a>),
    ) -> Result<Self, Self::Error> {
        match UNDERLINE_REGEX.captures(segment.text()) {
            Some(captures) => {
                let level = if captures.get(1).is_some() { 1 } else { 2 };
                Ok(Self {
                    level,
                    segments: paragraph
                        .segments
                        .iter()
                        .chain(iter::once(&segment))
                        .copied()
                        .collect(),
                })
            }
            None => Err(Error::invalid_segment()),
        }
    }
}

impl<'a> Consume<'a> for SetextHeadingNode<'a> {
    fn consume(&mut self, segment: Segment<'a>) -> NodeState<'a> {
        NodeState::InterruptedBy(BlockNode::from(segment))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from {
        use super::*;

        #[test]
        fn should_reject_an_empty_segment() {
            let first_segment = Segment::first("aaa\n");
            let second_segment = Segment::empty_at(first_segment.end());
            let paragraph = ParagraphNode::try_from(first_segment).unwrap();
            assert_eq!(
                SetextHeadingNode::try_from((&paragraph, second_segment)),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_a_whitespace_segment() {
            let first_segment = Segment::first("aaa\n");
            let second_segment = Segment::new(first_segment.end(), "\t\n");
            let paragraph = ParagraphNode::try_from(first_segment).unwrap();
            assert_eq!(
                SetextHeadingNode::try_from((&paragraph, second_segment)),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_an_underline_segment_with_other_characters() {
            let first_segment = Segment::first("aaa\n");
            let second_segment = Segment::new(first_segment.end(), "===a\n");
            let paragraph = ParagraphNode::try_from(first_segment).unwrap();
            assert_eq!(
                SetextHeadingNode::try_from((&paragraph, second_segment)),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_work_with_equals_underline() {
            let first_segment = Segment::first("aaa\n");
            let second_segment = Segment::new(first_segment.end(), "===\n");
            let paragraph = ParagraphNode::try_from(first_segment).unwrap();
            let setext_heading = SetextHeadingNode::try_from((&paragraph, second_segment)).unwrap();
            assert_eq!(setext_heading.level(), 1);
            assert_eq!(setext_heading.segments(), &[first_segment, second_segment]);
        }

        #[test]
        fn should_work_with_hyphens_underline() {
            let first_segment = Segment::first("aaa\n");
            let second_segment = Segment::new(first_segment.end(), "---\n");
            let paragraph = ParagraphNode::try_from(first_segment).unwrap();
            let setext_heading = SetextHeadingNode::try_from((&paragraph, second_segment)).unwrap();
            assert_eq!(setext_heading.level(), 2);
            assert_eq!(setext_heading.segments(), &[first_segment, second_segment]);
        }

        #[test]
        fn should_work_with_a_single_character_underline() {
            let first_segment = Segment::first("aaa\n");
            let second_segment = Segment::new(first_segment.end(), "=\n");
            let paragraph = ParagraphNode::try_from(first_segment).unwrap();
            let setext_heading = SetextHeadingNode::try_from((&paragraph, second_segment)).unwrap();
            assert_eq!(setext_heading.level(), 1);
            assert_eq!(setext_heading.segments(), &[first_segment, second_segment]);
        }

        #[test]
        fn should_work_with_many_characters_underline() {
            let first_segment = Segment::first("aaa\n");
            let second_segment = Segment::new(first_segment.end(), "============\n");
            let paragraph = ParagraphNode::try_from(first_segment).unwrap();
            let setext_heading = SetextHeadingNode::try_from((&paragraph, second_segment)).unwrap();
            assert_eq!(setext_heading.level(), 1);
            assert_eq!(setext_heading.segments(), &[first_segment, second_segment]);
        }

        #[test]
        fn should_work_with_3_spaces_before_the_underline() {
            let first_segment = Segment::first("aaa\n");
            let second_segment = Segment::new(first_segment.end(), "   ===\n");
            let paragraph = ParagraphNode::try_from(first_segment).unwrap();
            let setext_heading = SetextHeadingNode::try_from((&paragraph, second_segment)).unwrap();
            assert_eq!(setext_heading.level(), 1);
            assert_eq!(setext_heading.segments(), &[first_segment, second_segment]);
        }

        #[test]
        fn should_work_with_trailing_whitespaces_underline() {
            let first_segment = Segment::first("aaa\n");
            let second_segment = Segment::new(first_segment.end(), "===  \n");
            let paragraph = ParagraphNode::try_from(first_segment).unwrap();
            let setext_heading = SetextHeadingNode::try_from((&paragraph, second_segment)).unwrap();
            assert_eq!(setext_heading.level(), 1);
            assert_eq!(setext_heading.segments(), &[first_segment, second_segment]);
        }

        #[test]
        fn should_work_with_multiline_paragraph() {
            let first_segment = Segment::first("aaa\n");
            // Continuation line on paragraphs can have a lot of whitespaces.
            let second_segment = Segment::new(
                first_segment.end(),
                "         hello this is a continuation line\n",
            );
            let third_segment = Segment::new(second_segment.end(), "===\n");
            let mut paragraph = ParagraphNode::try_from(first_segment).unwrap();
            assert!(paragraph.consume(second_segment).in_progress());
            let setext_heading = SetextHeadingNode::try_from((&paragraph, third_segment)).unwrap();
            assert_eq!(setext_heading.level(), 1);
            assert_eq!(
                setext_heading.segments(),
                &[first_segment, second_segment, third_segment]
            );
        }
    }

    mod consume {
        use super::*;

        mod interruptions {
            use crate::block::tree::{
                blank_line::BlankLineNode, AtxHeadingNode, FencedCodeNode, IndentedCodeNode,
                ThematicBreakNode,
            };

            use super::*;

            #[test]
            fn should_be_interrupted_by_atx_heading() {
                let first_segment = Segment::first("aaa\n");
                let second_segment = Segment::new(first_segment.end(), "===\n");
                let third_segment =
                    Segment::new(second_segment.end(), "# Atx Heading Right Here\n");
                let paragraph = ParagraphNode::try_from(first_segment).unwrap();
                let mut setext_heading =
                    SetextHeadingNode::try_from((&paragraph, second_segment)).unwrap();
                assert_eq!(
                    setext_heading
                        .consume(third_segment)
                        .unwrap_interrupting_node()
                        .unwrap_leaf()
                        .unwrap_atx_heading(),
                    AtxHeadingNode::try_from(third_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_blank_line() {
                let first_segment = Segment::first("aaa\n");
                let second_segment = Segment::new(first_segment.end(), "===\n");
                let third_segment = Segment::new(second_segment.end(), "\n");
                let paragraph = ParagraphNode::try_from(first_segment).unwrap();
                let mut setext_heading =
                    SetextHeadingNode::try_from((&paragraph, second_segment)).unwrap();
                assert_eq!(
                    setext_heading
                        .consume(third_segment)
                        .unwrap_interrupting_node()
                        .unwrap_leaf()
                        .unwrap_blank_line(),
                    BlankLineNode::try_from(third_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_fenced_code() {
                let first_segment = Segment::first("aaa\n");
                let second_segment = Segment::new(first_segment.end(), "===\n");
                let third_segment = Segment::new(second_segment.end(), "```\n");
                let paragraph = ParagraphNode::try_from(first_segment).unwrap();
                let mut setext_heading =
                    SetextHeadingNode::try_from((&paragraph, second_segment)).unwrap();
                assert_eq!(
                    setext_heading
                        .consume(third_segment)
                        .unwrap_interrupting_node()
                        .unwrap_leaf()
                        .unwrap_fenced_code(),
                    FencedCodeNode::try_from(third_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_indented_code() {
                let first_segment = Segment::first("aaa\n");
                let second_segment = Segment::new(first_segment.end(), "===\n");
                let third_segment = Segment::new(second_segment.end(), "    hello\n");
                let paragraph = ParagraphNode::try_from(first_segment).unwrap();
                let mut setext_heading =
                    SetextHeadingNode::try_from((&paragraph, second_segment)).unwrap();
                assert_eq!(
                    setext_heading
                        .consume(third_segment)
                        .unwrap_interrupting_node()
                        .unwrap_leaf()
                        .unwrap_indented_code(),
                    IndentedCodeNode::try_from(third_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_paragraph() {
                let first_segment = Segment::first("aaa\n");
                let second_segment = Segment::new(first_segment.end(), "===\n");
                let third_segment = Segment::new(second_segment.end(), "hello\n");
                let paragraph = ParagraphNode::try_from(first_segment).unwrap();
                let mut setext_heading =
                    SetextHeadingNode::try_from((&paragraph, second_segment)).unwrap();
                assert_eq!(
                    setext_heading
                        .consume(third_segment)
                        .unwrap_interrupting_node()
                        .unwrap_leaf()
                        .unwrap_paragraph(),
                    ParagraphNode::try_from(third_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_thematic_break() {
                let first_segment = Segment::first("aaa\n");
                let second_segment = Segment::new(first_segment.end(), "===\n");
                let third_segment = Segment::new(second_segment.end(), "---\n");
                let paragraph = ParagraphNode::try_from(first_segment).unwrap();
                let mut setext_heading =
                    SetextHeadingNode::try_from((&paragraph, second_segment)).unwrap();
                assert_eq!(
                    setext_heading
                        .consume(third_segment)
                        .unwrap_interrupting_node()
                        .unwrap_leaf()
                        .unwrap_thematic_break(),
                    ThematicBreakNode::try_from(third_segment).unwrap()
                );
            }
        }
    }
}
