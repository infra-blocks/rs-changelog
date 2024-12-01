mod double_quotes;
mod parentheses;
mod single_quotes;

pub use double_quotes::*;
pub use parentheses::*;
pub use single_quotes::*;

use crate::Segment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkTitleOpeningSegment<'a> {
    //TODO: single quotes, double quotes, parentheses
    SingleQuotes(SingleQuotesLinkTitleOpeningSegment<'a>),
    DoubleQuotes(DoubleQuotesLinkTitleOpeningSegment<'a>),
    Parentheses(ParenthesesLinkTitleOpeningSegment<'a>),
}

impl<'a> LinkTitleOpeningSegment<'a> {
    pub fn is_closing(&self) -> bool {
        match self {
            Self::SingleQuotes(single_quotes) => single_quotes.is_closing(),
            Self::DoubleQuotes(double_quotes) => double_quotes.is_closing(),
            Self::Parentheses(parentheses) => parentheses.is_closing(),
        }
    }
}

impl<'a> TryFrom<Segment<'a>> for LinkTitleOpeningSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        match SingleQuotesLinkTitleOpeningSegment::try_from(segment) {
            Ok(single_quotes) => Ok(Self::SingleQuotes(single_quotes)),
            Err(segment) => match DoubleQuotesLinkTitleOpeningSegment::try_from(segment) {
                Ok(double_quotes) => Ok(Self::DoubleQuotes(double_quotes)),
                Err(segment) => match ParenthesesLinkTitleOpeningSegment::try_from(segment) {
                    Ok(parentheses) => Ok(Self::Parentheses(parentheses)),
                    Err(segment) => Err(segment),
                },
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkTitleContinuationSegment<'a> {
    SingleQuotes(SingleQuotesLinkTitleContinuationSegment<'a>),
    DoubleQuotes(DoubleQuotesLinkTitleContinuationSegment<'a>),
    Parentheses(ParenthesesLinkTitleContinuationSegment<'a>),
}

impl<'a> LinkTitleContinuationSegment<'a> {
    pub fn is_closing(&self) -> bool {
        match self {
            Self::SingleQuotes(single_quotes) => single_quotes.is_closing(),
            Self::DoubleQuotes(double_quotes) => double_quotes.is_closing(),
            Self::Parentheses(parentheses) => parentheses.is_closing(),
        }
    }
}

impl<'a> TryFrom<Segment<'a>> for LinkTitleContinuationSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        match SingleQuotesLinkTitleContinuationSegment::try_from(segment) {
            Ok(single_quotes) => Ok(Self::SingleQuotes(single_quotes)),
            Err(segment) => match DoubleQuotesLinkTitleContinuationSegment::try_from(segment) {
                Ok(double_quotes) => Ok(Self::DoubleQuotes(double_quotes)),
                Err(segment) => match ParenthesesLinkTitleContinuationSegment::try_from(segment) {
                    Ok(parentheses) => Ok(Self::Parentheses(parentheses)),
                    Err(segment) => Err(segment),
                },
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl<'a> LinkTitleOpeningSegment<'a> {
        pub fn unwrap_single_quotes(self) -> SingleQuotesLinkTitleOpeningSegment<'a> {
            match self {
                Self::SingleQuotes(single_quotes) => single_quotes,
                _ => panic!("cannot unwrap single quotes on {:?}", self),
            }
        }

        pub fn unwrap_double_quotes(self) -> DoubleQuotesLinkTitleOpeningSegment<'a> {
            match self {
                Self::DoubleQuotes(double_quotes) => double_quotes,
                _ => panic!("cannot unwrap double quotes on {:?}", self),
            }
        }

        pub fn unwrap_parentheses(self) -> ParenthesesLinkTitleOpeningSegment<'a> {
            match self {
                Self::Parentheses(parentheses) => parentheses,
                _ => panic!("cannot unwrap parentheses on {:?}", self),
            }
        }
    }

    impl<'a> LinkTitleContinuationSegment<'a> {
        pub fn unwrap_single_quotes(self) -> SingleQuotesLinkTitleContinuationSegment<'a> {
            match self {
                Self::SingleQuotes(single_quotes) => single_quotes,
                _ => panic!("cannot unwrap single quotes on {:?}", self),
            }
        }

        pub fn unwrap_double_quotes(self) -> DoubleQuotesLinkTitleContinuationSegment<'a> {
            match self {
                Self::DoubleQuotes(double_quotes) => double_quotes,
                _ => panic!("cannot unwrap double quotes on {:?}", self),
            }
        }

        pub fn unwrap_parentheses(self) -> ParenthesesLinkTitleContinuationSegment<'a> {
            match self {
                Self::Parentheses(parentheses) => parentheses,
                _ => panic!("cannot unwrap parentheses on {:?}", self),
            }
        }
    }

    mod opening {
        use super::*;

        #[test]
        fn should_work_with_single_quotes() {
            let segment = Segment::first("'\n");
            let result = LinkTitleOpeningSegment::try_from(segment.clone())
                .unwrap()
                .unwrap_single_quotes();
            assert_eq!(result.segment, segment);
            assert_eq!(result.is_closing(), false);
        }

        #[test]
        fn should_work_with_double_quotes() {
            let segment = Segment::first("\"\n");
            let result = LinkTitleOpeningSegment::try_from(segment.clone())
                .unwrap()
                .unwrap_double_quotes();
            assert_eq!(result.segment, segment);
            assert_eq!(result.is_closing(), false);
        }

        #[test]
        fn should_work_with_parentheses() {
            let segment = Segment::first("(\n");
            let result = LinkTitleOpeningSegment::try_from(segment.clone())
                .unwrap()
                .unwrap_parentheses();
            assert_eq!(result.segment, segment);
            assert_eq!(result.is_closing(), false);
        }
    }

    mod continuation {
        use super::*;

        // Because there is no distinguishing factor between most continuation segment, we only focus on the ones that are closing.
        #[test]
        fn should_work_with_single_quotes() {
            let segment = Segment::first("'");
            let continuation_segment =
                LinkTitleContinuationSegment::try_from(segment.clone()).unwrap();
            assert!(continuation_segment.is_closing());
            let single_quotes = continuation_segment.unwrap_single_quotes();
            assert_eq!(single_quotes.segment, segment);
            assert!(single_quotes.is_closing());
        }

        #[test]
        fn should_work_with_double_quotes() {
            let segment = Segment::first("\"");
            let continuation_segment =
                LinkTitleContinuationSegment::try_from(segment.clone()).unwrap();
            assert!(continuation_segment.is_closing());
            let double_quotes = continuation_segment.unwrap_double_quotes();
            assert_eq!(double_quotes.segment, segment);
            assert!(double_quotes.is_closing());
        }

        #[test]
        fn should_work_with_parentheses() {
            let segment = Segment::first(")");
            let continuation_segment =
                LinkTitleContinuationSegment::try_from(segment.clone()).unwrap();
            assert!(continuation_segment.is_closing());
            let parentheses = continuation_segment.unwrap_parentheses();
            assert_eq!(parentheses.segment, segment);
            assert!(parentheses.is_closing());
        }
    }
}
