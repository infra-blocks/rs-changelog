use std::sync::LazyLock;

use crate::Segment;

use super::{
    error::Error,
    ingestion::{Consume, NodeState},
    BlockNode,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlankLineNode<'a> {
    segment: Segment<'a>,
}

impl<'a> BlankLineNode<'a> {
    fn new(segment: Segment<'a>) -> Self {
        Self { segment }
    }

    pub fn segment(&self) -> &Segment<'a> {
        &self.segment
    }
}

// Only whitespace is allowed mfk, and at least one.
static REGEX: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"^\s+$").unwrap());

impl<'a> TryFrom<Segment<'a>> for BlankLineNode<'a> {
    type Error = Error;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        match REGEX.captures(segment.text()) {
            Some(_) => Ok(BlankLineNode::new(segment)),
            None => return Err(Error::invalid_segment()),
        }
    }
}

impl<'a> Consume<'a> for BlankLineNode<'a> {
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
        fn should_reject_empty() {
            let segment = Segment::first("");
            assert_eq!(
                BlankLineNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_line_with_a_char() {
            let segment = Segment::first("    a     \n");
            assert_eq!(
                BlankLineNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_work_with_one_whitespace() {
            let segment = Segment::first(" ");
            let node = BlankLineNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
        }

        #[test]
        fn should_work_with_any_whitespace() {
            let segment = Segment::first("\t\r\n ");
            let node = BlankLineNode::try_from(segment).unwrap();
            assert_eq!(node.segment, segment);
        }
    }

    // *Any* input should trigger an interrupt, as the BlankLineNode is a single segment node.
    mod consume {
        use crate::block::tree::{
            AtxHeadingNode, FencedCodeNode, IndentedCodeNode, ParagraphNode, ThematicBreakNode,
        };

        use super::*;

        #[test]
        fn should_be_interrupted_by_atx_heading() {
            let first_segment = Segment::first("    \n");
            let second_segment = Segment::new(first_segment.end(), "# Heading\n");
            let mut node = BlankLineNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_atx_heading(),
                AtxHeadingNode::try_from(second_segment).unwrap()
            );
        }

        #[test]
        fn should_be_interrupted_by_blank_line() {
            let first_segment = Segment::first("  \n");
            let second_segment = Segment::new(first_segment.end(), "  \n");
            let mut node = BlankLineNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_blank_line(),
                BlankLineNode {
                    segment: second_segment
                }
            );
        }

        #[test]
        fn should_be_interrupted_by_fenced_code() {
            let first_segment = Segment::first("    \n");
            let second_segment = Segment::new(first_segment.end(), "```\n");
            let mut node = BlankLineNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_fenced_code(),
                FencedCodeNode::try_from(second_segment).unwrap()
            );
        }

        #[test]
        fn should_be_interrupted_by_indented_code() {
            let first_segment = Segment::first("    \n");
            let second_segment = Segment::new(first_segment.end(), "    let x = 4;\n");
            let mut node = BlankLineNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_indented_code(),
                IndentedCodeNode::try_from(second_segment).unwrap()
            );
        }

        #[test]
        fn should_be_interrupted_by_paragraph() {
            let first_segment = Segment::first("    \n");
            let second_segment = Segment::new(first_segment.end(), "  Hello, world!\n");
            let mut node = BlankLineNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_paragraph(),
                ParagraphNode::try_from(second_segment).unwrap()
            );
        }

        #[test]
        fn should_be_interrupted_by_thematic_break() {
            let first_segment = Segment::first("    \n");
            let second_segment = Segment::new(first_segment.end(), "---\n");
            let mut node = BlankLineNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_thematic_break(),
                ThematicBreakNode::try_from(second_segment).unwrap()
            );
        }
    }
}
