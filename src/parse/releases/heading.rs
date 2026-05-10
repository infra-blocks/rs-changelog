use std::ops::Range;

use changelog_ast::{Heading, HeadingLevel, Node};
use semver::Version;

use crate::parse::{
    ast::Ast,
    releases::{ReleaseInfo, ReleaseInfoParseError},
};

// TODO: rename file as we now have a unreleased heading mafock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseHeading {
    range: Range<usize>,
    // TODO: consider flattening this struct.
    info: ReleaseInfo,
}

impl ReleaseHeading {
    pub fn new(range: Range<usize>, info: ReleaseInfo) -> Self {
        Self { range, info }
    }

    pub(crate) fn parse(ast: &mut Ast) -> Result<ReleaseHeading, ReleaseHeadingParseError> {
        // The first node has to be a heading.
        let Some(first) = ast.front() else {
            return Err(ReleaseHeadingParseError::Empty);
        };

        let info = match first {
            Node::Heading(Heading {
                range: _,
                children,
                level: HeadingLevel::H2,
                id: _,
                classes: _,
                attrs: _,
            }) => ReleaseInfo::parse(children)?,
            _ => {
                return Err(ReleaseHeadingParseError::InvalidHeading(
                    first.range().clone(),
                ));
            }
        };
        let first = ast.pop_front().unwrap().unwrap_heading();
        Ok(ReleaseHeading::new(first.range, info))
    }

    pub fn version(&self) -> &Version {
        &self.info.version
    }
}

// TODO: add ranges on errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseHeadingParseError {
    Empty,
    InvalidHeading(Range<usize>),
    InvalidReleaseInfo(ReleaseInfoParseError),
}

impl From<ReleaseInfoParseError> for ReleaseHeadingParseError {
    fn from(value: ReleaseInfoParseError) -> Self {
        Self::InvalidReleaseInfo(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse_release_heading {
        use chrono::NaiveDate;
        use semver::Version;

        use super::*;

        #[test]
        fn should_error_with_invalid_heading() {
            let mut ast = Ast::from(
                "# [0.1.0] - 2024-05-01\n[0.1.0]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
            );
            let result = ReleaseHeading::parse(&mut ast);
            assert_eq!(result, Err(ReleaseHeadingParseError::InvalidHeading(0..23)))
        }

        #[test]
        fn should_work_with_valid_release() {
            let mut ast = Ast::from(
                "## [0.1.0] - 2024-05-01\n[0.1.0]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
            );
            let result = ReleaseHeading::parse(&mut ast);
            assert_eq!(
                result,
                Ok(ReleaseHeading::new(
                    0..24,
                    ReleaseInfo::new(
                        Version::new(0, 1, 0),
                        NaiveDate::from_ymd_opt(2024, 5, 1).unwrap()
                    )
                ))
            )
        }
    }
}
