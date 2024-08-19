use std::sync::LazyLock;

use crate::Segment;

use super::{
    blank_line::BlankLineNode,
    error::Error,
    ingestion::{Consume, NodeState},
    BlockNode,
};

// TODO: unlike the paragraph's raw content, this one's formatting is preserved almost perfectly.
// New lines are new lines, spaces are not trimmed, etc...
// Blank lines are turned into a single newline.
// Leading and trailing blank lines are not included.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RawContent<'a> {
    segments: Vec<Segment<'a>>,
}

impl<'a> RawContent<'a> {
    fn push(&mut self, segment: Segment<'a>) {
        self.segments.push(segment);
    }
}

impl<'a> From<Vec<Segment<'a>>> for RawContent<'a> {
    fn from(segments: Vec<Segment<'a>>) -> Self {
        Self { segments }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentedCodeNode<'a> {
    segments: Vec<Segment<'a>>,
    raw_content: RawContent<'a>,
}

// The first line needs to include at least 4 spaces or one tab.
// The rest of the line is included in the raw content.
static REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^((?:[ ]{4})|(?:\t))(\s*\S.*\s)$").unwrap());

impl<'a> TryFrom<Segment<'a>> for IndentedCodeNode<'a> {
    type Error = Error;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        match REGEX.captures(segment.text()) {
            Some(captures) => {
                let segments = vec![segment];
                let raw_content_start = segment.start().walk(captures.get(1).unwrap().as_str());
                let raw_segment_text = captures.get(2).unwrap().as_str();
                Ok(Self {
                    segments,
                    raw_content: vec![Segment::new(raw_content_start, raw_segment_text)].into(),
                })
            }
            None => return Err(Error::invalid_segment()),
        }
    }
}

