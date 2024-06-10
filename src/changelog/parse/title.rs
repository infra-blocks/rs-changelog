use eyre::Error;

use crate::changelog;
use crate::changelog::parse::text::children_to_string;
use crate::changelog::parse::Position;

#[derive(Debug)]
pub struct Title {
    text: String,
    position: Option<Position>,
}

impl TryFrom<&changelog::markdown::Title> for Title {
    type Error = Error;

    fn try_from(markdown_title: &changelog::markdown::Title) -> Result<Self, Self::Error> {
        let markdown_node = &markdown_title.heading;
        let position = markdown_node
            .position
            .clone()
            .map(|position| position.into());

        let text = children_to_string(&markdown_node.children);
        Ok(Title { text, position })
    }
}
