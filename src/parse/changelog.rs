use std::{collections::VecDeque, error::Error, fmt::Display};

use changelog_ast::AstIterator;

use crate::parse::{
    releases::{ChangesParseError, Releases, ReleasesParseError, Unreleased, UnreleasedParseError},
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
    releases: Releases,
}

impl<'source> Changelog<'source> {
    fn new(
        source: &'source str,
        title: Title,
        unreleased: Option<Unreleased>,
        releases: Releases,
    ) -> Self {
        Self {
            source,
            title,
            unreleased,
            releases,
        }
    }

    pub fn parse(source: &'source str) -> Result<Changelog<'source>, ChangelogParseError> {
        let mut ast: VecDeque<_> = AstIterator::new(source).collect();
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
        let releases = Releases::parse(&mut ast)?;

        Ok(Changelog::new(source, title, unreleased, releases))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangelogParseError {
    InvalidTitle(TitleParseError),
    // The unreleased parsing can only fail for invalid changes. An invalid heading simply
    // moves on to the releases parsing.
    InvalidUnreleased(ChangesParseError),
    InvalidReleases(ReleasesParseError),
}

impl From<TitleParseError> for ChangelogParseError {
    fn from(err: TitleParseError) -> Self {
        ChangelogParseError::InvalidTitle(err)
    }
}

impl From<ReleasesParseError> for ChangelogParseError {
    fn from(value: ReleasesParseError) -> Self {
        Self::InvalidReleases(value)
    }
}

impl Display for ChangelogParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while parsing changelog: ",)?;

        // TODO: finish this gooooood.
        match self {
            ChangelogParseError::InvalidTitle(err) => write!(f, "{}", err),
            ChangelogParseError::InvalidUnreleased(err) => write!(f, "{:?}", err),
            ChangelogParseError::InvalidReleases(err) => write!(f, "{:?}", err),
        }
    }
}

impl Error for ChangelogParseError {}
