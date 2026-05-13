use std::ops::Range;

use changelog_ast::{Heading, HeadingLevel, Link, LinkType, Node, Text};
use chrono::NaiveDate;
use semver::Version;

use crate::parse::{ast::Ast, node_ext::NodeExt};

// TODO: rename file as we now have a unreleased heading mafock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseHeading {
    range: Range<usize>,
    version: Version,
    date: NaiveDate,
}

impl ReleaseHeading {
    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    pub(crate) fn new(range: Range<usize>, version: Version, date: NaiveDate) -> Self {
        Self {
            range,
            version,
            date,
        }
    }

    pub(crate) fn parse(ast: &mut Ast) -> Result<ReleaseHeading, ReleaseHeadingParseError> {
        // The first node has to be a heading.
        let Some(first) = ast.front() else {
            return Err(ReleaseHeadingParseError::Empty);
        };

        let (version, date) = match first {
            Node::Heading(Heading {
                range: _,
                children,
                level: HeadingLevel::H2,
                id: _,
                classes: _,
                attrs: _,
            }) if children.len() >= 2 => Self::parse_heading_children(children)?,
            _ => {
                return Err(ReleaseHeadingParseError::InvalidHeading(
                    first.range().clone(),
                ));
            }
        };
        let first = ast.pop_front().unwrap().unwrap_heading();
        Ok(ReleaseHeading::new(first.range, version, date))
    }

    pub(crate) fn parse_heading_children(
        nodes: &[Node<'_>],
    ) -> Result<(Version, NaiveDate), ReleaseHeadingParseError> {
        // When the version is properly linked, the heading should have 2 children: a link and a text
        // event with the date following.
        if nodes.len() == 2 {
            let version = Self::parse_version(&nodes[0])?;
            let date = Self::parse_date(&nodes[1])?;
            return Ok((version, date));
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
            return Err(ReleaseHeadingParseError::BrokenLink(
                nodes[0].range().start..nodes[2].range().end,
            ));
        }

        Err(ReleaseHeadingParseError::InvalidText(
            nodes[0].range().start..nodes[nodes.len() - 1].range().end,
        ))
    }

    fn parse_version<'source>(node: &Node<'source>) -> Result<Version, ReleaseHeadingParseError> {
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
                .map_err(|_| ReleaseHeadingParseError::InvalidVersion(node.range().clone()))?),
            _ => Err(ReleaseHeadingParseError::InvalidVersion(
                node.range().clone(),
            )),
        }
    }

    fn parse_date<'source>(node: &Node<'source>) -> Result<NaiveDate, ReleaseHeadingParseError> {
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
            Node::Text(Text { range, text }) => {
                let parsed = NaiveDate::parse_from_str(text, " - %Y-%m-%d")
                    .map_err(|_| ReleaseHeadingParseError::InvalidDate(range.clone()))?;
                Ok(parsed)
            }
            _ => Err(ReleaseHeadingParseError::InvalidDate(node.range().clone())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseHeadingParseError {
    BrokenLink(Range<usize>),
    Empty,
    InvalidDate(Range<usize>),
    InvalidHeading(Range<usize>),
    // TODO: invalid text might be useless here. It could just be invalid version instead?
    InvalidText(Range<usize>),
    InvalidVersion(Range<usize>),
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use chrono::NaiveDate;
        use semver::Version;

        use super::*;

        macro_rules! failure {
            ($markdown:expr,$error:expr) => {
                let mut ast = Ast::from($markdown);
                let result = ReleaseHeading::parse(&mut ast);
                assert_eq!(result, Err($error));
            };
        }

        #[test]
        fn should_error_with_empty_string() {
            failure!("", ReleaseHeadingParseError::Empty);
        }

        #[test]
        fn should_error_with_invalid_heading() {
            failure!(
                "# [0.1.0] - 2024-05-01\n[0.1.0]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
                ReleaseHeadingParseError::InvalidHeading(0..23)
            );
        }

        #[test]
        fn should_error_with_broken_link() {
            failure!(
                "## [0.1.0] - 2024-05-01",
                ReleaseHeadingParseError::BrokenLink(3..10)
            );
        }

        #[test]
        fn should_error_with_invalid_version() {
            failure!(
                "## [click-clack] - 2024-05-01\n\n[click-clack]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
                ReleaseHeadingParseError::InvalidVersion(3..16)
            );
        }

        #[test]
        fn should_error_with_invalid_date() {
            failure!(
                "## [0.1.0] - 01-05-2024\n\n[0.1.0]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
                ReleaseHeadingParseError::InvalidDate(10..23)
            );
        }

        #[test]
        fn should_work_with_valid_info() {
            let mut ast = Ast::from(
                "## [0.1.0] - 2024-05-01\n\n[0.1.0]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
            );
            let result = ReleaseHeading::parse(&mut ast);
            assert_eq!(
                result,
                Ok(ReleaseHeading::new(
                    0..24,
                    Version::new(0, 1, 0),
                    NaiveDate::from_ymd_opt(2024, 5, 1).unwrap()
                ))
            );
        }

        #[test]
        fn should_work_with_valid_release() {
            let mut ast = Ast::from(
                "## [0.1.0] - 2024-05-01\n[0.1.0]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
            );
            let result = ReleaseHeading::parse(&mut ast);
            assert_eq!(
                result,
                Ok(ReleaseHeading::new(
                    0..24,
                    Version::new(0, 1, 0),
                    NaiveDate::from_ymd_opt(2024, 5, 1).unwrap()
                ))
            );
            assert!(ast.is_empty());
        }
    }
}
