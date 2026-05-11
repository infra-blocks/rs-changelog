use chrono::NaiveDate;
use semver::Version;

use crate::parse::{
    ast::Ast,
    releases::{Changes, ChangesParseError, ReleaseHeading, ReleaseHeadingParseError},
};

// TODO: simplify this struct? I'm thinking something like... { heading_range, version, date, changes}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    heading: ReleaseHeading,
    changes: Changes,
}

impl Release {
    pub fn new(heading: ReleaseHeading, changes: Changes) -> Self {
        Self { heading, changes }
    }

    pub fn version(&self) -> &Version {
        self.heading.version()
    }

    pub fn date(&self) -> &NaiveDate {
        self.heading.date()
    }

    pub(crate) fn parse(ast: &mut Ast) -> Result<Self, ReleaseParseError> {
        let heading = ReleaseHeading::parse(ast)?;
        let changes = Changes::parse(ast)?;

        Ok(Release::new(heading, changes))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseParseError {
    InvalidHeading(ReleaseHeadingParseError),
    InvalidChanges(ChangesParseError),
}

impl From<ReleaseHeadingParseError> for ReleaseParseError {
    fn from(value: ReleaseHeadingParseError) -> Self {
        Self::InvalidHeading(value)
    }
}

impl From<ChangesParseError> for ReleaseParseError {
    fn from(value: ChangesParseError) -> Self {
        Self::InvalidChanges(value)
    }
}

// TODO: unit tests.
