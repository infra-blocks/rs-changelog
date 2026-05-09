use std::{error::Error, fmt::Display, ops::Range};

use crate::parse::parser::Unparsed;
pub use heading::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Title {
    heading: TitleHeading,
    text: Range<usize>,
}

impl Title {
    pub(crate) fn new(heading: TitleHeading, text: Range<usize>) -> Self {
        Self { heading, text }
    }

    pub(crate) fn parse(ast: &mut Unparsed) -> Result<Self, TitleParseError> {
        let heading = TitleHeading::parse(ast)?;

        let mut text_start = usize::MAX;
        let mut text_end = usize::MIN;
        while let Some(node) = ast.front()
            && !node.is_heading()
        {
            let range = node.range();
            if range.start < text_start {
                text_start = range.start;
            }
            if range.end > text_end {
                text_end = range.end;
            }
            ast.pop_front();
        }

        if text_start == usize::MAX {
            Err(TitleParseError::MissingContent)
        } else {
            Ok(Self::new(heading, text_start..text_end))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TitleParseError {
    InvalidHeading(TitleHeadingParseError),
    MissingContent,
}

impl Display for TitleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TitleParseError::MissingContent => write!(f, "missing content"),
            TitleParseError::InvalidHeading(_) => write!(f, "invalid node"),
        }
    }
}

impl Error for TitleParseError {}

impl From<TitleHeadingParseError> for TitleParseError {
    fn from(value: TitleHeadingParseError) -> Self {
        Self::InvalidHeading(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use std::collections::VecDeque;

        use changelog_ast::AstIterator;

        use super::*;

        #[test]
        fn should_error_for_invalid_heading() {
            let mut ast: VecDeque<_> =
                AstIterator::new("# Changestream\n\nHello. This is the changestream.").collect();
            let result = Title::parse(&mut ast);
            assert_eq!(
                result,
                Err(TitleParseError::InvalidHeading(
                    TitleHeadingParseError::InvalidText(2..14)
                ))
            );
        }

        #[test]
        fn should_error_for_missing_content() {
            let mut ast: VecDeque<_> = AstIterator::new("# Changelog\n\n\n").collect();
            let result = Title::parse(&mut ast);
            assert_eq!(result, Err(TitleParseError::MissingContent));
        }

        #[test]
        fn should_work_with_valid_title() {
            let mut ast: VecDeque<_> =
                AstIterator::new("# Changelog\n\nIpsum lorem stfu etc...\n\nContinued bro?")
                    .collect();
            let result = Title::parse(&mut ast);
            assert_eq!(result, Ok(Title::new(TitleHeading::new(0..12), 13..52)));
            assert!(ast.is_empty());
        }
    }
}

mod heading {
    use std::ops::Range;

    use changelog_ast::{HeadingLevel, Node};

    use crate::parse::{node_ext::NodeExt, parser::Unparsed};

    // This is guaranteed to be a heading of level 1.
    #[derive(Debug, Clone, PartialEq)]
    pub struct TitleHeading {
        range: Range<usize>,
    }

    impl TitleHeading {
        pub fn new(range: Range<usize>) -> Self {
            Self { range }
        }

        pub(crate) fn parse(ast: &mut Unparsed) -> Result<Self, TitleHeadingParseError> {
            let Some(first) = ast.front() else {
                return Err(TitleHeadingParseError::Empty);
            };

            if !first.is_heading_of_level(HeadingLevel::H1) {
                return Err(TitleHeadingParseError::InvalidNode(first.range().clone()));
            }

            match first {
                Node::Heading(heading) => {
                    let children = &heading.children;
                    if children.len() != 1 || !children[0].is_text_equals("Changelog") {
                        return Err(TitleHeadingParseError::InvalidText(
                            children[0].range().start..children[children.len() - 1].range().end,
                        ));
                    }

                    let range = first.range().clone();
                    ast.pop_front();
                    Ok(Self::new(range))
                }
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TitleHeadingParseError {
        Empty,
        InvalidNode(Range<usize>),
        InvalidText(Range<usize>),
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod parse {
            use std::collections::VecDeque;

            use changelog_ast::AstIterator;

            use super::*;

            macro_rules! failure {
                ($markdown:expr, $error:expr) => {
                    let mut ast: VecDeque<_> = AstIterator::new($markdown).collect();
                    let result = TitleHeading::parse(&mut ast);
                    assert_eq!(result, Err($error));
                };
            }

            #[test]
            fn should_error_with_empty_string() {
                failure!("", TitleHeadingParseError::Empty);
            }

            #[test]
            fn should_error_with_invalid_node() {
                failure!("some paracrap", TitleHeadingParseError::InvalidNode(0..13));
            }

            #[test]
            fn should_error_with_invalid_heading() {
                failure!("## Changelog", TitleHeadingParseError::InvalidNode(0..12));
            }

            #[test]
            fn should_error_with_invalid_text() {
                failure!("# Cuntlog", TitleHeadingParseError::InvalidText(2..9));
            }

            #[test]
            fn should_work_with_valid_title_heading() {
                let mut ast: VecDeque<_> = AstIterator::new("# Changelog").collect();
                let result = TitleHeading::parse(&mut ast);
                assert_eq!(result, Ok(TitleHeading::new(0..11)));
                assert!(ast.is_empty());
            }
        }
    }
}
