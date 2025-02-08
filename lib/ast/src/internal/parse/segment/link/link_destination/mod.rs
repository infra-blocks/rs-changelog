mod bracketed;
mod loose;

pub use bracketed::*;
pub use loose::*;

use crate::internal::parse::try_extract::{Extraction, TryExtract};
use segment::Segment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkDestinationSegment<'a> {
    Bracketed(BracketedLinkDestinationSegment<'a>),
    Loose(LooseLinkDestinationSegment<'a>),
}

impl<'a> TryFrom<Segment<'a>> for LinkDestinationSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        match BracketedLinkDestinationSegment::try_from(segment) {
            Ok(bracketed) => Ok(Self::Bracketed(bracketed)),
            Err(segment) => LooseLinkDestinationSegment::try_from(segment).map(Self::Loose),
        }
    }
}

impl<'a> TryExtract<Segment<'a>> for LinkDestinationSegment<'a> {
    type Remaining = Segment<'a>;
    type Error = Segment<'a>;

    fn try_extract(segment: Segment<'a>) -> Result<Extraction<Self, Segment<'a>>, Self::Error> {
        match BracketedLinkDestinationSegment::try_extract(segment) {
            Ok(extraction) => Ok(extraction.map_extracted(LinkDestinationSegment::Bracketed)),
            Err(segment) => LooseLinkDestinationSegment::try_extract(segment)
                .map(|extraction| extraction.map_extracted(LinkDestinationSegment::Loose)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from {
        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        LinkDestinationSegment::try_from($segment.clone()),
                        Err($segment)
                    );
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(LinkDestinationSegment::try_from($segment), Ok($expected));
                }
            };
        }

        failure_case!(should_reject_empty_segment, Segment::default());
        failure_case!(should_reject_blank_line, Segment::first("\n"));

        success_case!(
            should_work_with_a_bracketed_variant,
            Segment::first("<bracketed>"),
            LinkDestinationSegment::Bracketed(
                BracketedLinkDestinationSegment::try_from(Segment::first("<bracketed>")).unwrap()
            )
        );
        success_case!(
            should_work_with_a_loose_variant,
            Segment::first("loose"),
            LinkDestinationSegment::Loose(
                LooseLinkDestinationSegment::try_from(Segment::first("loose")).unwrap()
            )
        );
    }

    mod try_extract {
        use super::*;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        LinkDestinationSegment::try_extract($segment.clone()),
                        Err($segment)
                    );
                }
            };
        }

        macro_rules! success_case {
            (
                $test:ident,
                $segment:expr,
                $expected_link_destination:expr,
                $expected_remaining:expr
            ) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        LinkDestinationSegment::try_extract($segment.clone()),
                        Ok(Extraction::new(
                            $expected_link_destination,
                            $expected_remaining
                        ))
                    );
                }
            };
        }

        failure_case!(should_reject_empty_segment, Segment::default());
        failure_case!(should_reject_blank_line, Segment::first("\n"));

        success_case!(
            should_work_with_a_bracketed_variant,
            Segment::first("<bracketed> stuff"),
            LinkDestinationSegment::Bracketed(
                BracketedLinkDestinationSegment::try_from(Segment::first("<bracketed>")).unwrap()
            ),
            Segment::new(location::Position::new(1, 12, 11), " stuff")
        );
        success_case!(
            should_work_with_a_loose_variant,
            Segment::first("loose stuff"),
            LinkDestinationSegment::Loose(
                LooseLinkDestinationSegment::try_from(Segment::first("loose")).unwrap()
            ),
            Segment::new(location::Position::new(1, 6, 5), " stuff")
        );
    }
}
