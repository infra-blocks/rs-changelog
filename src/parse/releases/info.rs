use std::ops::Range;

use changelog_ast::{Link, Node, Text};
use chrono::NaiveDate;
use pulldown_cmark::LinkType;
use semver::Version;

use crate::parse::node_ext::NodeExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseInfoParseError {
    Empty,
    BrokenLink(Range<usize>),
    InvalidNodes(Range<usize>),
    InvalidVersion(VersionParseError),
    InvalidDate(DateParseError),
}

impl From<VersionParseError> for ReleaseInfoParseError {
    fn from(value: VersionParseError) -> Self {
        Self::InvalidVersion(value)
    }
}

impl From<DateParseError> for ReleaseInfoParseError {
    fn from(value: DateParseError) -> Self {
        Self::InvalidDate(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseInfo {
    version: Version,
    date: NaiveDate,
}

impl ReleaseInfo {
    pub fn new(version: Version, date: NaiveDate) -> Self {
        Self { version, date }
    }

    pub(crate) fn parse<'source>(
        // Those are expected to be the children of the title heading.
        nodes: &[Node<'source>],
    ) -> Result<Self, ReleaseInfoParseError> {
        if nodes.len() < 2 {
            return Err(ReleaseInfoParseError::Empty);
        }

        // When the version is properly linked, the heading should have 2 children: a link and a text
        // event with the date following.
        if nodes.len() == 2 {
            let version = Self::parse_version(&nodes[0])?;
            let date = Self::parse_date(&nodes[1])?;
            return Ok(Self::new(version, date));
        }

        // If the link is broken, then the child will be split into 4 fragments, one for '[', one for
        // the version, one for ']', and finally one for the rest.
        // Example case where the node describes a broken link:
        // Text(
        //         Text {
        //             range: 3..4,
        //             text: Borrowed(
        //                 "[",
        //             ),
        //         },
        //     ),
        //     Text(
        //         Text {
        //             range: 4..9,
        //             text: Borrowed(
        //                 "0.1.0",
        //             ),
        //         },
        //     ),
        //     Text(
        //         Text {
        //             range: 9..10,
        //             text: Borrowed(
        //                 "]",
        //             ),
        //         },
        //     ),
        //     Text(
        //         Text {
        //             range: 10..23,
        //             text: Borrowed(
        //                 " - 2024-05-01",
        //             ),
        //         },
        //     ),
        if nodes.len() >= 3 && nodes[0].is_text_equals("[") && nodes[2].is_text_equals("]") {
            return Err(ReleaseInfoParseError::BrokenLink(
                nodes[0].range().start..nodes[2].range().end,
            ));
        }

        Err(ReleaseInfoParseError::InvalidNodes(
            nodes[0].range().start..nodes[nodes.len() - 1].range().end,
        ))
    }

    fn parse_version<'source>(node: &Node<'source>) -> Result<Version, VersionParseError> {
        // Example valid node:
        // Link(
        //     Link {
        //         range: 3..10,
        //         children: [
        //             Text(
        //                 Text {
        //                     range: 4..9,
        //                     text: Borrowed(
        //                         "0.1.0",
        //                     ),
        //                 },
        //             ),
        //         ],
        //         dest_url: Borrowed(
        //             "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0",
        //         ),
        //         id: Borrowed(
        //             "0.1.0",
        //         ),
        //         link_type: Shortcut,
        //         title: Borrowed(
        //             "",
        //         ),
        //     },
        // ),
        // Text(
        //     Text {
        //         range: 10..23,
        //         text: Borrowed(
        //             " - 2024-05-01",
        //         ),
        //     },
        // ),
        match node {
            Node::Link(Link {
                range: _,
                children: _,
                dest_url: _,
                // Because it's a shortcut link, the ID should match the text. It should match the version.
                id,
                link_type: LinkType::Shortcut,
                title: _,
            }) => Ok(Version::parse(id)
                .map_err(|_| VersionParseError::InvalidSemver(node.range().clone()))?),
            _ => Err(VersionParseError::InvalidNode(node.range().clone())),
        }
    }

    fn parse_date<'source>(node: &Node<'source>) -> Result<NaiveDate, DateParseError> {
        // Example valid node:
        // Text(
        //     Text {
        //         range: 10..23,
        //         text: Borrowed(
        //             " - 2024-05-01",
        //         ),
        //     },
        // ),
        match node {
            Node::Text(Text { range, text }) => Ok(NaiveDate::parse_from_str(text, " - %Y-%m-%d")
                .map_err(|_| DateParseError::InvalidFormat(range.clone()))?),
            _ => Err(DateParseError::InvalidNode(node.range().clone())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionParseError {
    /// Happens when the provided node does not match the expected markdown node kind.
    InvalidNode(Range<usize>),
    /// Happens when the textual content of the node cannot be parsed into a valid [semver::Version]
    InvalidSemver(Range<usize>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateParseError {
    /// Happens when the provided node does not match the expected markdown node kind.
    InvalidNode(Range<usize>),
    /// Happens when the textual content cannot be parsed into a date as it does not match expectations.
    InvalidFormat(Range<usize>),
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use changelog_ast::AstIterator;

        use super::*;

        #[test]
        fn should_error_with_empty_string() {
            let nodes: Vec<_> = AstIterator::new("").collect();
            let result = ReleaseInfo::parse(&nodes);
            assert_eq!(result, Err(ReleaseInfoParseError::Empty));
        }

        #[test]
        fn should_error_with_broken_link() {
            let mut nodes: Vec<_> = AstIterator::new("[0.1.0] - 2024-05-01").collect();
            // We expect a single top-level paragraph node that has the children we are
            // interested in.
            assert!(nodes.len() == 1);
            let paragraph = nodes.pop().unwrap().unwrap_paragraph();
            let result = ReleaseInfo::parse(&paragraph.children);
            assert_eq!(result, Err(ReleaseInfoParseError::BrokenLink(0..7)));
        }

        #[test]
        fn should_error_with_invalid_version() {
            let mut nodes: Vec<_> = AstIterator::new(
                "[click-clack] - 2024-05-01\n\n[click-clack]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
            )
            .collect();
            assert!(nodes.len() == 1);
            let paragraph = nodes.pop().unwrap().unwrap_paragraph();
            let result = ReleaseInfo::parse(&paragraph.children);
            assert_eq!(
                result,
                Err(ReleaseInfoParseError::InvalidVersion(
                    VersionParseError::InvalidSemver(0..13)
                ))
            );
        }

        #[test]
        fn should_error_with_invalid_date() {
            let mut nodes: Vec<_> = AstIterator::new(
                "[0.1.0] - 01-05-2024\n\n[0.1.0]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
            )
            .collect();
            assert!(nodes.len() == 1);
            let paragraph = nodes.pop().unwrap().unwrap_paragraph();
            let result = ReleaseInfo::parse(&paragraph.children);
            assert_eq!(
                result,
                Err(ReleaseInfoParseError::InvalidDate(
                    DateParseError::InvalidFormat(7..20)
                ))
            );
        }

        #[test]
        fn should_work_with_valid_info() {
            let mut nodes: Vec<_> = AstIterator::new(
                "[0.1.0] - 2024-05-01\n\n[0.1.0]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
            )
            .collect();
            assert!(nodes.len() == 1);
            let paragraph = nodes.pop().unwrap().unwrap_paragraph();
            let result = ReleaseInfo::parse(&paragraph.children);
            assert_eq!(
                result,
                Ok(ReleaseInfo {
                    version: Version::new(0, 1, 0),
                    date: NaiveDate::from_ymd_opt(2024, 5, 1).unwrap()
                })
            );
        }
    }
}
