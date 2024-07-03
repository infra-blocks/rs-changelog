use markdown::mdast::Node;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use crate::markdown::{parse_markdown, MarkdownNodeKind, MarkdownParseError};
use crate::node::TryFromNodes;
use crate::node::{Nodes, TryFromNode};
use crate::{
    ChangelogNode, ChangelogNodeKind, Description, Link, ParseError, Release, Title,
};

#[derive(Debug)]
pub struct Changelog {
    pub title: Title,
    pub description: Description,
    pub releases: Vec<Release>,
    pub links: Vec<Link>,
}

impl ChangelogNode for Changelog {
    fn node_kind() -> ChangelogNodeKind {
        ChangelogNodeKind::Changelog
    }
}

impl TryFromNode for Changelog {
    fn try_from_node(node: &Node) -> Result<Self, ParseError> {
        let Node::Root(root) = node else {
            return ParseError::invalid_node_type(MarkdownNodeKind::Root, &node).into();
        };

        let mut nodes: Nodes = root.children.as_slice().into();
        let title = Title::try_from_nodes(&mut nodes)?;
        let description = Description::try_from_nodes(&mut nodes)?;
        let releases = Vec::try_from_nodes(&mut nodes)?;
        let links = Vec::try_from_nodes(&mut nodes)?;

        if !nodes.is_empty() {
            return Err(ParseError::trailing_nodes(&nodes)
                .at_position(nodes.take_first()?.position().cloned().unwrap()));
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

#[derive(Debug)]
pub enum ChangelogParseError {
    InvalidMarkdown(MarkdownParseError),
    ParseError(ParseError),
}

impl Display for ChangelogParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangelogParseError::InvalidMarkdown(err) => write!(f, "{}", err),
            ChangelogParseError::ParseError(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ChangelogParseError {}

impl FromStr for Changelog {
    type Err = ChangelogParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Changelog::from_markdown(s)
    }
}

impl Changelog {
    pub fn from_markdown(markdown: &str) -> Result<Changelog, ChangelogParseError> {
        let tree =
            parse_markdown(markdown).map_err(|err| ChangelogParseError::InvalidMarkdown(err))?;
        Self::try_from_node(&tree).map_err(|err| ChangelogParseError::ParseError(err))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use date::NaiveDate;
        use semver::Version;

        #[test]
        fn should_work_with_valid_changelog() {
            let markdown = r#"
# Changelog

Here is the bullshit description.

## [1.0.0] - 2020-01-01

### Added

- Big release for y'all.

[1.0.0]: https://github.com/rs-changelog-tests/releases/tag/v0.1.0
"#;
            let changelog: Changelog = markdown.parse().unwrap();
            assert_eq!(changelog.title.text, "Changelog");
            assert_eq!(
                changelog.description.text,
                "Here is the bullshit description."
            );
            let releases = &changelog.releases;
            assert_eq!(releases.len(), 1);
            let release = &releases[0];
            assert_eq!(release.version, Version::new(1, 0, 0));
            assert_eq!(release.date, NaiveDate::from_ymd(2020, 1, 1).unwrap());
            let changes = &release.changes;
            let added = &changes.added.as_ref().unwrap().changes;
            assert_eq!(added.len(), 1);
            assert_eq!(added[0].text, "Big release for y'all.");
            assert!(changes.changed.is_none());
            assert!(changes.deprecated.is_none());
            assert!(changes.fixed.is_none());
            assert!(changes.removed.is_none());
            assert!(changes.security.is_none());
        }
    }
}