impl<'a> Consume<'a> for IndentedCodeNode<'a> {
    fn consume(&mut self, segment: Segment<'a>) -> NodeState<'a> {
        // We keep blank lines.
        if BlankLineNode::try_from(segment).is_ok() {
            self.segments.push(segment);
            self.raw_content.push(segment);
            return NodeState::InProgress;
        }

        match REGEX.captures(segment.text()) {
            Some(captures) => {
                self.segments.push(segment);
                let raw_content_start = segment.start().walk(captures.get(1).unwrap().as_str());
                let raw_content_text = captures.get(2).unwrap().as_str();
                self.raw_content
                    .push(Segment::new(raw_content_start, raw_content_text));
                NodeState::InProgress
            }
            //TODO: test can be interrupted by paragraph
            None => NodeState::InterruptedBy(BlockNode::from(segment)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from {
        use super::*;

        #[test]
        fn should_reject_empty_line() {
            let segment = Segment::first("");
            assert_eq!(
                IndentedCodeNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_blank_line() {
            let segment = Segment::first("    \n");
            assert_eq!(
                IndentedCodeNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_3_whitespaces_indent() {
            let segment = Segment::first("   Missing one space\n");
            assert_eq!(
                IndentedCodeNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_work_with_4_whitespaces_indent() {
            let segment = Segment::first("    This is indented code. Finally.\n");
            let node = IndentedCodeNode::try_from(segment).unwrap();
            assert_eq!(node.segments, vec![segment]);
            assert_eq!(
                node.raw_content,
                vec![Segment::new(
                    location::Position::new(1, 5, 4),
                    "This is indented code. Finally.\n"
                )]
                .into()
            );
        }

        #[test]
        fn should_work_with_a_tab_indent() {
            let segment = Segment::first("\tThis is indented code. Finally.\n");
            let node = IndentedCodeNode::try_from(segment).unwrap();
            assert_eq!(node.segments, vec![segment]);
            assert_eq!(
                node.raw_content,
                vec![Segment::new(
                    location::Position::new(1, 2, 1),
                    "This is indented code. Finally.\n"
                )]
                .into()
            );
        }
    }

    mod consume {
        use crate::block::tree::ParagraphNode;

        use super::*;

        #[test]
        fn should_work_with_several_lines() {
            let first_segment = Segment::first("    first line\n");
            let second_segment = Segment::new(first_segment.end(), "    second line\n");
            let mut node = IndentedCodeNode::try_from(first_segment).unwrap();
            assert!(node.consume(second_segment).in_progress());
            assert_eq!(
                node,
                IndentedCodeNode {
                    segments: vec![first_segment, second_segment],
                    raw_content: vec![
                        Segment::new(location::Position::new(1, 5, 4), "first line\n"),
                        Segment::new(location::Position::new(2, 5, 19), "second line\n"),
                    ]
                    .into(),
                }
            );
        }

        #[test]
        fn should_work_with_empty_lines_interspersed() {
            let first_segment = Segment::first("    first line\n");
            let second_segment = Segment::new(first_segment.end(), "\t\t\n");
            let third_segment = Segment::new(second_segment.end(), "    keep me!\n");
            let mut node = IndentedCodeNode::try_from(first_segment).unwrap();
            assert!(node.consume(second_segment).in_progress());
            assert!(node.consume(third_segment).in_progress());
            assert_eq!(
                node.segments,
                vec![first_segment, second_segment, third_segment]
            );
            assert_eq!(
                node.raw_content,
                vec![
                    Segment::new(location::Position::new(1, 5, 4), "first line\n"),
                    Segment::new(location::Position::new(2, 1, 15), "\t\t\n"),
                    Segment::new(location::Position::new(3, 5, 22), "keep me!\n"),
                ]
                .into()
            )
        }

        mod interruptions {
            use crate::block::tree::{AtxHeadingNode, FencedCodeNode, ThematicBreakNode};

            use super::*;

            #[test]
            fn should_be_interrupted_by_atx_heading() {
                let first_segment = Segment::first("    this is indented code\n");
                let second_segment = Segment::new(first_segment.end(), "# Heading\n");
                let mut node = IndentedCodeNode::try_from(first_segment).unwrap();
                let interrupting = node.consume(second_segment).unwrap_interrupting_node();
                assert_eq!(node.segments, vec![first_segment]);
                assert_eq!(
                    node.raw_content.segments,
                    vec![Segment::new(
                        location::Position::new(1, 5, 4),
                        "this is indented code\n"
                    )]
                );
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_atx_heading(),
                    AtxHeadingNode::try_from(second_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_fenced_code() {
                let first_segment = Segment::first("    this is indented code\n");
                let second_segment = Segment::new(first_segment.end(), "```\n");
                let mut node = IndentedCodeNode::try_from(first_segment).unwrap();
                let interrupting = node.consume(second_segment).unwrap_interrupting_node();
                assert_eq!(node.segments, vec![first_segment]);
                assert_eq!(
                    node.raw_content.segments,
                    vec![Segment::new(
                        location::Position::new(1, 5, 4),
                        "this is indented code\n"
                    )]
                );
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_fenced_code(),
                    FencedCodeNode::try_from(second_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_paragraph() {
                let first_segment = Segment::first("    this is indented code\n");
                let second_segment = Segment::new(first_segment.end(), "This is a paragraph\n");
                let mut node = IndentedCodeNode::try_from(first_segment).unwrap();
                let interrupting = node.consume(second_segment).unwrap_interrupting_node();
                assert_eq!(node.segments, vec![first_segment]);
                assert_eq!(
                    node.raw_content.segments,
                    vec![Segment::new(
                        location::Position::new(1, 5, 4),
                        "this is indented code\n"
                    )]
                );
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_paragraph(),
                    ParagraphNode::try_from(second_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_thematic_break() {
                let first_segment = Segment::first("    this is indented code\n");
                let second_segment = Segment::new(first_segment.end(), "---\n");
                let mut node = IndentedCodeNode::try_from(first_segment).unwrap();
                let interrupting = node.consume(second_segment).unwrap_interrupting_node();
                assert_eq!(node.segments, vec![first_segment]);
                assert_eq!(
                    node.raw_content.segments,
                    vec![Segment::new(
                        location::Position::new(1, 5, 4),
                        "this is indented code\n"
                    )]
                );
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_thematic_break(),
                    ThematicBreakNode::try_from(second_segment).unwrap()
                );
            }
        }
    }
}
