mod debug;
mod lint;
mod parse;

use std::{error::Error, fmt::Display};

pub use debug::*;
pub use parse::*;

use crate::lint::ChangelogLintError;

pub fn parse(source: &str) -> Result<Changelog<'_>, ChangelogCheckError> {
    let changelog = Changelog::parse(source)?;
    changelog.lint()?;
    Ok(changelog)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangelogCheckError {
    ParseError(ChangelogParseError),
    LintError(ChangelogLintError),
}

impl From<ChangelogParseError> for ChangelogCheckError {
    fn from(value: ChangelogParseError) -> Self {
        Self::ParseError(value)
    }
}

impl From<ChangelogLintError> for ChangelogCheckError {
    fn from(value: ChangelogLintError) -> Self {
        Self::LintError(value)
    }
}

impl Display for ChangelogCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangelogCheckError::ParseError(err) => write!(f, "{:?}", err),
            ChangelogCheckError::LintError(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ChangelogCheckError {}
