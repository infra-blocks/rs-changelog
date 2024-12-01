mod equals;
mod hyphens;

pub use equals::*;
pub use hyphens::*;

use crate::Segment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetextHeadingUnderlineSegment<'a> {
    Equals(SetextHeadingEqualsUnderlineSegment<'a>),
    Hyphens(SetextHeadingHyphensUnderlineSegment<'a>),
}

impl<'a> SetextHeadingUnderlineSegment<'a> {
    pub fn level(&self) -> u8 {
        match self {
            Self::Equals(segment) => segment.level(),
            Self::Hyphens(segment) => segment.level(),
        }
    }
}

impl<'a> From<SetextHeadingEqualsUnderlineSegment<'a>> for SetextHeadingUnderlineSegment<'a> {
    fn from(segment: SetextHeadingEqualsUnderlineSegment<'a>) -> Self {
        Self::Equals(segment)
    }
}

impl<'a> From<SetextHeadingHyphensUnderlineSegment<'a>> for SetextHeadingUnderlineSegment<'a> {
    fn from(segment: SetextHeadingHyphensUnderlineSegment<'a>) -> Self {
        Self::Hyphens(segment)
    }
}

impl<'a> TryFrom<Segment<'a>> for SetextHeadingUnderlineSegment<'a> {
    type Error = Segment<'a>;

    fn try_from(segment: Segment<'a>) -> Result<Self, Self::Error> {
        if let Ok(segment) = SetextHeadingEqualsUnderlineSegment::try_from(segment) {
            Ok(segment.into())
        } else if let Ok(segment) = SetextHeadingHyphensUnderlineSegment::try_from(segment) {
            Ok(segment.into())
        } else {
            Err(segment)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! failure_case {
        ($test:ident, $segment:expr) => {
            #[test]
            fn $test() {
                assert_eq!(
                    SetextHeadingUnderlineSegment::try_from($segment.clone()),
                    Err($segment)
                );
            }
        };
    }

    macro_rules! success_case {
        ($test:ident, $segment:expr, $expected:expr) => {
            #[test]
            fn $test() {
                assert_eq!(
                    SetextHeadingUnderlineSegment::try_from($segment),
                    Ok($expected)
                );
            }
        };
    }

    failure_case!(should_reject_empty, Segment::default());
    failure_case!(should_reject_blank_line, Segment::first("\n"));

    success_case!(
        should_accept_equals,
        Segment::first("=\n"),
        SetextHeadingUnderlineSegment::Equals(
            SetextHeadingEqualsUnderlineSegment::try_from(Segment::first("=\n")).unwrap()
        )
    );
    success_case!(
        should_accept_hyphens,
        Segment::first("-\n"),
        SetextHeadingUnderlineSegment::Hyphens(
            SetextHeadingHyphensUnderlineSegment::try_from(Segment::first("-\n")).unwrap()
        )
    );
}
