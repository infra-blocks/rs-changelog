mod change_set;
mod changes;
mod heading;
mod info;
mod release;
mod unreleased;

pub use changes::*;
pub use heading::*;
pub use info::*;
pub use unreleased::*;

use crate::parse::{
    ast::Ast,
    releases::release::{Release, ReleaseParseError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Releases {
    // TODO: move the logic into Changelog
    releases: Vec<Release>,
}

impl Releases {
    pub fn new(releases: Vec<Release>) -> Self {
        Self { releases }
    }

    pub(crate) fn parse(ast: &mut Ast) -> Result<Self, ReleasesParseError> {
        // TODO: parse unreleased

        let mut releases = vec![];
        loop {
            match Release::parse(ast) {
                Ok(release) => releases.push(release),
                Err(_) if !releases.is_empty() => return Ok(Self::new(releases)),
                Err(err) => return Err(err.into()),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleasesParseError {
    // InvalidUnreleased
    InvalidRelease(ReleaseParseError),
}

impl From<ReleaseParseError> for ReleasesParseError {
    fn from(value: ReleaseParseError) -> Self {
        Self::InvalidRelease(value)
    }
}
