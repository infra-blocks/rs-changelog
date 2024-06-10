use std::fmt::Display;

use eyre::Error;
use markdown::mdast::Node;

use crate::changelog::error::ChangelogParseError;
use crate::changelog::markdown::from_slice::TryFromSlice;
use crate::changelog::markdown::{
    trailing_nodes_error, Description, Link, MarkdownError, Release, Title,
};
use crate::changelog::position::Position;

// TODO: rename and expose a single error type for the interface of this module.
#[derive(Debug)]
pub enum MarkdownParseError {
    FromMarkdown(markdown::message::Message),
    FromEyre(Error),
}

impl Display for MarkdownParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkdownParseError::FromMarkdown(err) => write!(f, "error parsing markdown: {}", err),
            MarkdownParseError::FromEyre(err) => write!(f, "error parsing changelog: {}", err),
        }
    }
}

impl std::error::Error for MarkdownParseError {}

#[derive(Debug)]
pub struct Changelog {
    pub title: Title,
    pub description: Description,
    pub releases: Vec<Release>,
    pub links: Vec<Link>,
}

impl TryFrom<&Node> for Changelog {
    type Error = ChangelogParseError<MarkdownError>;

    fn try_from(node: &Node) -> std::result::Result<Self, Self::Error> {
        let Node::Root(root) = node else {
            panic!("Attempting to parse changelog from a non-root node.")
        };

        let (slice, title) = Title::try_from_slice(&root.children)?;
        let (slice, description) = Description::try_from_slice(slice)?;
        let (slice, releases) = Vec::try_from_slice(slice)?;
        let (slice, links) = Vec::try_from_slice(slice)?;

        if !slice.is_empty() {
            return Err(trailing_nodes_error(
                slice[0].position().map(Position::from),
                format!("{:?}", slice),
            ));
        }

        let changelog = Changelog {
            title,
            description,
            releases,
            links,
        };
        Ok(changelog)
    }
}
