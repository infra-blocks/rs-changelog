use std::{error::Error, fmt::Display};

use crate::parse::{
    ast::Ast,
    releases::{ChangesParseError, Release, ReleaseParseError, Unreleased, UnreleasedParseError},
    title::{Title, TitleParseError},
};

// TODO: implement ToOwned
#[derive(Debug, Clone, PartialEq)]
pub struct Changelog<'source> {
    source: &'source str,
    title: Title,
    /// The unreleased section of a document is optional, as it would basically become empty
    /// after each release. So, whether the user decides to have one or not, is up to them.
    unreleased: Option<Unreleased>,
    releases: Vec<Release>,
}

impl<'source> Changelog<'source> {
    fn new(
        source: &'source str,
        title: Title,
        unreleased: Option<Unreleased>,
        releases: Vec<Release>,
    ) -> Self {
        Self {
            source,
            title,
            unreleased,
            releases,
        }
    }

    pub fn parse(source: &'source str) -> Result<Changelog<'source>, ChangelogParseError> {
        let mut ast = Ast::from(source);
        let title = Title::parse(&mut ast)?;
        let unreleased = match Unreleased::parse(&mut ast) {
            Ok(unreleased) => Some(unreleased),
            Err(err) => match err {
                UnreleasedParseError::InvalidHeading(_) => None,
                UnreleasedParseError::InvalidChanges(err) => {
                    return Err(ChangelogParseError::InvalidUnreleased(err));
                }
            },
        };
        let mut releases = vec![];
        loop {
            match Release::parse(&mut ast) {
                Ok(release) => releases.push(release),
                // If we were able to construct at least one release,
                // we may just be at the end, or reaching the ref defs.
                Err(_) if !releases.is_empty() => break,
                // TODO: should there really be this rule?
                // It is considered an error to not have a single release
                // at this moment, but that could change.
                Err(err) => return Err(err.into()),
            }
        }

        Ok(Changelog::new(source, title, unreleased, releases))
    }

    pub fn releases(&self) -> &[Release] {
        &self.releases
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangelogParseError {
    InvalidTitle(TitleParseError),
    // The unreleased parsing can only fail for invalid changes. An invalid heading simply
    // moves on to the releases parsing.
    InvalidUnreleased(ChangesParseError),
    InvalidRelease(ReleaseParseError),
}

impl From<TitleParseError> for ChangelogParseError {
    fn from(err: TitleParseError) -> Self {
        ChangelogParseError::InvalidTitle(err)
    }
}

impl From<ReleaseParseError> for ChangelogParseError {
    fn from(value: ReleaseParseError) -> Self {
        Self::InvalidRelease(value)
    }
}

impl Display for ChangelogParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while parsing changelog: ",)?;

        // TODO: finish this gooooood.
        match self {
            ChangelogParseError::InvalidTitle(err) => write!(f, "{}", err),
            ChangelogParseError::InvalidUnreleased(err) => write!(f, "{:?}", err),
            ChangelogParseError::InvalidRelease(err) => write!(f, "{:?}", err),
        }
    }
}

impl Error for ChangelogParseError {}
