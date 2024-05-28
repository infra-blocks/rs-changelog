use eyre::Error;
use markdown::mdast::{Heading, Node};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Title {
    heading: Heading,
}

impl TryFrom<&Node> for Title {
    type Error = Error;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        let Node::Heading(title_heading) = node else {
            return Err(eyre::eyre!("Title must be a heading"));
        };
        if title_heading.depth != 1 {
            return Err(eyre::eyre!("Title must be a level 1 heading"));
        }
        Ok(Title {
            heading: title_heading.clone(),
        })
    }
}
