use std::{collections::VecDeque, marker::PhantomData, ops::Range};

use changelog_ast::{Ast, Heading, Internal, InternalEvent, Node};

use crate::lint::rules::Rules;

pub type Remaining<'source> = VecDeque<Node<'source>>;

pub struct Title<'source> {
    // TODO: going to map from a heading node with children and shit.
    // TODO: maybe type it even further to a heading struct.
    heading: Heading<'source>,
    text: Option<Vec<Node<'source>>>,
    range: Range<usize>,
    children: Vec<Node<'source>>,
}

impl<'source> Title<'source> {
    pub fn new(
        heading: Heading<'source>,
        range: Range<usize>,
        children: Vec<Node<'source>>,
    ) -> Self {
        Self {
            heading,
            range,
            children,
        }
    }
}

pub enum TitleParseError {
    MissingTitle,
    Invalid,
}

impl<'source> Validator<'source> for Title<'source> {
    type Output = Option<Title<'source>>;
    // TODO: error type
    type Error = TitleParseError;

    fn validate(
        remaining: &mut Remaining<'source>,
        rules: &Rules,
    ) -> Result<Option<Title<'source>>, TitleParseError> {
        let success = matches!(
            remaining.front(),
            Some(Node::Internal(Internal {
                event: InternalEvent::Heading(Heading {
                    level: HeadingLevel::H1,
                    id: _,
                    classes: _,
                    attrs: _,
                }),
                range: _,
                children: _,
            }))
        );

        if !success {
            if remaining.front().is_none() {
                // TODO: those can probably be wrapped in a function.
                if rules.title {
                    return Err(TitleParseError::MissingTitle);
                } else {
                    return Ok(None);
                }
            } else {
                if rules.title {
                    return Err(TitleParseError::Invalid);
                } else {
                    return Ok(None);
                }
            }
        }

        // If we make it this far, we know we have a proper title heading.
        let first = remaining.pop_front().unwrap();
        let Internal {
            event,
            range,
            children,
        } = first.unwrap_internal();
        let heading = event.unwrap_heading();
        Ok(Some(Title::new(heading, range, children)))
        // TODO: validate it's followed by a paragraph yo?
    }
}
