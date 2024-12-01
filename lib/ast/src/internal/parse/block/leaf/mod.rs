mod atx_heading;
mod blank_line;
mod fenced_code;
mod indented_code;
mod paragraph;
mod thematic_break;

pub use atx_heading::*;
pub use blank_line::*;
pub use fenced_code::*;
pub use indented_code::*;
pub use thematic_break::*;

use crate::{
    internal::parse::{
        parser::{Finalize, Ingest, IngestResult},
        segment,
    },
    Segment,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Leaf<'a> {
    AtxHeading(AtxHeading<'a>),
    BlankLine(BlankLine<'a>),
    FencedCode(FencedCode<'a>),
    IndentedCode(IndentedCode<'a>),
    ThematicBreak(ThematicBreak<'a>),
}

impl<'a> From<AtxHeading<'a>> for Leaf<'a> {
    fn from(value: AtxHeading<'a>) -> Self {
        Self::AtxHeading(value)
    }
}

impl<'a> From<BlankLine<'a>> for Leaf<'a> {
    fn from(value: BlankLine<'a>) -> Self {
        Self::BlankLine(value)
    }
}

impl<'a> From<FencedCode<'a>> for Leaf<'a> {
    fn from(value: FencedCode<'a>) -> Self {
        Self::FencedCode(value)
    }
}

impl<'a> From<IndentedCode<'a>> for Leaf<'a> {
    fn from(value: IndentedCode<'a>) -> Self {
        Self::IndentedCode(value)
    }
}

impl<'a> From<ThematicBreak<'a>> for Leaf<'a> {
    fn from(value: ThematicBreak<'a>) -> Self {
        Self::ThematicBreak(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LeafParser<'a> {
    Idle,
    IndentedCode(IndentedCodeParser<'a>),
    FencedCode(FencedCodeParser<'a>),
}

impl<'a> Default for LeafParser<'a> {
    fn default() -> Self {
        Self::Idle
    }
}

impl<'a> LeafParser<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

pub(super) enum IngestSuccess<'a> {
    Single(Leaf<'a>),
    SingleWithInterrupt(Leaf<'a>, Segment<'a>),
    MultipleWithInterrupt(Vec<Leaf<'a>>, Segment<'a>),
}

impl<'a> From<indented_code::IngestSuccess<'a>> for IngestSuccess<'a> {
    fn from(success: indented_code::IngestSuccess<'a>) -> Self {
        let leaf = success.result.indented_code.into();
        let trailing_blank_lines = success.result.trailing_blank_lines;
        let rejected_segment = success.rejected_segment;
        if trailing_blank_lines.is_empty() {
            return IngestSuccess::SingleWithInterrupt(leaf, rejected_segment);
        } else {
            let mut leaves = vec![leaf];
            leaves.extend(
                trailing_blank_lines
                    .into_iter()
                    .map(BlankLine::from)
                    .map(Leaf::from),
            );
            return IngestSuccess::MultipleWithInterrupt(leaves, rejected_segment);
        }
    }
}

impl<'a> Ingest for LeafParser<'a> {
    type Input = Segment<'a>;
    type Ready = Self;
    type Success = IngestSuccess<'a>;
    type Failure = Segment<'a>;

    fn ingest(self, input: Self::Input) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        match self {
            LeafParser::Idle => {
                if let Ok(atx_heading) = AtxHeading::try_from(input) {
                    return IngestResult::Success(IngestSuccess::Single(atx_heading.into()));
                }

                if let Ok(thematic_break) = ThematicBreak::try_from(input) {
                    return IngestResult::Success(IngestSuccess::Single(thematic_break.into()));
                }

                if let Ok(blank_line) = BlankLine::try_from(input) {
                    return IngestResult::Success(IngestSuccess::Single(blank_line.into()));
                }

                let indented_code_parser = IndentedCodeParser::new();
                match indented_code_parser.ingest(input) {
                    IngestResult::Ready(parser) => {
                        return IngestResult::Ready(Self::IndentedCode(parser))
                    }
                    // Not a possibility on first ingestion.
                    IngestResult::Success(success) => return IngestResult::Success(success.into()),
                    IngestResult::Failure(input) => {
                        let fenced_code_parser = FencedCodeParser::new();
                        match fenced_code_parser.ingest(input) {
                            IngestResult::Ready(parser) => {
                                IngestResult::Ready(Self::FencedCode(parser))
                            }
                            // This is actually not a possibility on first ingestion.
                            IngestResult::Success(fenced_code) => {
                                IngestResult::Success(IngestSuccess::Single(fenced_code.into()))
                            }
                            IngestResult::Failure(segment) => IngestResult::Failure(segment),
                        }
                    }
                }

                // TODO: paragraph comes last.
                // The paragraph parser can definitely produce a setext heading instead of a paragraph.
                // If a valid paragraph is found, then it can also be turned into a sequence of link
                // reference definitions.
                // Finally, several types of segments can interrupt a paragraph and trigger early termination.
            }
            LeafParser::IndentedCode(parser) => match parser.ingest(input) {
                IngestResult::Ready(parser) => IngestResult::Ready(Self::IndentedCode(parser)),
                IngestResult::Success(success) => IngestResult::Success(success.into()),
                IngestResult::Failure(failure) => IngestResult::Failure(failure),
            },
            LeafParser::FencedCode(parser) => match parser.ingest(input) {
                IngestResult::Ready(parser) => IngestResult::Ready(Self::FencedCode(parser)),
                IngestResult::Success(success) => {
                    IngestResult::Success(IngestSuccess::Single(success.into()))
                }
                // Should not happen on the second segment.
                IngestResult::Failure(failure) => IngestResult::Failure(failure),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FinalizeResult<'a> {
    Nothing,
    Single(Leaf<'a>),
    Multiple(Vec<Leaf<'a>>),
}

impl<'a> From<indented_code::FinalizeResult<'a>> for FinalizeResult<'a> {
    fn from(result: indented_code::FinalizeResult<'a>) -> Self {
        match result {
            indented_code::FinalizeResult::Nothing => FinalizeResult::Nothing,
            indented_code::FinalizeResult::Success(result) => {
                let leaf = result.indented_code.into();
                let trailing_blank_lines = result.trailing_blank_lines;
                if trailing_blank_lines.is_empty() {
                    FinalizeResult::Single(leaf)
                } else {
                    let mut leaves = vec![leaf];
                    leaves.extend(
                        trailing_blank_lines
                            .into_iter()
                            .map(BlankLine::from)
                            .map(Leaf::from),
                    );
                    FinalizeResult::Multiple(leaves)
                }
            }
        }
    }
}

impl<'a> Finalize for LeafParser<'a> {
    type Result = FinalizeResult<'a>;

    fn finalize(self) -> Self::Result {
        match self {
            Self::Idle => FinalizeResult::Nothing,
            Self::IndentedCode(parser) => match parser.finalize() {
                // TODO: map result here so that nothing stays nothing and success becomes leaf.
                indented_code::FinalizeResult::Nothing => FinalizeResult::Nothing,
                indented_code::FinalizeResult::Success(result) => {
                    let leaf = result.indented_code.into();
                    let trailing_blank_lines = result.trailing_blank_lines;
                    if trailing_blank_lines.is_empty() {
                        return FinalizeResult::Single(leaf);
                    }
                    let mut leaves = vec![leaf];
                    leaves.extend(
                        trailing_blank_lines
                            .into_iter()
                            .map(BlankLine::from)
                            .map(Leaf::from),
                    );
                    FinalizeResult::Multiple(leaves)
                }
            },
            // TODO: finish fenced code finalize.
            Self::FencedCode(parser) => match parser.finalize() {
                Some(fenced_code) => FinalizeResult::Single(fenced_code.into()),
                None => FinalizeResult::Nothing,
            },
        }
    }
}
