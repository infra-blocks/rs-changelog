use std::{collections::VecDeque, iter::once, sync::LazyLock};

use crate::Segment;

use super::{
    blank_line::BlankLineNode,
    error::Error,
    ingestion::{Consume, NodeState},
    parser::{ParseResult, Parser, ParserState, PartialParseResult},
    BlockNode, FencedCodeNode, LeafNode, LinkReferenceDefinitionNode,
    LinkReferenceDefinitionParser, SetextHeadingNode, ThematicBreakNode,
};

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

// TODO: on display, trim the last segment of trailing whitespace.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphNode<'a> {
    pub segments: Vec<Segment<'a>>,
    raw_content: RawContent<'a>,
}

impl<'a> ParagraphNode<'a> {
    fn raw_dog<T: Into<RawContent<'a>>>(segments: Vec<Segment<'a>>, raw_content: T) -> Self {
        Self {
            segments,
            raw_content: raw_content.into(),
        }
    }

    fn into_link_reference_definitions(&mut self) -> (Vec<BlockNode<'a>>, Option<Self>) {
        // Try to make as many link rerefence definitions as possible.
        // Because we know that the first link reference definition has to start at the
        // beginning of the paragraph, we can fail as soon as one segment cannot be parsed.
        // This is because a link reference definition requires a blank line after a
        // paragraph. This also means that a partial parse should also stop the iteration.
        let mut link_reference_definitions = Vec::new();

        let mut unparsed_segments = Vec::from_iter(self.segments.iter().copied());
        let mut iter = unparsed_segments.iter().copied();
        let rejected_segments = loop {
            let Some(segment) = iter.next() else {
                // Finish once we have exhausted the segments. If we make it here,
                // either all segments were parsed or there were never any segments!
                break None;
            };
            match LinkReferenceDefinitionParser::start_with(segment) {
                ParserState::Ready(parser) => match parser.consume_all(&mut iter) {
                    // If the parser was able to consume all segments and produce a value, then we store the value
                    // and keep going.
                    PartialParseResult::Parsed(node) => {
                        link_reference_definitions
                            .push(BlockNode::Leaf(LeafNode::LinkReferenceDefinition(node)));
                    }

                    // If the parser could produce a value, but couldn't consume all the segments, we
                    // need to store the produced value and put back the last segment. It could indeed start
                    // another link reference definition.
                    PartialParseResult::Partial(node, rejected_segments) => {
                        link_reference_definitions
                            .push(BlockNode::Leaf(LeafNode::LinkReferenceDefinition(node)));
                        // TODO: optimize this. We should find a way to make this a constant time operation.
                        // the difficulty resides in the typing hehe. So, instead of having a collection and
                        // an iter, maybe we should produce some kind of mutable iterator that can be prepended to.
                        unparsed_segments =
                            Vec::from_iter(rejected_segments.into_iter().chain(iter));
                        iter = unparsed_segments.iter().copied();
                    }

                    // If the parser could consume the first segment but, in the end, could produce a value,
                    // we stop consuming.
                    PartialParseResult::Rejected(rejected_segments) => {
                        break Some(rejected_segments)
                    }
                },
                ParserState::Finalized(result) => match result {
                    // If the parser could make a link reference definition with one segment, then we
                    // store the value and keep going.
                    ParseResult::Parsed(node) => {
                        link_reference_definitions
                            .push(BlockNode::Leaf(LeafNode::LinkReferenceDefinition(node)));
                    }
                    // If the parser has finalized and rejected the segment, then we are done.
                    ParseResult::Rejected(rejected_segments) => break Some(rejected_segments),
                },
            }
        };

        match rejected_segments {
            // The unparsed segments signal that the paragraph was not entirely consumed. Maybe not even a little bit!
            Some(rejected_segments) => {
                let new_segments: Vec<_> = rejected_segments.into_iter().chain(iter).collect();
                // We keep the last part of the raw content of the same length as the new segments.
                let new_raw_content = self.raw_content.segments
                    [self.raw_content.segments.len() - new_segments.len()..]
                    .to_vec();
                (
                    link_reference_definitions,
                    Some(Self::raw_dog(new_segments, new_raw_content)),
                )
            }
            // If we have no unparsed segments, then we morphed the whole paragraph into link reference definitions.
            None => (link_reference_definitions, None),
        }
    }
}

