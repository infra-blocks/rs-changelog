use std::ops::Range;

use pulldown_cmark::{Event, HeadingLevel, Tag};
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
#[error("invalid heading size found at {range:?}: expected <=3 found {size}")]
pub struct InvalidHeading {
    size: usize,
    range: Range<usize>,
}

impl TryFrom<(Event<'_>, Range<usize>)> for InvalidHeading {
    type Error = ();

    fn try_from(value: (Event<'_>, Range<usize>)) -> Result<Self, Self::Error> {
        match value.0 {
            Event::Start(Tag::Heading { level, .. }) if level > HeadingLevel::H3 => {
                Ok(InvalidHeading {
                    size: level as usize,
                    range: value.1,
                })
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from {
        use super::*;

        fn heading_start<'a>(level: HeadingLevel) -> Event<'a> {
            let heading = Tag::Heading {
                level,
                id: None,
                classes: vec![],
                attrs: vec![],
            };
            Event::Start(heading)
        }

        #[test]
        fn should_fail_with_heading_1() {
            let result = InvalidHeading::try_from((heading_start(HeadingLevel::H1), (0..10)));
            assert_eq!(result, Err(()));
        }

        #[test]
        fn should_fail_with_heading_2() {
            let result = InvalidHeading::try_from((heading_start(HeadingLevel::H2), (0..10)));
            assert_eq!(result, Err(()));
        }

        #[test]
        fn should_fail_with_heading_3() {
            let result = InvalidHeading::try_from((heading_start(HeadingLevel::H3), (0..10)));
            assert_eq!(result, Err(()));
        }

        #[test]
        fn should_succeed_with_heading_4() {
            let result = InvalidHeading::try_from((heading_start(HeadingLevel::H4), (0..10)));
            assert_eq!(
                result,
                Ok(InvalidHeading {
                    size: 4,
                    range: (0..10)
                })
            );
        }

        #[test]
        fn should_succeed_with_heading_5() {
            let result = InvalidHeading::try_from((heading_start(HeadingLevel::H5), (0..10)));
            assert_eq!(
                result,
                Ok(InvalidHeading {
                    size: 5,
                    range: (0..10)
                })
            );
        }

        #[test]
        fn should_succeed_with_heading_6() {
            let result = InvalidHeading::try_from((heading_start(HeadingLevel::H6), (0..10)));
            assert_eq!(
                result,
                Ok(InvalidHeading {
                    size: 6,
                    range: (0..10)
                })
            );
        }
    }
}
