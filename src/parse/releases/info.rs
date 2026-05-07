use changelog_ast::{Link, Node, Text};
use chrono::NaiveDate;
use pulldown_cmark::LinkType;
use semver::Version;

use crate::parse::node_ext::NodeExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseInfoParseError {
    BrokenLink,
    InvalidFormat,
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
        // dbg!(&nodes);

        // When the version is properly linked, the heading should have 2 children: a link and a text
        // event with the date following.
        if nodes.len() == 2
            && let Ok(version) = Self::parse_version(&nodes[0])
            && let Ok(date) = Self::parse_date(&nodes[1])
        {
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
            return Err(ReleaseInfoParseError::BrokenLink);
        }

        Err(ReleaseInfoParseError::InvalidFormat)
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
            }) => Ok(Version::parse(id)?),
            _ => Err(VersionParseError::InvalidNode),
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
            Node::Text(Text { range: _, text }) => {
                Ok(NaiveDate::parse_from_str(text, " - %Y-%m-%d")?)
            }
            _ => Err(DateParseError::InvalidNode),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionParseError {
    InvalidNode,
    InvalidSemver,
}

impl From<semver::Error> for VersionParseError {
    fn from(_: semver::Error) -> Self {
        Self::InvalidSemver
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateParseError {
    InvalidNode,
    InvalidFormat,
}

impl From<chrono::ParseError> for DateParseError {
    fn from(_: chrono::ParseError) -> Self {
        Self::InvalidFormat
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use changelog_ast::AstIterator;

        use super::*;

        #[test]
        fn should_error_with_broken_link() {
            let mut nodes: Vec<_> = AstIterator::new("[0.1.0] - 2024-05-01").collect();
            // We expect a single top-level paragraph node that has the children we are
            // interested in.
            assert!(nodes.len() == 1);
            let paragraph = nodes.pop().unwrap().unwrap_paragraph();
            let result = ReleaseInfo::parse(&paragraph.children);
            assert_eq!(result, Err(ReleaseInfoParseError::BrokenLink));
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
            assert_eq!(result, Err(ReleaseInfoParseError::InvalidFormat));
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
            assert_eq!(result, Err(ReleaseInfoParseError::InvalidFormat));
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
