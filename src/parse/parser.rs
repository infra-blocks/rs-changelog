use std::{collections::VecDeque, error::Error, fmt::Display, ops::Range};

use changelog_ast::{AstIterator, HeadingLevel, Node};

use crate::parse::{
    changelog::{Changelog, Title, TitleHeading},
    node_ext::NodeExt,
    releases::{ChangesParseError, Releases, ReleasesParseError, Unreleased, UnreleasedParseError},
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TitleParseError {
    /// Happens when not enough nodes are present to successfully parse an item.
    Empty,
    /// Happens when the markdown node received does not match expectations.
    ///
    /// It could happen because it is not a [Node::Heading], or because its
    /// [HeadingLevel] is not 1.
    InvalidNode(Range<usize>),
    MissingContent,
}

impl Display for TitleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TitleParseError::Empty => write!(f, "node nodes to parse"),
            TitleParseError::InvalidNode(_) => write!(f, "invalid heading"),
            TitleParseError::MissingContent => write!(f, "missing content"),
        }
    }
}

impl Error for TitleParseError {}

pub struct ChangelogParser {}

impl ChangelogParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse<'source>(&self, source: &'source str) -> Result<Changelog<'source>, ParseError> {
        let branches = AstIterator::new(source);

        let mut unparsed: VecDeque<_> = branches.collect();
        // This might not be addressed on the type itself because, unlike Changelog,
        // the function might not produce a title.
        let title = self.parse_title(&mut unparsed)?;
        let unreleased = match Unreleased::parse(&mut unparsed) {
            Ok(unreleased) => Some(unreleased),
            Err(err) => match err {
                UnreleasedParseError::InvalidHeading(_) => None,
                UnreleasedParseError::InvalidChanges(err) => {
                    return Err(ParseError::InvalidUnreleased(err));
                }
            },
        };
        let releases = Releases::parse(&mut unparsed)?;

        Ok(Changelog {
            title,
            unreleased,
            releases,
        })
    }

    // TODO: Title::parse
    pub(crate) fn parse_title<'source>(
        &self,
        ast: &mut Unparsed<'source>,
    ) -> Result<Title<'source>, TitleParseError> {
        // The first node must match the title heading node.
        let title_heading = self.parse_title_heading(ast)?;
        let text_nodes = self.parse_title_text(ast)?;
        Ok(Title::new(title_heading, text_nodes))
    }

    pub(crate) fn parse_title_heading<'source>(
        &self,
        ast: &mut Unparsed<'source>,
    ) -> Result<TitleHeading<'source>, TitleParseError> {
        let Some(first) = ast.front() else {
            return Err(TitleParseError::Empty);
        };

        if !first.is_heading_of_level(HeadingLevel::H1) {
            return Err(TitleParseError::InvalidNode(first.range().clone()));
        }

        // Safe to pop at this point.
        let heading = ast.pop_front().unwrap().unwrap_heading();
        Ok(TitleHeading::new(heading.range, heading.children))
    }

    pub(crate) fn parse_title_text<'source>(
        &self,
        ast: &mut Unparsed<'source>,
    ) -> Result<Vec<Node<'source>>, TitleParseError> {
        let mut result = vec![];
        while let Some(node) = ast.front()
            && !node.is_heading()
        {
            result.push(ast.pop_front().unwrap())
        }

        if result.is_empty() {
            Err(TitleParseError::MissingContent)
        } else {
            Ok(result)
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    mod parse_title {
        use super::*;

        use changelog_ast::{CowStr, Paragraph, Text};

        macro_rules! assert_fails {
            ($source:expr, $error:expr) => {
                let mut unparsed: VecDeque<_> = AstIterator::new($source).collect();
                // TODO: TitleParser struct?
                let parser = ChangelogParser::new();
                let result = parser.parse_title(&mut unparsed);
                assert_eq!(result, Err($error));
            };
        }

        #[test]
        fn should_error_for_empty_string() {
            assert_fails!("", TitleParseError::Empty);
        }

        #[test]
        fn should_error_for_missing_heading() {
            assert_fails!(
                "Just some text without the foreplay of the heading.",
                TitleParseError::InvalidNode(0..51)
            );
        }

        #[test]
        fn should_error_for_invalid_heading_size() {
            assert_fails!(
                "## Changelog with invalid heading",
                TitleParseError::InvalidNode(0..33)
            );
        }

        #[test]
        fn should_error_for_missing_text_until_next_heading() {
            assert_fails!(
                "# Changelog\n\n## Unreleased",
                TitleParseError::MissingContent
            );
        }

        #[test]
        fn should_succeed_for_a_valid_heading_and_one_paragraph() {
            let mut unparsed: VecDeque<_> =
                AstIterator::new("# Changelog\nIpsum lorem stfu etc...").collect();
            let parser = ChangelogParser::new();
            let result = parser.parse_title(&mut unparsed);
            assert_eq!(
                result,
                Ok(Title::new(
                    TitleHeading {
                        children: vec![Node::Text(Text::new(2..11, CowStr::Borrowed("Changelog")))],
                        range: 0..12,
                    },
                    vec![Node::Paragraph(Paragraph::new(
                        12..35,
                        vec![Node::Text(Text::new(
                            12..35,
                            CowStr::Borrowed("Ipsum lorem stfu etc...")
                        ))],
                    ))],
                ))
            );
        }
    }
}
