use std::iter::once;

use crate::{
    internal::parse::{
        parser::{Finalize, Ingest, IngestResult},
        segment::{ParagraphContinuationSegment, ParagraphOpeningSegment, ParagraphSegments},
    },
    IntoSegments, Segment,
};

use super::{ParagraphResult, SetextHeading};

pub enum ParagraphParser<'a> {
    Idle,
    Opened(ParagraphSegments<'a>),
}

impl<'a> Default for ParagraphParser<'a> {
    fn default() -> Self {
        Self::Idle
    }
}

impl<'a> ParagraphParser<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

pub enum ParagraphParserResult<'a> {
    SetextHeading(SetextHeading<'a>),
    // TODO: rename inner?
    Paragraph(ParagraphResult<'a>),
}

impl<'a> From<SetextHeading<'a>> for ParagraphParserResult<'a> {
    fn from(value: SetextHeading<'a>) -> Self {
        Self::SetextHeading(value)
    }
}

impl<'a> From<ParagraphResult<'a>> for ParagraphParserResult<'a> {
    fn from(value: ParagraphResult<'a>) -> Self {
        Self::Paragraph(value)
    }
}

// At any point after the opening segment, a setext heading can be made.
impl<'a> Ingest for ParagraphParser<'a> {
    type Input = Segment<'a>;
    type Ready = Self;
    type Success = ParagraphParserResult<'a>;
    type Failure = Vec<Segment<'a>>;

    fn ingest(
        self,
        segment: Self::Input,
    ) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        match self {
            ParagraphParser::Idle => match ParagraphOpeningSegment::try_from(segment) {
                Ok(opening_segment) => {
                    IngestResult::Ready(ParagraphParser::Opened(opening_segment.into()))
                }
                Err(segment) => IngestResult::Failure(vec![segment]),
            },
            ParagraphParser::Opened(mut paragraph_segments) => {
                match ParagraphContinuationSegment::try_from(segment) {
                    // TODO: handle setext heading and other interrupts.
                    Ok(continuation_segment) => {
                        paragraph_segments.continuations.push(continuation_segment);
                        IngestResult::Ready(ParagraphParser::Opened(paragraph_segments))
                    }
                    Err(segment) => IngestResult::Failure(
                        paragraph_segments
                            .into_segments()
                            .chain(once(segment))
                            .collect(),
                    ),
                }
            }
        }
    }
}

impl<'a> Finalize for ParagraphParser<'a> {
    type Result = Option<ParagraphResult<'a>>;

    fn finalize(self) -> Self::Result {
        match self {
            ParagraphParser::Idle => None,
            ParagraphParser::Opened(paragraph_segments) => Some(paragraph_segments.into()),
        }
    }
}

// Furthermore, paragraphs can be interrupted by several other types of blocks.

