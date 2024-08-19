use std::sync::LazyLock;

use crate::Segment;

use super::{
    error::Error,
    ingestion::{Consume, NodeState},
    BlockNode,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Fence {
    Backticks(usize),
    Tildes(usize),
}

// TODO: display removing indentation.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RawContent<'a> {
    // Up to 3 spaces.
    indent: usize,
    segments: Vec<Segment<'a>>,
}

impl<'a> RawContent<'a> {
    fn new(indent: usize) -> Self {
        if indent > 3 {
            panic!("unexpected indent greater than 3: {}", indent);
        }

        Self {
            indent,
            segments: Vec::new(),
        }
    }

    fn push(&mut self, segment: Segment<'a>) {
        self.segments.push(segment);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FencedCodeNode<'a> {
    pub segments: Vec<Segment<'a>>,
    /// Only set if there was a least one non whitespace character provided.
    info_string: Option<Segment<'a>>,
    fence: Fence,
    raw_content: RawContent<'a>,
    // This is the segment that closes the node. You can have a valid
    // fenced clode block that doesn't have a closing segment in one case:
    // when we reach the EOF. Otherwise, this will always be set.
    closing_segment: Option<Segment<'a>>,
}

impl<'a> FencedCodeNode<'a> {
    fn new(
        segment: Segment<'a>,
        indent: usize,
        info_string: Option<Segment<'a>>,
        fence: Fence,
    ) -> Self {
        Self {
            segments: vec![segment],
            info_string,
            fence,
            raw_content: RawContent::new(indent),
            closing_segment: None,
        }
    }

    fn is_exit_segment(&self, segment: Segment<'a>) -> bool {
        match EXIT_REGEX.captures(segment.text()) {
            Some(captures) => match self.fence {
                Fence::Backticks(count) => match captures.get(1) {
                    Some(capture) => capture.as_str().len() >= count,
                    None => false,
                },
                Fence::Tildes(count) => match captures.get(2) {
                    Some(capture) => capture.as_str().len() >= count,
                    None => false,
                },
            },
            None => false,
        }
    }

