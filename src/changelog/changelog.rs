use std::fmt::{Debug, Display};
use std::path::Path;

use date::NaiveDate;
use semver::Version;

use crate::changelog::{validate, ValidationOptions};
use clast;

#[derive(Debug, Clone, PartialEq)]
pub struct Changelog {
    title: String,
    description: String,
    unreleased: Option<ChangeSet>,
    releases: Vec<Release>,
    links: Vec<Link>,
}

impl Changelog {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn change_sets(&self) -> &[ChangeSet] {
        &self.change_sets
    }

    pub fn links(&self) -> &[Link] {
        &self.links
    }
}

impl From<clast::Changelog> for Changelog {
    fn from(value: clast::Changelog) -> Self {
        let title = value.title.text;
        let description = value.description.text;
        let releases = value.change_sets.into_iter().map(ChangeSet::from).collect();
        let links = value.links.into_iter().map(Link::from).collect();

        Changelog {
            title,
            description,
            change_sets: releases,
            links,
        }
    }
}

impl Changelog {
    pub fn try_from_file<T: AsRef<Path>>(file: T) -> Result<Changelog, ChangelogError> {
        Self::try_from_file_with_options(&file, ParseOptions::default())
    }

    pub fn try_from_file_with_options<T: AsRef<Path>>(
        file: &T,
        options: ParseOptions,
    ) -> Result<Changelog, ChangelogError> {
        let content = std::fs::read_to_string(file)?;

        Self::try_from_markdown_with_options(&content, options)
    }

    pub fn try_from_markdown(markdown: &str) -> Result<Changelog, ChangelogError> {
        Self::try_from_markdown_with_options(markdown, ParseOptions::default())
    }

    pub fn try_from_markdown_with_options(
        markdown: &str,
        options: ParseOptions,
    ) -> Result<Changelog, ChangelogError> {
        let parsed = clast::Changelog::from_markdown(markdown)?;
        validate(&parsed, options.validation_options)?;
        Ok(parsed.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Release {
    version: Version,
    date: NaiveDate,
    change_set: ChangeSet,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChangeSet {
    added: Vec<String>,
    changed: Vec<String>,
    deprecated: Vec<String>,
    removed: Vec<String>,
    fixed: Vec<String>,
    security: Vec<String>,
}

impl From<clast::ChangeSet> for ChangeSet {
    fn from(value: clast::ChangeSet) -> Self {
        ChangeSet {
            version: value.version,
            date: value.date,
            changes: value.changes.into(),
        }
    }
}

impl From<clast::Changes> for Changes {
    fn from(value: clast::Changes) -> Self {
        Changes {
            added: vec_from_change_set(value.added),
            changed: vec_from_change_set(value.changed),
            deprecated: vec_from_change_set(value.deprecated),
            removed: vec_from_change_set(value.removed),
            fixed: vec_from_change_set(value.fixed),
            security: vec_from_change_set(value.security),
        }
    }
}

fn vec_from_change_set(change_set: Option<clast::ChangeSubset>) -> Vec<String> {
    change_set
        .into_iter()
        .map(|change_set| change_set.changes)
        .flatten()
        .map(|change| change.text)
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    url: String,
    version: Version,
}

impl From<clast::ReleaseLink> for Link {
    fn from(value: clast::ReleaseLink) -> Self {
        Link {
            url: value.url,
            version: value.version,
        }
    }
}

#[derive(Debug)]
pub struct ParseOptions {
    validation_options: ValidationOptions,
}

impl Default for ParseOptions {
    fn default() -> Self {
        ParseOptions {
            validation_options: ValidationOptions::default(),
        }
    }
}

#[derive(Debug)]
pub enum ChangelogError {
    IoError(std::io::Error),
    ParseError(clast::ChangelogParseError),
    ValidationError(validate::ValidationError),
}

impl From<std::io::Error> for ChangelogError {
    fn from(value: std::io::Error) -> Self {
        ChangelogError::IoError(value)
    }
}

impl From<clast::ChangelogParseError> for ChangelogError {
    fn from(value: clast::ChangelogParseError) -> Self {
        ChangelogError::ParseError(value)
    }
}

impl From<validate::ValidationError> for ChangelogError {
    fn from(value: validate::ValidationError) -> Self {
        ChangelogError::ValidationError(value)
    }
}

impl Display for ChangelogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangelogError::IoError(err) => write!(f, "{}", err),
            ChangelogError::ParseError(err) => write!(f, "{}", err),
            ChangelogError::ValidationError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ChangelogError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ChangelogError::IoError(err) => err.source(),
            ChangelogError::ParseError(err) => err.source(),
            ChangelogError::ValidationError(err) => err.source(),
        }
    }
}
