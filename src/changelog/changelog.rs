use std::fmt::Display;
use std::str::FromStr;

use crate::changelog::{Description, Title};

use eyre::{eyre, Context, Error, Result};
use markdown::mdast::Node;
use markdown::{Constructs, ParseOptions};

#[derive(Debug)]
pub enum ChangelogParseError {
    FromMarkdown(markdown::message::Message),
    FromEyre(Error),
}

impl Display for ChangelogParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangelogParseError::FromMarkdown(err) => write!(f, "Error parsing markdown: {}", err),
            ChangelogParseError::FromEyre(err) => write!(f, "Error parsing changelog: {}", err),
        }
    }
}

impl std::error::Error for ChangelogParseError {}

#[derive(Debug)]
pub struct Changelog {
    title: Title,
    description: Description,
    releases: Vec<Release>,
}

impl FromStr for Changelog {
    type Err = ChangelogParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let ast = markdown::to_mdast(
            s,
            &ParseOptions {
                constructs: Constructs::gfm(),
                ..ParseOptions::default()
            },
        )
        .map_err(ChangelogParseError::FromMarkdown)?;
        Changelog::try_from(&ast).map_err(ChangelogParseError::FromEyre)
    }
}

impl TryFrom<&Node> for Changelog {
    type Error = Error;

    fn try_from(node: &Node) -> std::result::Result<Self, Self::Error> {
        let Node::Root(root) = node else {
            panic!("Attempting to parse changelog from a non-root node.")
        };

        let mut iter = root.children.iter();
        let title: Title = iter
            .next()
            .ok_or(eyre!("Changelog must have a title!"))?
            .try_into()?;
        let description: Description = iter
            .next()
            .ok_or(eyre!("Changelog must have a description!"))?
            .try_into()?;
        // TODO: make the description optional.
        Ok(Changelog { title, description })
    }
}

// TODO wrap externally visible errors. eyre::Error -> our own type, which will be pretty much the same.
impl Changelog {
    pub fn parse_from_file<T: Into<String>>(file: T) -> Result<Changelog> {
        let file = file.into();
        let content = std::fs::read_to_string(&file)
            .wrap_err(format!("Error reading changelog file '{}'", file))?;

        content.parse().wrap_err("Error parsing changelog")
    }
}
