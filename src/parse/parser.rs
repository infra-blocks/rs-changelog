use std::{collections::VecDeque, error::Error, fmt::Display};

use changelog_ast::{AstIterator, Node};

use crate::parse::{
    changelog::{Changelog, Title, TitleHeading},
    rules::{Rules, TitleRule},
};

// TODO: could try to just reverse the vec if the parsing always goes in the same direction instead.
pub(crate) type Unparsed<'source> = VecDeque<Node<'source>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Title(ParseTitleError),
}

impl From<ParseTitleError> for ParseError {
    fn from(err: ParseTitleError) -> Self {
        ParseError::Title(err)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while parsing changelog: ",)?;

        match self {
            ParseError::Title(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseTitleError {
    InvalidHeading,
    MissingContent,
}

impl Display for ParseTitleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseTitleError::InvalidHeading => write!(f, "invalid heading"),
            ParseTitleError::MissingContent => write!(f, "missing content"),
        }
    }
}

impl Error for ParseTitleError {}

pub struct ChangelogParser {
    pub(crate) rules: Rules,
}

impl ChangelogParser {
    pub fn new(rules: Rules) -> Self {
        Self { rules }
    }

    pub fn parse<'source>(&self, source: &'source str) -> Result<Changelog<'source>, ParseError> {
        let branches = AstIterator::new(source);

        let mut unparsed: VecDeque<_> = branches.collect();
        // This might not be addressed on the type itself because, unlike Changelog,
        // the function might not produce a title.
        let title = self.parse_title(&mut unparsed)?;

        Ok(Changelog { title })
    }

    pub(crate) fn parse_title<'source>(
        &self,
        unparsed: &mut Unparsed<'source>,
    ) -> Result<Option<Title<'source>>, ParseTitleError> {
        match self.rules.title {
            TitleRule::HeadingOnly => todo!(),
            TitleRule::HeadingAndText => {
                // The first node must match the title heading node.
                let title_heading = self
                    .parse_title_heading(unparsed)
                    .ok_or(ParseTitleError::InvalidHeading)?;
                let text_nodes = self.parse_title_text(unparsed);
                if text_nodes.is_empty() {
                    Err(ParseTitleError::MissingContent)
                } else {
                    Ok(Some(Title::new(title_heading, text_nodes)))
                }
            }
            TitleRule::None => todo!(),
        }
    }

    pub(crate) fn parse_title_heading<'source>(
        &self,
        unparsed: &mut Unparsed<'source>,
    ) -> Option<TitleHeading<'source>> {
        unparsed
            .pop_front_if(|node| TitleHeading::is_title_heading(node))
            // TODO: if the enum variants included the children and range directly, it would be easier
            // to convert through a TryFrom, for example.
            .map(|node| {
                let internal = node.unwrap_internal();
                let heading = internal.event.unwrap_heading();
                TitleHeading {
                    children: internal.children,
                    range: internal.range,
                    id: heading.id,
                    classes: heading.classes,
                    attrs: heading.attrs,
                }
            })
    }

    // TODO: no need to be on &self
    pub(crate) fn parse_title_text<'source>(
        &self,
        unparsed: &mut Unparsed<'source>,
    ) -> Vec<Node<'source>> {
        let mut result = vec![];
        while let Some(node) = unparsed.front()
            && !is_heading(node)
        {
            result.push(unparsed.pop_front().unwrap())
        }
        result
    }
}

pub(crate) fn is_heading(node: &Node<'_>) -> bool {
    node.is_internal_that(|internal| internal.event.is_heading())
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    mod parse_title {
        use super::*;

        mod heading_and_text_rule {
            use changelog_ast::{Internal, InternalEvent, Leaf, LeafEvent};
            use pulldown_cmark::CowStr;

            use super::*;

            macro_rules! assert_fails {
                ($source:expr, $error:expr) => {
                    let mut unparsed: VecDeque<_> = AstIterator::new($source).collect();
                    // TODO: TitleParser struct?
                    let parser = ChangelogParser::new(Rules::default());
                    let result = parser.parse_title(&mut unparsed);
                    assert_eq!(result, Err($error));
                };
            }

            #[test]
            fn should_error_for_empty_string() {
                assert_fails!("", ParseTitleError::InvalidHeading);
            }

            #[test]
            fn should_error_for_missing_heading() {
                assert_fails!(
                    "Just some text without the foreplay of the heading.",
                    ParseTitleError::InvalidHeading
                );
            }

            #[test]
            fn should_error_for_invalid_heading_size() {
                assert_fails!(
                    "## Changelog with invalid heading",
                    ParseTitleError::InvalidHeading
                );
            }

            #[test]
            fn should_error_for_missing_text_until_next_heading() {
                assert_fails!(
                    "# Changelog\n\n## Unreleased",
                    ParseTitleError::MissingContent
                );
            }

            #[test]
            fn should_succeed_for_a_valid_heading_and_one_paragraph() {
                let mut unparsed: VecDeque<_> =
                    AstIterator::new("# Changelog\nIpsum lorem stfu etc...").collect();
                let parser = ChangelogParser::new(Rules::default());
                let result = parser.parse_title(&mut unparsed);
                assert_eq!(
                    result,
                    Ok(Some(Title::new(
                        TitleHeading {
                            children: vec![Node::Leaf(Leaf {
                                event: LeafEvent::Text(CowStr::Borrowed("Changelog")),
                                range: 2..11
                            })],
                            range: 0..12,
                            id: None,
                            classes: Default::default(),
                            attrs: Default::default()
                        },
                        vec![Node::Internal(Internal {
                            event: InternalEvent::Paragraph,
                            range: 12..35,
                            children: vec![Node::Leaf(Leaf {
                                event: LeafEvent::Text(CowStr::Borrowed("Ipsum lorem stfu etc...")),
                                range: 12..35
                            })]
                        })]
                    )))
                );
            }
        }
    }
}