/* impl<'a> Paragraph<'a> {
    /// Morphs a paragraph instance into potentially several link reference definitions, and maybe
    /// a paragraph with the remaining segments.
    ///
    /// The return could produce no link reference definitions coupled with the same paragraph as
    /// provided. It could return the just link reference definitions and no paragraph. Finally,
    /// it can return a mix of both, where the paragraph is made up of the segments following the
    /// last valid link reference definition.
    fn into_link_reference_definitions(self) -> (Vec<Block<'a>>, Option<Self>) {
        // We will try to make as many link rerefence definitions as possible.
        // Because we know that the first link reference definition has to start at the
        // beginning of the paragraph, we can fail as soon as one segment cannot be parsed.
        // This is because a link reference definition requires a blank line after a
        // paragraph. This also means that a partial parse should also stop the iteration.
        let mut link_reference_definitions = Vec::new();

        // TODO: make a special kind of iterator here. One that is backed by a vecdeque and allows
        // to put back items.
        let mut unparsed_segments = Vec::from_iter(self.segments.iter().copied());
        let mut iter = unparsed_segments.iter().copied();

        // When we stop the iteration, we might stop because the link reference definition parser rejected
        // some segments. In that case, those segments will form the beginning of the remaining paragraph.
        let maybe_rejected_segments = loop {
            let Some(segment) = iter.next() else {
                // Finish once we have exhausted the segments. If we make it here,
                // either all segments were parsed or there were never any segments!
                break None;
            };
            match LinkReferenceDefinitionParser::start_with(segment) {
                ParserState::Ready(parser) => match parser.consume_all_and_finalize(&mut iter) {
                    // If the parser was able to consume all segments and produce a value, then we store the value
                    // and keep going.
                    PartialParseResult::Parsed(node) => {
                        link_reference_definitions
                            .push(Block::Leaf(Leaf::LinkReferenceDefinition(node)));
                    }

                    // If the parser could produce a value, but couldn't consume all the segments, we
                    // need to store the produced value and put back the last segment. It could indeed start
                    // another link reference definition.
                    PartialParseResult::Partial(node, rejected_segments) => {
                        link_reference_definitions
                            .push(Block::Leaf(Leaf::LinkReferenceDefinition(node)));
                        unparsed_segments =
                            Vec::from_iter(rejected_segments.into_iter().chain(iter));
                        iter = unparsed_segments.iter().copied();
                    }

                    // If the parser could consume the first segment but, in the end, could not produce a value,
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
                            .push(Block::Leaf(Leaf::LinkReferenceDefinition(node)));
                    }
                    // If the parser has finalized and rejected the segment, then we are done.
                    ParseResult::Rejected(rejected_segments) => break Some(rejected_segments),
                },
            }
        };

        // The remaining paragraph's segments are made of, first, the rejected segments, if any, followed by
        // the rest of the segments found in the iterator. If the result is empty, then no paragraph is returned.
        let mut remaining_paragraph_segments = match maybe_rejected_segments {
            Some(rejected_segments) => rejected_segments,
            None => Vec::new(),
        };
        remaining_paragraph_segments.extend(iter);

        (
            link_reference_definitions,
            if remaining_paragraph_segments.is_empty() {
                None
            } else {
                Some(Self::new(remaining_paragraph_segments))
            },
        )
    }
}

// Paragraphs require at least one non whitespace character.
static ENTRY_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[ ]{0,3}\S+.*?\s+$").unwrap());
static CONTINUATION_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[ ]*\S+.*?\s+$").unwrap());

/* /// The parser for [Paragraph]s.
///
/// It is a stateful parser and, unlike most parsers, can produce multiple blocks.
/// What makes this parser unique is that, upon finalization, it attempts to turn
/// the produced paragraph into as many link reference definitions as possible.
///
/// The remaining segments are kept as the produced paragraph (if any).
///
/// In addition to that, because paragraphs have the lowest precedence, it can be interrupted
/// by many other blocks when there is ambiguity. For example, a fenced code block can interrupt
/// a paragraph mid parsing.
///
/// Because of these, the caller cannot assume that the parser will inevitably produce a paragraph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphParser<'a> {
    segments: Vec<Segment<'a>>,
}

impl<'a> ParagraphParser<'a> {
    /// Constructs a new [ParagraphParser] with the given segments.
    fn new(segments: Vec<Segment<'a>>) -> Self {
        Self { segments }
    }

    /// Constructs a new [ParagraphParser] with the given opening segment.
    fn opening_with(segment: Segment<'a>) -> Self {
        Self::new(vec![segment])
    }

    /// Attempts to start a new [ParagraphParser] with the given segment.
    ///
    /// If the segment is a valid paragraph opening segment, then a new ready parser is returned.
    /// Otherwise, None is returned.
    ///
    /// Upon success, the parser state returned is always [ParserState::Ready], as there
    /// is no way to determine that the paragraph is complete with just one segment.
    pub fn start_with(segment: Segment<'a>) -> Option<ParserState<Self, BlockParseResult<'a>>> {
        if ENTRY_REGEX.is_match(segment.text()) {
            Some(Self::opening_with(segment).into())
        } else {
            None
        }
    }
}

impl<'a> Parser for ParagraphParser<'a> {
    type Input = Segment<'a>;
    type Output = BlockParseResult<'a>;

    /// Consumes and attempts to add the segment to the paragraph.
    ///
    /// This method first check for possible interruptions. If the additional segment can turn the paragraph into a
    /// [SetextHeading], then the parser is finalized and the setext heading is returned.
    ///
    /// If any of the following blocks can be bootstrapped with the segment, then those take precedence, finalize
    /// this parser, and return either the new block or the new parser:
    /// - Thematic break
    /// - Blank line
    /// - Fenced code block
    ///
    /// Upon finalization, the parser attempts to turn the paragraph into as many link reference definitions as possible.
    /// For example, if the paragraph had 3 segments when a blank line was found, then the parser tries to turn those
    /// 3 segments into a link reference definitions. If it could, those are returned first, then a paragraph made of the
    /// remaining segments and finally the interrupting block/parser.
    ///
    /// If no interrupting block can be started, then the segment is added to the paragraph.
    fn consume(self, segment: Self::Input) -> ParserState<Self, Self::Output> {
        // Setext headings have precedence over thematic breaks. There is ambiguity, for example, if the segment is "---\n".
        // In that case, the setext heading must win. Setext headings also take all the previous segments, not just the last one.
        let mut segments = match SetextHeading::try_from((Paragraph::new(self.segments), segment)) {
            // If we could fabricate a setext heading, we return that.
            Ok(block) => {
                return ParserState::Finalized(BlockParseResult::single_block(Block::Leaf(
                    Leaf::SetextHeading(block),
                )))
            }
            // We recuperate the paragrap segments and move forward.
            Err(paragraph) => paragraph.segments,
        };

        // Some blocks can interrupt a paragraph. We check those here. Unlike the Setext heading, those don't consume
        // the previous segments of the paragraph. They simply trigger finalization.
        let interrupting_state = if let Some(node) = ThematicBreakParser::parse(segment) {
            Some(ParserState::Finalized(node))
        } else if let Some(node) = BlankLineParser::parse(segment) {
            Some(ParserState::Finalized(node))
        } else if let Some(parser) = FencedCodeParser::start_with(segment) {
            Some(ParserState::Ready(BlockParser::Leaf(
                LeafParser::FencedCode(parser),
            )))
        } else {
            None
        };

        if let Some(state) = interrupting_state {
            match Paragraph::new(segments).into_link_reference_definitions() {
                // This branch matches the case where the paragraph is consumed entirely and replaced with at least one link
                // reference definition.
                (link_reference_definitions, None) => {
                    let mut blocks = link_reference_definitions;
                    match state {
                        ParserState::Finalized(interrupting_block) => {
                            blocks.push(interrupting_block);
                            return ParserState::Finalized(BlockParseResult::only_blocks(blocks));
                        }
                        ParserState::Ready(parser) => {
                            return ParserState::Finalized(BlockParseResult::blocks_with_parser(
                                blocks, parser,
                            ));
                        }
                    }
                }
                // This branch matches the case where the paragraph isn't consumed entirely. In this case,
                // the link reference definitions come first, followed by the remaining paragraph, followed
                // by the interrupting node.
                (link_reference_definitions, Some(paragraph)) => {
                    let mut blocks = link_reference_definitions;
                    blocks.push(Block::Leaf(Leaf::Paragraph(paragraph)));
                    match state {
                        ParserState::Finalized(interrupting_block) => {
                            blocks.push(interrupting_block);
                            return ParserState::Finalized(BlockParseResult::only_blocks(blocks));
                        }
                        ParserState::Ready(parser) => {
                            return ParserState::Finalized(BlockParseResult::blocks_with_parser(
                                blocks, parser,
                            ));
                        }
                    }
                }
            }
        }

        if CONTINUATION_REGEX.is_match(segment.text()) {
            segments.push(segment);
            return ParserState::Ready(Self::new(segments));
        }

        // The paragraph can be interrupted by almost anything, if we make it here, there is likely a bug in the code.
        unreachable!("missing interrupting node")
    }

    /// Attempts to turn the segments read so far into as many link reference definitions as possible, then
    /// appends the remaining segments as a paragraph.
    fn finalize(self) -> Self::Output {
        let (link_reference_definition, maybe_paragraph) =
            Paragraph::new(self.segments).into_link_reference_definitions();
        let mut blocks = link_reference_definition;
        if let Some(paragraph) = maybe_paragraph {
            blocks.push(Block::Leaf(Leaf::Paragraph(paragraph)));
        }

        BlockParseResult::only_blocks(blocks)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod paragraph {
        use super::*;

        // TODO: make dem tests, they should execute every logical branch!
        mod into_link_reference_definitions {
            use super::*;
        }
    }

    mod parser {
        use super::*;

        mod start_with {
            use super::*;

            macro_rules! test_reject {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        assert_eq!(ParagraphParser::start_with($segment), None);
                    }
                };
            }

            macro_rules! test_accept {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        let parser = ParagraphParser::start_with($segment)
                            .unwrap()
                            .unwrap_ready();
                        assert_eq!(parser.segments, vec![$segment]);
                    }
                };
            }

            test_reject!(should_reject_empty_segment, Segment::default());
            test_reject!(should_reject_blank_line, Segment::first("\n"));
            test_reject!(
                should_reject_four_leading_whitespaces,
                Segment::first("    a\n")
            );
            test_reject!(should_reject_leading_tag, Segment::first("\ta\n"));

            test_accept!(should_work_with_a_single_char, Segment::first("a\n"));
            test_accept!(
                should_work_with_three_leading_whitespaces,
                Segment::first("   a\n")
            );
        }

        mod consume {

            use crate::{
                block::{BlankLine, ThematicBreak},
                StrExt,
            };

            use super::*;

            impl<'a> ParagraphParser<'a> {
                // TODO: use iter.
                fn maybe_parse(
                    segments: Vec<Segment<'a>>,
                ) -> Option<ParserState<Self, BlockParseResult<'a>>> {
                    let parser = Self::start_with(segments[0])?.unwrap_ready();
                    let result = parser.consume_until_finalized(&mut segments.into_iter().skip(1));
                    Some(result)
                }
            }

            #[test]
            fn should_turn_into_a_setext_heading_when_possible() {
                let segments = "aaa\n---\n".line_segments().collect::<Vec<_>>();
                let result = ParagraphParser::maybe_parse(segments.clone())
                    .unwrap()
                    .unwrap_finalized();
                assert_eq!(
                    result.unwrap_block().unwrap_leaf().unwrap_setext_heading(),
                    SetextHeading::try_from((Paragraph::new(vec![segments[0]]), segments[1]))
                        .unwrap()
                )
            }

            #[test]
            fn should_be_interrupted_by_blank_line() {
                let segments = "aaa\n\n".line_segments().collect::<Vec<_>>();
                let result = ParagraphParser::maybe_parse(segments.clone())
                    .unwrap()
                    .unwrap_finalized();
                assert_eq!(
                    result.blocks,
                    Some(vec![
                        Block::Leaf(Leaf::Paragraph(Paragraph::new(vec![segments[0]]))),
                        Block::Leaf(Leaf::BlankLine(BlankLine::new(segments[1])))
                    ])
                )
            }

            #[test]
            fn should_be_interrupted_by_fenced_code() {
                let segments = "aaa\n```\n".line_segments().collect::<Vec<_>>();
                let result = ParagraphParser::maybe_parse(segments.clone())
                    .unwrap()
                    .unwrap_finalized();
                assert_eq!(
                    result.blocks,
                    Some(vec![Block::Leaf(Leaf::Paragraph(Paragraph::new(vec![
                        segments[0]
                    ]))),])
                );
                assert_eq!(
                    result
                        .next_parser
                        .unwrap()
                        .unwrap_leaf()
                        .unwrap_fenced_code(),
                    FencedCodeParser::start_with(segments[1]).unwrap()
                );
            }

            #[test]
            fn should_be_interrupted_by_thematic_break() {
                let segments = "aaa\n***\n".line_segments().collect::<Vec<_>>();
                let result = ParagraphParser::maybe_parse(segments.clone())
                    .unwrap()
                    .unwrap_finalized();
                assert_eq!(
                    result.blocks,
                    Some(vec![
                        Block::Leaf(Leaf::Paragraph(Paragraph::new(vec![segments[0]]))),
                        Block::Leaf(Leaf::ThematicBreak(ThematicBreak::new(segments[1])))
                    ])
                )
            }

            #[test]
            fn should_keep_the_new_segment_otherwise() {
                let segments = "aaa\nbbb\n".line_segments().collect::<Vec<_>>();
                let parser = ParagraphParser::maybe_parse(segments.clone())
                    .unwrap()
                    .unwrap_ready();
                assert_eq!(parser.segments, segments);
            }

            // We are not testing how link reference definitions are parsed here, rather how the paragraph handles
            // valid link reference definitions.
            /* mod link_reference_definitions {
                use crate::block::tree::LinkReferenceDefinition;

                use super::*;

                #[test]
                fn should_work_with_exactly_one_single_line_link_reference_definition() {
                    let segments = "[toto]: /tata\n***\n".line_segments().collect::<Vec<_>>();
                    let result = ParagraphParser::maybe_parse(segments.clone())
                        .unwrap()
                        .unwrap_finalized();
                    assert_eq!(
                        result.blocks,
                        Some(vec![
                            Block::Leaf(Leaf::LinkReferenceDefinition(
                                LinkReferenceDefinition::new()
                            )),
                            Block::Leaf(Leaf::ThematicBreak(ThematicBreak::new(segments[1])))
                        ])
                    )
                }

                #[test]
                fn should_work_with_one_link_reference_definition_and_remaining_text() {
                    let segments = "[toto]: /tata\n***\n".line_segments().collect::<Vec<_>>();
                    let result = ParagraphParser::maybe_parse(segments.clone())
                        .unwrap()
                        .unwrap_finalized();
                    assert_eq!(
                        result.blocks,
                        Some(vec![
                            Block::Leaf(Leaf::LinkReferenceDefinition(
                                LinkReferenceDefinition::new()
                            )),
                            Block::Leaf(Leaf::ThematicBreak(ThematicBreak::new(segments[1])))
                        ])
                    )
                }

                #[test]
                fn should_work_with_two_link_reference_definitions() {}
            } */
        }
    }
}
 */
 */
