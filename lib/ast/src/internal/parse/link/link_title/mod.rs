mod double_quotes;
mod parentheses;
mod single_quotes;

use std::convert::Infallible;

// TODO: don't expose inner parsers.
pub use double_quotes::*;
pub use parentheses::*;
use segment::Segment;
pub use single_quotes::*;

use crate::internal::{
    parse::parser::{Finalize, Ingest, IngestResult},
    utils::unwrap_singleton::UnwrapSingleton,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkTitle<'a> {
    SingleQuotes(SingleQuotesLinkTitle<'a>),
    DoubleQuotes(DoubleQuotesLinkTitle<'a>),
    Parentheses(ParenthesesLinkTitle<'a>),
}

impl<'a> From<SingleQuotesLinkTitle<'a>> for LinkTitle<'a> {
    fn from(inner: SingleQuotesLinkTitle<'a>) -> Self {
        Self::SingleQuotes(inner)
    }
}

impl<'a> From<DoubleQuotesLinkTitle<'a>> for LinkTitle<'a> {
    fn from(inner: DoubleQuotesLinkTitle<'a>) -> Self {
        Self::DoubleQuotes(inner)
    }
}

impl<'a> From<ParenthesesLinkTitle<'a>> for LinkTitle<'a> {
    fn from(inner: ParenthesesLinkTitle<'a>) -> Self {
        Self::Parentheses(inner)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkTitleParser<'a> {
    Idle,
    SingleQuotes(SingleQuotesLinkTitleParser<'a>),
    DoubleQuotes(DoubleQuotesLinkTitleParser<'a>),
    Parentheses(ParenthesesLinkTitleParser<'a>),
}

impl<'a> Default for LinkTitleParser<'a> {
    fn default() -> Self {
        Self::Idle
    }
}

impl<'a> LinkTitleParser<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> From<SingleQuotesLinkTitleParser<'a>> for LinkTitleParser<'a> {
    fn from(inner: SingleQuotesLinkTitleParser<'a>) -> Self {
        Self::SingleQuotes(inner)
    }
}

impl<'a> From<DoubleQuotesLinkTitleParser<'a>> for LinkTitleParser<'a> {
    fn from(inner: DoubleQuotesLinkTitleParser<'a>) -> Self {
        Self::DoubleQuotes(inner)
    }
}

impl<'a> From<ParenthesesLinkTitleParser<'a>> for LinkTitleParser<'a> {
    fn from(inner: ParenthesesLinkTitleParser<'a>) -> Self {
        Self::Parentheses(inner)
    }
}

impl<'a> Ingest for LinkTitleParser<'a> {
    type Input = Segment<'a>;
    type Ready = Self;
    type Success = LinkTitle<'a>;
    type Failure = Vec<Segment<'a>>;

    fn ingest(
        self,
        segment: Self::Input,
    ) -> IngestResult<Self::Ready, Self::Success, Self::Failure> {
        match self {
            LinkTitleParser::Idle => {
                let inner = SingleQuotesLinkTitleParser::new();
                inner
                    .ingest(segment)
                    .map_ready(Self::from)
                    .map_success(Self::Success::from)
                    .on_failure(|segments| {
                        let inner = DoubleQuotesLinkTitleParser::new();
                        inner
                            .ingest(segments.unwrap_singleton())
                            .map_ready(Self::from)
                            .map_success(Self::Success::from)
                            .on_failure(|segments| {
                                let inner = ParenthesesLinkTitleParser::new();
                                inner
                                    .ingest(segments.unwrap_singleton())
                                    .map_ready(Self::from)
                                    .map_success(Self::Success::from)
                            })
                    })
            }
            LinkTitleParser::SingleQuotes(inner) => inner
                .ingest(segment)
                .map_ready(Self::from)
                .map_success(Self::Success::from),
            LinkTitleParser::DoubleQuotes(inner) => inner
                .ingest(segment)
                .map_ready(Self::from)
                .map_success(Self::Success::from),
            LinkTitleParser::Parentheses(inner) => inner
                .ingest(segment)
                .map_ready(Self::from)
                .map_success(Self::Success::from),
        }
    }
}

impl<'a> Finalize for LinkTitleParser<'a> {
    type Result = Result<Infallible, Vec<Segment<'a>>>;

    fn finalize(self) -> Self::Result {
        match self {
            LinkTitleParser::Idle => Err(Vec::new()),
            LinkTitleParser::SingleQuotes(inner) => inner.finalize(),
            LinkTitleParser::DoubleQuotes(inner) => inner.finalize(),
            LinkTitleParser::Parentheses(inner) => inner.finalize(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::internal::parse::segment::{
        DoubleQuotesLinkTitleOpeningSegment, ParenthesesLinkTitleOpeningSegment,
        SingleQuotesLinkTitleOpeningSegment,
    };

    #[test]
    fn idle_reject() {
        let parser = LinkTitleParser::new();
        let segment = Segment::first("title");
        assert_eq!(parser.ingest(segment).unwrap_failure(), vec![segment]);
    }

    #[test]
    fn idle_success_single_quotes() {
        let parser = LinkTitleParser::new();
        let segment = Segment::first("'title'");
        assert_eq!(
            parser.ingest(segment).unwrap_success(),
            LinkTitle::SingleQuotes(SingleQuotesLinkTitle::new(
                SingleQuotesLinkTitleOpeningSegment::try_from(segment).unwrap(),
                Vec::new()
            ))
        );
    }

    #[test]
    fn idle_success_double_quotes() {
        let parser = LinkTitleParser::new();
        let segment = Segment::first("\"title\"");
        assert_eq!(
            parser.ingest(segment).unwrap_success(),
            LinkTitle::DoubleQuotes(DoubleQuotesLinkTitle::new(
                DoubleQuotesLinkTitleOpeningSegment::try_from(segment).unwrap(),
                Vec::new()
            ))
        );
    }

    #[test]
    fn idle_success_parentheses() {
        let parser = LinkTitleParser::new();
        let segment = Segment::first("(title)");
        assert_eq!(
            parser.ingest(segment).unwrap_success(),
            LinkTitle::Parentheses(ParenthesesLinkTitle::new(
                ParenthesesLinkTitleOpeningSegment::try_from(segment).unwrap(),
                Vec::new()
            ))
        );
    }
}
