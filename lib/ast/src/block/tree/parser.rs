use std::fmt::Debug;

use crate::Segment;

/// Trait to encapsulate stateful parser behavior in a standard interface.
///
/// Note that usage of this trait is only recommended for parsers that are stateful in nature.
/// Stateless parsing should preferrably be implemented using more common traits, like [TryFrom] or [From]
/// for example.
///
/// The parser's construction is not part of this interface on purpose.
/// Some parsers can be constructed empty, while others may require some initial state.
/// Some factory function could produce a parser for any segment, whereas some others could fail.
/// Some parsers could return the same type of result upon instantiation as they would mid consumption.
pub trait Parser<'a>
where
    Self: Sized,
{
    type Result;

    /// Consumes an additional segment.
    ///
    /// When the parser can consume more segments, it returns a [ParseResult::Ready] variant
    /// container itself. When the parser cannot take anymore segments, it returns the final parse
    /// result a [ParseResult::Finalized] variant.
    fn consume(self, segment: Segment<'a>) -> ParserState<Self, Self::Result>;

    /// Finalizes the parser and returns the parsed result.
    fn finalize(self) -> Self::Result;

    /// Convenience default implementation to continuously consume segments until the parser is finalized.
    ///
    /// If the parser hasn't finalized by the time the input is exhausted, this function will explicitly
    /// finalize it. It is also possible that the parser finalizes early and that the input is not
    /// exhausted when this function returns.
    fn consume_all<T: Iterator<Item = Segment<'a>>>(self, segments: &mut T) -> Self::Result {
        let mut current = self;
        for segment in segments {
            match current.consume(segment) {
                ParserState::Ready(next) => current = next,
                ParserState::Finalized(result) => return result,
            }
        }
        // If we reach the end of input before the parser has finalized, we finalize it.
        current.finalize()
    }
}

/// The result returned by a parser after consuming a segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserState<R, F> {
    /// This variant wraps a parser that is ready to receive more segments.
    Ready(R),
    /// This variant wraps the finalized result of the parser.
    Finalized(F),
}

impl<R: Debug, F: Debug> ParserState<R, F> {
    /// Unwraps the finalized result.
    ///
    /// # Panics
    ///
    /// Panics when the parser state is not finalized.
    pub fn unwrap_finalized(self) -> F {
        match self {
            ParserState::Finalized(result) => result,
            _ => panic!("cannot unwrap finalized on {:?}", self),
        }
    }

    /// Unwraps the ready parser.
    ///
    /// # Panics
    ///
    /// Panics when the parser state is not ready.
    pub fn unwrap_ready(self) -> R {
        match self {
            ParserState::Ready(parser) => parser,
            _ => panic!("cannot unwrap ready on {:?}", self),
        }
    }
}

/// A result type that is conventional for parsers that could fail to parse input at any time.
///
/// Upon success, the parsed type is returned. Upon failure, all the segments that had been
/// accumulated so far, including the one triggering the failure, are returned in the order
/// they were provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseResult<'a, T> {
    Parsed(T),
    Rejected(Vec<Segment<'a>>),
}

impl<'a, T: Debug> ParseResult<'a, T> {
    /// Unwraps the parsed result.
    ///
    /// # Panics
    ///
    /// Panics when the result is not [ParseResult::Parsed].
    pub fn unwrap_parsed(self) -> T {
        match self {
            ParseResult::Parsed(result) => result,
            _ => panic!("cannot unwrap parsed on {:?}", self),
        }
    }

    /// Unwraps the rejected segments.
    ///
    /// # Panics
    ///
    /// Panics when the parser result is not [ParseResult::Rejected].
    pub fn unwrap_rejected(self) -> Vec<Segment<'a>> {
        match self {
            ParseResult::Rejected(segments) => segments,
            _ => panic!("cannot unwrap rejected on {:?}", self),
        }
    }
}

/// A conventional result type for parsers tha can do partial parsing.
///
/// This can happen when, for example, the parser receives a new segment that signifies
/// that the result needs to be finalized, but the new segment cannot be consumed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartialParseResult<'a, T> {
    /// This variant signifies that all the segments were used to parsed the result.
    Parsed(T),
    /// This variant signifies that only a subset of the segments were used in the parsing,
    /// the unused ones are returned in the order they were provided.
    Partial(T, Vec<Segment<'a>>),
    /// This variant signifies that no correct result could be parsed, and all segments
    /// are returned in the order they were provided.
    Rejected(Vec<Segment<'a>>),
}

impl<'a, T: Debug> PartialParseResult<'a, T> {
    /// Unwraps the parsed result.
    ///
    /// # Panics
    ///
    /// Panics when the result is not [PartialParseResult::Parsed].
    pub fn unwrap_parsed(self) -> T {
        match self {
            PartialParseResult::Parsed(result) => result,
            _ => panic!("cannot unwrap parsed on {:?}", self),
        }
    }

    /// Unwraps the partial result.
    ///
    /// # Panics
    ///
    /// Panics when the result is not [PartialParseResult::Partial].
    pub fn unwrap_partial(self) -> (T, Vec<Segment<'a>>) {
        match self {
            PartialParseResult::Partial(result, segments) => (result, segments),
            _ => panic!("cannot unwrap partial on {:?}", self),
        }
    }

    /// Unwraps the rejected segments.
    ///
    /// # Panics
    ///
    /// Panics when the parser result is not [PartialParseResult::Rejected].
    pub fn unwrap_rejected(self) -> Vec<Segment<'a>> {
        match self {
            PartialParseResult::Rejected(segments) => segments,
            _ => panic!("cannot unwrap rejected on {:?}", self),
        }
    }
}
