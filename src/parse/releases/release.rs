use std::ops::Range;

use chrono::NaiveDate;
use semver::Version;

use crate::parse::{
    ast::Ast,
    releases::{Changes, ChangesParseError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    heading: Range<usize>,
    version: Version,
    date: NaiveDate,
    changes: Changes,
}

impl Release {
    pub fn changes(&self) -> &Changes {
        &self.changes
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    pub(crate) fn parse(ast: &mut Ast) -> Result<Self, ReleaseParseError> {
        let (heading, version, date) = heading::parse(ast)?;
        let changes = Changes::parse(ast)?;

        Ok(Release::new(heading, version, date, changes))
    }

    fn new(heading: Range<usize>, version: Version, date: NaiveDate, changes: Changes) -> Self {
        Self {
            heading,
            version,
            date,
            changes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseParseError {
    InvalidHeading(heading::ParseError),
    InvalidChanges(ChangesParseError),
}

impl From<heading::ParseError> for ReleaseParseError {
    fn from(value: heading::ParseError) -> Self {
        Self::InvalidHeading(value)
    }
}

impl From<ChangesParseError> for ReleaseParseError {
    fn from(value: ChangesParseError) -> Self {
        Self::InvalidChanges(value)
    }
}

mod heading {
    use std::ops::Range;

    use changelog_ast::{Heading, HeadingLevel, Link, LinkType, Node, Text};
    use chrono::NaiveDate;
    use semver::Version;

    use crate::parse::{ast::Ast, node_ext::NodeExt};

    pub fn parse(ast: &mut Ast) -> Result<(Range<usize>, Version, NaiveDate), ParseError> {
        // The first node has to be a heading.
        let Some(first) = ast.front() else {
            return Err(ParseError::Empty);
        };

        let (version, date) = match first {
            Node::Heading(Heading {
                range: _,
                children,
                level: HeadingLevel::H2,
                id: _,
                classes: _,
                attrs: _,
            }) if children.len() >= 2 => parse_heading_children(children)?,
            _ => {
                return Err(ParseError::InvalidHeading(first.range().clone()));
            }
        };
        let first = ast.pop_front().unwrap().unwrap_heading();
        Ok((first.range, version, date))
    }

    fn parse_heading_children(nodes: &[Node<'_>]) -> Result<(Version, NaiveDate), ParseError> {
        // When the version is properly linked, the heading should have 2 children: a link and a text
        // event with the date following.
        if nodes.len() == 2 {
            let version = parse_version(&nodes[0])?;
            let date = parse_date(&nodes[1])?;
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
            return Err(ParseError::BrokenLink(
                nodes[0].range().start..nodes[2].range().end,
            ));
        }

        Err(ParseError::InvalidText(
            nodes[0].range().start..nodes[nodes.len() - 1].range().end,
        ))
    }

    // TODO: test with prerelease and build info, and enforce they fail.
    fn parse_version<'source>(node: &Node<'source>) -> Result<Version, ParseError> {
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
            }) => {
                Ok(Version::parse(id)
                    .map_err(|_| ParseError::InvalidVersion(node.range().clone()))?)
            }
            _ => Err(ParseError::InvalidVersion(node.range().clone())),
        }
    }

    fn parse_date<'source>(node: &Node<'source>) -> Result<NaiveDate, ParseError> {
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
                    .map_err(|_| ParseError::InvalidDate(range.clone()))?;
                Ok(parsed)
            }
            _ => Err(ParseError::InvalidDate(node.range().clone())),
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ParseError {
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
                    let result = parse(&mut ast);
                    assert_eq!(result, Err($error));
                };
            }

            #[test]
            fn should_error_with_empty_string() {
                failure!("", ParseError::Empty);
            }

            #[test]
            fn should_error_with_invalid_heading() {
                failure!(
                    "# [0.1.0] - 2024-05-01\n[0.1.0]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
                    ParseError::InvalidHeading(0..23)
                );
            }

            #[test]
            fn should_error_with_broken_link() {
                failure!("## [0.1.0] - 2024-05-01", ParseError::BrokenLink(3..10));
            }

            #[test]
            fn should_error_with_invalid_version() {
                failure!(
                    "## [click-clack] - 2024-05-01\n\n[click-clack]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
                    ParseError::InvalidVersion(3..16)
                );
            }

            #[test]
            fn should_error_with_invalid_date() {
                failure!(
                    "## [0.1.0] - 01-05-2024\n\n[0.1.0]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
                    ParseError::InvalidDate(10..23)
                );
            }

            #[test]
            fn should_work_with_valid_info() {
                let mut ast = Ast::from(
                    "## [0.1.0] - 2024-05-01\n\n[0.1.0]: https://github.com/yo-mama/azz/releases/tag/v0.1.0",
                );
                let result = parse(&mut ast);
                assert_eq!(
                    result,
                    Ok((
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
                let result = parse(&mut ast);
                assert_eq!(
                    result,
                    Ok((
                        0..24,
                        Version::new(0, 1, 0),
                        NaiveDate::from_ymd_opt(2024, 5, 1).unwrap()
                    ))
                );
                assert!(ast.is_empty());
            }
        }
    }
}