// Paragraphs require at least one non whitespace character.
// The trailing whitespaces are not part of the "raw content" of the paragraph.
// The raw content is the part that is going to be parsed as inline.
static ENTRY_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^([ ]{0,3})(\S+.*?\s+)$").unwrap());
static CONTINUATION_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^([ ]*)(\S+.*?\s+)$").unwrap());

impl<'a> TryFrom<Segment<'a>> for ParagraphNode<'a> {
    type Error = Error;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        match ENTRY_REGEX.captures(segment.text()) {
            Some(captures) => {
                let segments = [segment].into();
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

impl<'a> Consume<'a> for ParagraphNode<'a> {
    fn consume(&mut self, segment: Segment<'a>) -> NodeState<'a> {
        // Setext headings have precedence over thematic breaks. There is ambiguity, for example, if the segment is "---\n".
        // In that case, the setext heading must win. Setext headings also take all the previous segments, not just the last one.
        if let Ok(node) = SetextHeadingNode::try_from((&*self, segment)) {
            return NodeState::ReplacedBy([BlockNode::Leaf(LeafNode::SetextHeading(node))].into());
        }

        let interrupting = if let Ok(node) = ThematicBreakNode::try_from(segment) {
            Some(BlockNode::Leaf(LeafNode::ThematicBreak(node)))
        } else if let Ok(node) = BlankLineNode::try_from(segment) {
            Some(BlockNode::Leaf(LeafNode::BlankLine(node)))
        } else if let Ok(node) = FencedCodeNode::try_from(segment) {
            Some(BlockNode::Leaf(LeafNode::FencedCode(node)))
        } else {
            None
        };

        if let Some(node) = interrupting {
            match self.into_link_reference_definitions() {
                // This branch matches the case where the paragraph is consumed entirely and replaced with at least one link
                // reference definition.
                (link_reference_definitions, None) => {
                    let mut replacement_nodes = link_reference_definitions;
                    replacement_nodes.push(node);
                    return NodeState::ReplacedBy(replacement_nodes.into());
                }
                // This branch matches the case where the paragraph isn't consumed entirely. In this case,
                // the link reference definitions come first, followed by the remaining paragraph, followed
                // by the interrupting node. To stay consistent with the API, however, we will return
                // an interruption even if no link reference definitions were made. In the future, we
                // plan on streamlining the API so this case here is not required.
                (link_reference_definitions, Some(paragraph)) => {
                    if link_reference_definitions.is_empty() {
                        return NodeState::InterruptedBy(node);
                    }

                    let mut replacement_nodes = link_reference_definitions;
                    replacement_nodes.push(BlockNode::Leaf(LeafNode::Paragraph(paragraph)));
                    replacement_nodes.push(node);
                    return NodeState::ReplacedBy(replacement_nodes.into());
                }
            }
        }

        match CONTINUATION_REGEX.captures(segment.text()) {
            Some(captures) => {
                self.segments.push(segment);
                let raw_content_start = segment.start().walk(captures.get(1).unwrap().as_str());
                let raw_content_text = captures.get(2).unwrap().as_str();
                self.raw_content
                    .push(Segment::new(raw_content_start, raw_content_text));
                NodeState::InProgress
            }
            // The paragraph can be interrupted by almost anything.
            None => panic!("missing interrupting node"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from {
        use super::*;

        #[test]
        fn should_reject_first_segment_when_empty() {
            let segment = Segment::first("");
            assert_eq!(
                ParagraphNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_first_segment_with_four_whitespaces() {
            let segment = Segment::first("    aaa\n");
            assert_eq!(
                ParagraphNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_reject_first_segment_starting_with_tab() {
            let segment = Segment::first("\taaa\n");
            assert_eq!(
                ParagraphNode::try_from(segment),
                Err(Error::invalid_segment())
            );
        }

        #[test]
        fn should_work_with_a_single_char() {
            let segment = Segment::first("a\n");
            let node = ParagraphNode::try_from(segment).unwrap();
            assert_eq!(node.segments, vec![segment]);
            assert_eq!(node.raw_content, vec![Segment::first("a\n")].into());
        }

        #[test]
        fn should_work_with_up_to_three_whitespaces() {
            let segment = Segment::first("   aaa\n");
            let node = ParagraphNode::try_from(segment).unwrap();
            assert_eq!(node.segments, vec![segment]);
            assert_eq!(
                node.raw_content,
                vec![Segment::new(location::Position::new(1, 4, 3), "aaa\n")].into()
            );
        }
    }

    mod consume {
        use super::*;
        use crate::StrExt;

        #[test]
        fn should_work_with_multiple_valid_lines() {
            // The second line tolerates any amount of indentation.
            // The trailing whitespaces are kept as part of the raw content. They are compressed on display.
            let segments = "   aaa\n        bbb\nccc   \n"
                .line_segments()
                .collect::<Vec<_>>();
            let mut node = ParagraphNode::try_from(segments[0]).unwrap();
            assert!(matches!(node.consume(segments[1]), NodeState::InProgress));
            assert!(matches!(node.consume(segments[2]), NodeState::InProgress));
            assert_eq!(node.segments, segments);
            assert_eq!(
                node.raw_content,
                vec![
                    Segment::new(location::Position::new(1, 4, 3), "aaa\n"),
                    Segment::new(location::Position::new(2, 9, 15), "bbb\n"),
                    Segment::new(location::Position::new(3, 1, 19), "ccc   \n"),
                ]
                .into()
            );
        }

        mod interruptions {
            use super::*;

            #[test]
            fn should_be_interrupted_by_thematic_break() {
                let segments = "aaa\n***\n".line_segments().collect::<Vec<_>>();
                let mut node = ParagraphNode::try_from(segments[0]).unwrap();
                let interrupting = node.consume(segments[1]).unwrap_interrupting_node();
                assert_eq!(node.segments, vec![segments[0]]);
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_thematic_break().segment(),
                    &segments[1]
                );
            }

            #[test]
            fn should_be_interrupted_by_blank_line() {
                let segments = "aaa\n\n".line_segments().collect::<Vec<_>>();
                let mut node = ParagraphNode::try_from(segments[0]).unwrap();
                let interrupting = node.consume(segments[1]).unwrap_interrupting_node();
                assert_eq!(node.segments, vec![segments[0]]);
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_blank_line().segment(),
                    &segments[1]
                );
            }

            #[test]
            fn should_be_interrupted_by_fenced_code() {
                let segments = "aaa\n```\n".line_segments().collect::<Vec<_>>();
                let mut node = ParagraphNode::try_from(segments[0]).unwrap();
                let interrupting = node.consume(segments[1]).unwrap_interrupting_node();
                assert_eq!(node.segments, vec![segments[0]]);
                assert_eq!(
                    interrupting.unwrap_leaf().unwrap_fenced_code().segments,
                    vec![segments[1]]
                );
            }
        }

        mod replacements {
            use super::*;

            use crate::unwrap_singleton::UnwrapSingleton;

            #[test]
            fn should_be_replaced_by_setext_heading_for_valid_underline() {
                let first_segment = Segment::first("aaa\n");
                let second_segment = Segment::new(first_segment.end(), "===\n");

                let mut node = ParagraphNode::try_from(Segment::first("aaa\n")).unwrap();
                let replacing = node
                    .consume(second_segment)
                    .unwrap_replacement_nodes()
                    .unwrap_singleton()
                    .unwrap_leaf()
                    .unwrap_setext_heading();
                assert_eq!(node.segments, vec![first_segment]);
                assert_eq!(replacing.level(), 1);
                assert_eq!(replacing.segments(), vec![first_segment, second_segment]);
            }

            #[test]
            fn should_be_replaced_by_setext_heading_in_precedence_over_thematic_break_on_hyphen_underline(
            ) {
                let first_segment = Segment::first("aaa\n");
                let second_segment = Segment::new(first_segment.end(), "---\n");

                let mut node = ParagraphNode::try_from(Segment::first("aaa\n")).unwrap();
                let replacing = node
                    .consume(second_segment)
                    .unwrap_replacement_nodes()
                    .unwrap_singleton()
                    .unwrap_leaf()
                    .unwrap_setext_heading();
                assert_eq!(node.segments, vec![first_segment]);
                assert_eq!(replacing.level(), 2);
                assert_eq!(replacing.segments(), vec![first_segment, second_segment]);
            }

            // TODO: test that a replacement with a setext heading does not trigger link reference definition conversions.
            // I need to double check that, but I think if the setext heading text is a link ref def, it is not considered
            // as such and simply kept literally.
        }

        // These test the behavior of conversion into link reference definitions.
        // At the time of this writing, only interruptions should trigger the conversions. Eventually,
        // the interface will also be extended to support a "finalize" method that should also trigger that conversion.
        // For the sake of simplicity, we will always use the same trigger for now, which will be a blank line.
        // The tests here are also not exhaustive in terms of link reference definitions parsing. Those are handled
        // in that module. Here, we look more specifically at the behavior around the conversion from paragraph
        // to link ref defs.
        mod link_reference_definitions {
            use crate::block::tree::link::{LinkDestination, LinkLabel};

            use super::*;

            #[test]
            fn should_work_with_a_single_segment_link_reference_definition() {
                let segments = "[foo]: bar\n\n".line_segments().collect::<Vec<_>>();

                let mut node = ParagraphNode::try_from(segments[0]).unwrap();
                let mut nodes = node.consume(segments[1]).unwrap_replacement_nodes();
                // The first node is expected to be a link reference definition, and the second node is expected to be a blank line.
                let definition = nodes
                    .pop_front()
                    .unwrap()
                    .unwrap_leaf()
                    .unwrap_link_reference_definition();
                assert_eq!(
                    definition.label,
                    LinkLabel::try_from(Segment::first("[foo]")).unwrap()
                );
                assert_eq!(
                    definition.destination,
                    LinkDestination::try_from(Segment::new(
                        location::Position::new(1, 8, 7),
                        "bar"
                    ))
                    .unwrap()
                );
            }

            #[test]
            fn should_work_with_sequential_link_reference_definitions() {
                let segments = "[foo]: bar\n[toto]: tata\n\n"
                    .line_segments()
                    .collect::<Vec<_>>();

                let mut node = ParagraphNode::try_from(segments[0]).unwrap();
                assert_eq!(node.consume(segments[1]), NodeState::InProgress);
                let mut nodes = node.consume(segments[2]).unwrap_replacement_nodes();
                let first_definition = nodes
                    .pop_front()
                    .unwrap()
                    .unwrap_leaf()
                    .unwrap_link_reference_definition();
                let second_definition = nodes
                    .pop_front()
                    .unwrap()
                    .unwrap_leaf()
                    .unwrap_link_reference_definition();
                assert_eq!(
                    first_definition.label,
                    LinkLabel::try_from(Segment::first("[foo]")).unwrap()
                );
                assert_eq!(
                    first_definition.destination,
                    LinkDestination::try_from(Segment::new(
                        location::Position::new(1, 8, 7),
                        "bar"
                    ))
                    .unwrap()
                );
                assert_eq!(
                    second_definition.label,
                    LinkLabel::try_from(Segment::new(location::Position::new(2, 1, 11), "[toto]"))
                        .unwrap()
                );
                assert_eq!(
                    second_definition.destination,
                    LinkDestination::try_from(Segment::new(
                        location::Position::new(2, 9, 19),
                        "tata"
                    ))
                    .unwrap()
                );
            }
        }
    }
}
