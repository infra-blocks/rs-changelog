use std::{collections::VecDeque, error::Error, fmt::Display};

use changelog_ast::{AstIterator, Node};

use crate::parse::{
    changelog::Changelog,
    releases::{ChangesParseError, Releases, ReleasesParseError, Unreleased, UnreleasedParseError},
    title::{Title, TitleParseError},
};

// TODO: could try to just reverse the vec if the parsing always goes in the same direction instead.
pub(crate) type Unparsed<'source> = VecDeque<Node<'source>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidTitle(TitleParseError),
    // The unreleased parsing can only fail for invalid changes. An invalid heading simply
    // moves on to the releases parsing.
    InvalidUnreleased(ChangesParseError),
    InvalidReleases(ReleasesParseError),
}

impl From<TitleParseError> for ParseError {
    fn from(err: TitleParseError) -> Self {
        ParseError::InvalidTitle(err)
    }
}

impl From<ReleasesParseError> for ParseError {
    fn from(value: ReleasesParseError) -> Self {
        Self::InvalidReleases(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while parsing changelog: ",)?;

        // TODO: finish this gooooood.
        match self {
            ParseError::InvalidTitle(err) => write!(f, "{}", err),
            ParseError::InvalidUnreleased(err) => write!(f, "{:?}", err),
            ParseError::InvalidReleases(err) => write!(f, "{:?}", err),
        }
    }
}

impl Error for ParseError {}

pub struct ChangelogParser {}

impl ChangelogParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse<'source>(&self, source: &'source str) -> Result<Changelog<'source>, ParseError> {
        let branches = AstIterator::new(source);

        let mut ast: VecDeque<_> = branches.collect();
        let title = Title::parse(&mut ast)?;
        let unreleased = match Unreleased::parse(&mut ast) {
            Ok(unreleased) => Some(unreleased),
            Err(err) => match err {
                UnreleasedParseError::InvalidHeading(_) => None,
                UnreleasedParseError::InvalidChanges(err) => {
                    return Err(ParseError::InvalidUnreleased(err));
                }
            },
        };
        let releases = Releases::parse(&mut ast)?;

        Ok(Changelog {
            source,
            title,
            unreleased,
            releases,
        })
    }
}
