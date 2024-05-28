use std::convert::TryFrom;

use eyre::Error;
use markdown::mdast::{Node, Paragraph};

#[derive(Debug)]
pub struct Description {
    paragraph: Paragraph,
}

impl TryFrom<&Node> for Description {
    type Error = Error;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        let Node::Paragraph(paragraph) = node else {
            return Err(eyre::eyre!("Description must be a heading"));
        };
        Ok(Description {
            paragraph: paragraph.clone(),
        })
    }
}
