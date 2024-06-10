use eyre::Error;

use crate::changelog;
use crate::changelog::parse::text::children_to_string;
use crate::changelog::parse::Position;

#[derive(Debug)]
pub struct Description {
    text: String,
    position: Option<Position>,
}

impl TryFrom<&changelog::markdown::Description> for Description {
    type Error = Error;

    fn try_from(
        markdown_description: &changelog::markdown::Description,
    ) -> Result<Self, Self::Error> {
        let markdown_node = &markdown_description.paragraph;
        let position = markdown_node
            .position
            .clone()
            .map(|position| position.into());

        let text = children_to_string(&markdown_node.children);
        Ok(Description { text, position })
    }
}
