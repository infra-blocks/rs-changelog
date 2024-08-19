use std::sync::LazyLock;

use crate::Segment;

use super::{
    error::Error,
    ingestion::{Consume, NodeState},
    BlockNode,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreakNode<'a> {
    segment: Segment<'a>,
}

impl<'a> ThematicBreakNode<'a> {
    pub fn segment(&self) -> &Segment<'a> {
        &self.segment
    }
}

// Thematic breaks are three or more matching -, _, or * characters. They can be preceded by up to 3 spaces
// and followed by any amount. They can also be interspersed with spaces. No other characters are allowed on the same line.
static REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^[ ]{0,3}(?:(?:_[ \t]*){3,}|(?:-[ \t]*){3,}|(?:\*[ \t]*){3,})\n$").unwrap()
});

impl<'a> TryFrom<Segment<'a>> for ThematicBreakNode<'a> {
    type Error = Error;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if REGEX.is_match(&segment.text()) {
            Ok(Self { segment })
        } else {
            Err(Error::invalid_segment())
        }
    }
}

impl<'a> Consume<'a> for ThematicBreakNode<'a> {
    fn consume(&mut self, segment: Segment<'a>) -> NodeState<'a> {
        // Thematic breaks are only one segment, so we always dispatch to the next block on second
        // segment.
        NodeState::InterruptedBy(BlockNode::from(segment))
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
                ThematicBreakNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_blank_line() {
            let segment = Segment::first("  \n");
            assert_eq!(
                ThematicBreakNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_tab_indent() {
            let segment = Segment::first("\t---\n");
            assert_eq!(
                ThematicBreakNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_four_spaces_indent() {
            let segment = Segment::first("    ---\n");
            assert_eq!(
                ThematicBreakNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_non_consecutive_tokens() {
            let segment = Segment::first(" -_*\n");
            assert_eq!(
                ThematicBreakNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_with_presence_of_other_characters() {
            let segment = Segment::first("---a\n");
            assert_eq!(
                ThematicBreakNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_work_with_three_underscores() {
            let first_segment = Segment::first("___\n");
            let node = ThematicBreakNode::try_from(first_segment).unwrap();
            assert_eq!(node.segment, first_segment);
        }

        #[test]
        fn should_work_with_four_underscores() {
            let first_segment = Segment::first("____\n");
            let node = ThematicBreakNode::try_from(first_segment).unwrap();
            assert_eq!(node.segment, first_segment);
        }

        #[test]
        fn should_work_with_three_hyphens() {
            let first_segment = Segment::first("---\n");
            let node = ThematicBreakNode::try_from(first_segment).unwrap();
            assert_eq!(node.segment, first_segment);
        }

        #[test]
        fn should_work_with_four_hyphens() {
            let first_segment = Segment::first("----\n");
            let node = ThematicBreakNode::try_from(first_segment).unwrap();
            assert_eq!(node.segment, first_segment);
        }

        #[test]
        fn should_work_with_three_asterisks() {
            let first_segment = Segment::first("***\n");
            let node = ThematicBreakNode::try_from(first_segment).unwrap();
            assert_eq!(node.segment, first_segment);
        }

        #[test]
        fn should_work_with_four_asterisks() {
            let first_segment = Segment::first("****\n");
            let node = ThematicBreakNode::try_from(first_segment).unwrap();
            assert_eq!(node.segment, first_segment);
        }

        #[test]
        fn should_work_with_three_spaces_indent() {
            let first_segment = Segment::first("   ---\n");
            let node = ThematicBreakNode::try_from(first_segment).unwrap();
            assert_eq!(node.segment, first_segment);
        }

        #[test]
        fn should_work_with_trailing_whitespace() {
            let first_segment = Segment::first("--- \t \n");
            let node = ThematicBreakNode::try_from(first_segment).unwrap();
            assert_eq!(node.segment, first_segment);
        }

        #[test]
        fn should_work_with_spaces_interspersed() {
            let first_segment = Segment::first(" - - -\n");
            let node = ThematicBreakNode::try_from(first_segment).unwrap();
            assert_eq!(node.segment, first_segment);
        }
    }

    mod consume {
        use crate::block::tree::{
            blank_line::BlankLineNode, FencedCodeNode, IndentedCodeNode, ParagraphNode,
        };

        use super::*;

        #[test]
        fn should_be_interrupted_by_blank_line() {
            let first_segment = Segment::first("---\n");
            let second_segment = Segment::new(first_segment.end(), "\n");
            let mut node = ThematicBreakNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_blank_line(),
                BlankLineNode::try_from(second_segment).unwrap()
            );
        }

        #[test]
        fn should_be_interrupted_by_fenced_code() {
            let first_segment = Segment::first("---\n");
            let second_segment = Segment::new(first_segment.end(), "~~~\n");
            let mut node = ThematicBreakNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_fenced_code(),
                FencedCodeNode::try_from(second_segment).unwrap()
            );
        }

        #[test]
        fn should_be_interrupted_by_indented_code() {
            let first_segment = Segment::first("---\n");
            let second_segment = Segment::new(first_segment.end(), "\tlet jew = 5;\n");
            let mut node = ThematicBreakNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_indented_code(),
                IndentedCodeNode::try_from(second_segment).unwrap()
            );
        }

        #[test]
        fn should_be_interrupted_by_paragraph() {
            let first_segment = Segment::first("---\n");
            let second_segment = Segment::new(first_segment.end(), "Is this a paragraph?\n");
            let mut node = ThematicBreakNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_paragraph(),
                ParagraphNode::try_from(second_segment).unwrap()
            );
        }

        #[test]
        fn should_be_interrupted_by_thematic_break() {
            let first_segment = Segment::first("---\n");
            let second_segment = Segment::new(first_segment.end(), "*****\n");
            let mut node = ThematicBreakNode::try_from(first_segment).unwrap();
            let interrupting = node.consume(second_segment).unwrap_interrupting_node();
            assert_eq!(
                interrupting.unwrap_leaf().unwrap_thematic_break(),
                ThematicBreakNode::try_from(second_segment).unwrap()
            );
        }
    }
}
