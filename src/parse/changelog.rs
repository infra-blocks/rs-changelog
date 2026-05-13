use std::{error::Error, fmt::Display};

use crate::parse::{
    ast::Ast,
    reference_definition::ReferenceDefinition,
    releases::{ChangesParseError, Release, ReleaseParseError, Unreleased, UnreleasedParseError},
    title::{Title, TitleParseError},
};

// TODO: implement ToOwned
// TODO: force to have at least an unreleased or a release?
#[derive(Debug, Clone)]
pub struct Changelog<'source> {
    #[allow(dead_code)]
    source: &'source str,
    #[allow(dead_code)]
    title: Title,
    /// The unreleased section of a document is optional, as it would basically become empty
    /// after each release. So, whether the user decides to have one or not, is up to them.
    unreleased: Option<Unreleased>,
    releases: Vec<Release>,
    reference_definitions: Vec<ReferenceDefinition<'source>>,
}

impl<'source> Changelog<'source> {
    fn new(
        source: &'source str,
        title: Title,
        unreleased: Option<Unreleased>,
        releases: Vec<Release>,
        reference_definitions: Vec<ReferenceDefinition<'source>>,
    ) -> Self {
        Self {
            source,
            title,
            unreleased,
            releases,
            reference_definitions,
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
                // If we were able to construct at least one release or we have an
                // unreleased block, we may just be at the end, or reaching the ref defs.
                Err(_) if !releases.is_empty() || unreleased.is_some() => break,
                // TODO: maybe that's always an actual error, given that ref def aren't emitted as events.
                Err(err) => return Err(err.into()),
            }
        }
        let mut reference_definitions: Vec<_> = ast
            .reference_definitions()
            .iter()
            .map(|(k, v)| ReferenceDefinition::new(k.to_owned(), v.dest.clone(), v.span.clone()))
            .collect();
        reference_definitions.sort_unstable_by_key(|rd| rd.range().start);

        Ok(Changelog::new(
            source,
            title,
            unreleased,
            releases,
            reference_definitions,
        ))
    }

    pub fn unreleased(&self) -> &Option<Unreleased> {
        &self.unreleased
    }

    // TODO: store in Release/Unreleased as reference_definition() instead?
    pub fn reference_definitions(&self) -> &[ReferenceDefinition<'source>] {
        &self.reference_definitions
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