    fn closed(&self) -> bool {
        self.closing_segment.is_some()
    }
}

/// Info string cannot contain backtick characters.
static ENTRY_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^([ ]{0,3})(?:(?:(`{3,})\s*([^`]*)\s+)|(?:(~{3,})\s*(.*)\s+))$").unwrap()
});
/// Note that matching this regex is insufficient to determine if the segment is an exit segment.
/// We still need to check the type of the fence (backtick vs. tilde) and the repetition count.
static EXIT_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[ ]{0,3}(?:(`{3,})|(~{3,}))\s+$").unwrap());

impl<'a> TryFrom<Segment<'a>> for FencedCodeNode<'a> {
    type Error = Error;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        match ENTRY_REGEX.captures(segment.text()) {
            Some(captures) => {
                // Indent is first group.
                let indent = captures.get(1).unwrap().as_str();
                // If second group has a match, then we have backticks. The info string is in the 3rd group.
                if let Some(capture) = captures.get(2) {
                    let backticks = capture.as_str();
                    let fence = Fence::Backticks(backticks.len());
                    let info_string = captures
                        .get(3)
                        .map(|m| {
                            Segment::new(segment.start().walk(indent).walk(backticks), m.as_str())
                        })
                        .filter(|content| !content.text().trim().is_empty());
                    return Ok(Self::new(segment, indent.len(), info_string, fence));
                }

                // If the fourth group has a match, then we have tildes. The info string is in the 5th group.
                let tildes = captures.get(4).unwrap().as_str();
                let fence = Fence::Tildes(tildes.len());
                let info_string = captures
                    .get(5)
                    .map(|m| Segment::new(segment.start().walk(indent).walk(tildes), m.as_str()))
                    .filter(|content| !content.text().trim().is_empty());

                Ok(Self::new(segment, indent.len(), info_string, fence))
            }
            None => return Err(Error::invalid_segment()),
        }
    }
}

impl<'a> Consume<'a> for FencedCodeNode<'a> {
    fn consume(&mut self, segment: Segment<'a>) -> NodeState<'a> {
        // If we found the closing line earlier, then we're done with this block. We simply
        // return the next matching one.
        if self.closed() {
            return NodeState::InterruptedBy(BlockNode::from(segment));
        }
        if self.is_exit_segment(segment) {
            self.closing_segment = Some(segment);
            // TODO: we could add an enum value to express that we are done already. But it doesn't
            // improve the code much, since this is pretty exceptional.
            return NodeState::InProgress;
        }

        self.segments.push(segment);
        self.raw_content.push(segment);
        NodeState::InProgress
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
                FencedCodeNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_blank_line() {
            let segment = Segment::first("    \n");
            assert_eq!(
                FencedCodeNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        mod backticks {
            use super::*;

            #[test]
            fn should_reject_backticks_in_info_string() {
                let segment = Segment::first("```rust`\n");
                assert_eq!(
                    FencedCodeNode::try_from(segment),
                    Err(Error::invalid_segment())
                );
            }

            #[test]
            fn should_reject_4_whitespace_indent() {
                let segment = Segment::first("    ```\n");
                assert_eq!(
                    FencedCodeNode::try_from(segment),
                    Err(Error::invalid_segment())
                );
            }

            #[test]
            fn should_reject_tab_indent() {
                let segment = Segment::first("\t```rust\n");
                assert_eq!(
                    FencedCodeNode::try_from(segment),
                    Err(Error::invalid_segment())
                );
            }
        }

        mod tildes {
            use super::*;

            #[test]
            fn should_reject_4_whitespace_indent() {
                let segment = Segment::first("    ~~~\n");
                assert_eq!(
                    FencedCodeNode::try_from(segment),
                    Err(Error::invalid_segment())
                );
            }

            #[test]
            fn should_reject_tab_indent() {
                let segment = Segment::first("\t~~~rust\n");
                assert_eq!(
                    FencedCodeNode::try_from(segment),
                    Err(Error::invalid_segment())
                );
            }
        }
    }

    mod consume {
        use super::*;

        mod backticks {
            use super::*;

            #[test]
            fn should_work_with_minimal_block() {
                let first_segment = Segment::first("```\n");
                let second_segment = Segment::new(first_segment.end(), "```\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                assert_eq!(node.segments, vec![first_segment]);
                assert_eq!(node.closing_segment, Some(second_segment));
                assert_eq!(node.info_string, None);
                assert_eq!(node.fence, Fence::Backticks(3));
                assert_eq!(node.raw_content.indent, 0);
                assert_eq!(node.raw_content.segments.len(), 0);
            }

            #[test]
            fn should_work_with_info_string_and_content() {
                let first_segment = Segment::first("```rust\n");
                let second_segment = Segment::new(first_segment.end(), "let x = 4;\n");
                let third_segment = Segment::new(second_segment.end(), "```\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                assert_eq!(node.consume(third_segment), NodeState::InProgress);
                assert_eq!(node.segments, vec![first_segment, second_segment]);
                assert_eq!(node.closing_segment, Some(third_segment));
                assert_eq!(
                    node.info_string,
                    Some(Segment::new(location::Position::new(1, 4, 3), "rust"))
                );
                assert_eq!(node.fence, Fence::Backticks(3));
                assert_eq!(node.raw_content.indent, 0);
                assert_eq!(node.raw_content.segments, vec![second_segment]);
            }

            #[test]
            fn should_tolerate_3_whitespace_indent() {
                let first_segment = Segment::first("   ```rust\n");
                let second_segment = Segment::new(first_segment.end(), "let x = 4;\n");
                let third_segment = Segment::new(second_segment.end(), "```\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                assert_eq!(node.consume(third_segment), NodeState::InProgress);
                assert_eq!(node.segments, vec![first_segment, second_segment]);
                assert_eq!(node.closing_segment, Some(third_segment));
                assert_eq!(
                    node.info_string,
                    Some(Segment::new(location::Position::new(1, 7, 6), "rust"))
                );
                assert_eq!(node.fence, Fence::Backticks(3));
                assert_eq!(node.raw_content.indent, 3);
                assert_eq!(node.raw_content.segments, vec![second_segment]);
            }

            #[test]
            fn should_work_with_more_than_3_backticks() {
                let first_segment = Segment::first("`````rust\n");
                let second_segment = Segment::new(first_segment.end(), "let x = 4;\n");
                let third_segment = Segment::new(second_segment.end(), "`````\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                assert_eq!(node.consume(third_segment), NodeState::InProgress);
                assert_eq!(node.segments, vec![first_segment, second_segment]);
                assert_eq!(node.closing_segment, Some(third_segment));
                assert_eq!(
                    node.info_string,
                    Some(Segment::new(location::Position::new(1, 6, 5), "rust"))
                );
                assert_eq!(node.fence, Fence::Backticks(5));
                assert_eq!(node.raw_content.indent, 0);
                assert_eq!(node.raw_content.segments, vec![second_segment]);
            }

            #[test]
            fn should_treat_shorter_fence_as_content() {
                let first_segment = Segment::first("`````rust\n");
                let second_segment = Segment::new(first_segment.end(), "let x = 4;\n");
                let third_segment = Segment::new(second_segment.end(), "````\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                assert_eq!(node.consume(third_segment), NodeState::InProgress);
                assert_eq!(
                    node.segments,
                    vec![first_segment, second_segment, third_segment]
                );
                assert_eq!(node.closing_segment, None);
                assert_eq!(
                    node.info_string,
                    Some(Segment::new(location::Position::new(1, 6, 5), "rust"))
                );
                assert_eq!(node.fence, Fence::Backticks(5));
                assert_eq!(node.raw_content.indent, 0);
                // Notice how the third segment is included in the raw content. This means the node is
                // not yet closed.
                assert_eq!(
                    node.raw_content.segments,
                    vec![second_segment, third_segment]
                );
            }
        }

        mod tildes {
            use super::*;

            #[test]
            fn should_work_with_minimal_block() {
                let first_segment = Segment::first("~~~\n");
                let second_segment = Segment::new(first_segment.end(), "~~~\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                assert_eq!(node.segments, vec![first_segment]);
                assert_eq!(node.closing_segment, Some(second_segment));
                assert_eq!(node.info_string, None);
                assert_eq!(node.fence, Fence::Tildes(3));
                assert_eq!(node.raw_content.indent, 0);
                assert_eq!(node.raw_content.segments.len(), 0);
            }

            #[test]
            fn should_work_with_info_string_and_content() {
                let first_segment = Segment::first("~~~rust\n");
                let second_segment = Segment::new(first_segment.end(), "let x = 4;\n");
                let third_segment = Segment::new(second_segment.end(), "~~~\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                assert_eq!(node.consume(third_segment), NodeState::InProgress);
                assert_eq!(node.segments, vec![first_segment, second_segment]);
                assert_eq!(node.closing_segment, Some(third_segment));
                assert_eq!(
                    node.info_string,
                    Some(Segment::new(location::Position::new(1, 4, 3), "rust"))
                );
                assert_eq!(node.fence, Fence::Tildes(3));
                assert_eq!(node.raw_content.indent, 0);
                assert_eq!(node.raw_content.segments, vec![second_segment]);
            }

            #[test]
            fn should_tolerate_3_whitespace_indent() {
                let first_segment = Segment::first("   ~~~rust\n");
                let second_segment = Segment::new(first_segment.end(), "let x = 4;\n");
                let third_segment = Segment::new(second_segment.end(), "~~~\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                assert_eq!(node.consume(third_segment), NodeState::InProgress);
                assert_eq!(node.segments, vec![first_segment, second_segment]);
                assert_eq!(node.closing_segment, Some(third_segment));
                assert_eq!(
                    node.info_string,
                    Some(Segment::new(location::Position::new(1, 7, 6), "rust"))
                );
                assert_eq!(node.fence, Fence::Tildes(3));
                assert_eq!(node.raw_content.indent, 3);
                assert_eq!(node.raw_content.segments, vec![second_segment]);
            }

            #[test]
            fn should_work_with_more_than_3_tildes() {
                let first_segment = Segment::first("~~~~~rust\n");
                let second_segment = Segment::new(first_segment.end(), "let x = 4;\n");
                let third_segment = Segment::new(second_segment.end(), "~~~~~\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                assert_eq!(node.consume(third_segment), NodeState::InProgress);
                assert_eq!(node.segments, vec![first_segment, second_segment]);
                assert_eq!(node.closing_segment, Some(third_segment));
                assert_eq!(
                    node.info_string,
                    Some(Segment::new(location::Position::new(1, 6, 5), "rust"))
                );
                assert_eq!(node.fence, Fence::Tildes(5));
                assert_eq!(node.raw_content.indent, 0);
                assert_eq!(node.raw_content.segments, vec![second_segment]);
            }

            #[test]
            fn should_treat_shorter_fence_as_content() {
                let first_segment = Segment::first("~~~~~rust\n");
                let second_segment = Segment::new(first_segment.end(), "let x = 4;\n");
                let third_segment = Segment::new(second_segment.end(), "~~~~\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                assert_eq!(node.consume(third_segment), NodeState::InProgress);
                assert_eq!(
                    node.segments,
                    vec![first_segment, second_segment, third_segment]
                );
                assert_eq!(node.closing_segment, None);
                assert_eq!(
                    node.info_string,
                    Some(Segment::new(location::Position::new(1, 6, 5), "rust"))
                );
                assert_eq!(node.fence, Fence::Tildes(5));
                assert_eq!(node.raw_content.indent, 0);
                // Notice how the third segment is included in the raw content. This means the node is
                // not yet closed.
                assert_eq!(
                    node.raw_content.segments,
                    vec![second_segment, third_segment]
                );
            }
        }

        // Once a code block is closed, any subsequent line should result in an interrupt.
        mod interruptions {
            use crate::block::tree::{
                blank_line::BlankLineNode, AtxHeadingNode, IndentedCodeNode, ParagraphNode,
                ThematicBreakNode,
            };

            use super::*;

            #[test]
            fn should_be_interrupted_by_atx_heading() {
                let first_segment = Segment::first("```\n");
                let second_segment = Segment::new(first_segment.end(), "```\n");
                let third_segment = Segment::new(second_segment.end(), "# Heading\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                let interrupting = node.consume(third_segment).unwrap_interrupting_node();
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_atx_heading(),
                    AtxHeadingNode::try_from(third_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_blank_line() {
                let first_segment = Segment::first("```\n");
                let second_segment = Segment::new(first_segment.end(), "```\n");
                let third_segment = Segment::new(second_segment.end(), "\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                let interrupting = node.consume(third_segment).unwrap_interrupting_node();
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_blank_line(),
                    BlankLineNode::try_from(third_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_fenced_code() {
                let first_segment = Segment::first("```\n");
                let second_segment = Segment::new(first_segment.end(), "```\n");
                let third_segment = Segment::new(second_segment.end(), "```\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                let interrupting = node.consume(third_segment).unwrap_interrupting_node();
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_fenced_code(),
                    FencedCodeNode::try_from(third_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_indented_code() {
                let first_segment = Segment::first("```\n");
                let second_segment = Segment::new(first_segment.end(), "```\n");
                let third_segment = Segment::new(second_segment.end(), "    let x = 4;\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                let interrupting = node.consume(third_segment).unwrap_interrupting_node();
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_indented_code(),
                    IndentedCodeNode::try_from(third_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_paragraph() {
                let first_segment = Segment::first("```\n");
                let second_segment = Segment::new(first_segment.end(), "```\n");
                let third_segment = Segment::new(second_segment.end(), "  Hello, world!\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                let interrupting = node.consume(third_segment).unwrap_interrupting_node();
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_paragraph(),
                    ParagraphNode::try_from(third_segment).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_thematic_break() {
                let first_segment = Segment::first("```\n");
                let second_segment = Segment::new(first_segment.end(), "```\n");
                let third_segment = Segment::new(second_segment.end(), "---\n");
                let mut node = FencedCodeNode::try_from(first_segment).unwrap();
                assert_eq!(node.consume(second_segment), NodeState::InProgress);
                let interrupting = node.consume(third_segment).unwrap_interrupting_node();
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_thematic_break(),
                    ThematicBreakNode::try_from(third_segment).unwrap()
                );
            }
        }
    }
}
