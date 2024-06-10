use crate::changelog;

use crate::changelog::parse::Position;

use eyre::Error;

#[derive(Debug)]
pub struct Link {
    pub position: Option<Position>,
    pub url: String,
    pub anchor: String,
}

impl TryFrom<&changelog::markdown::Link> for Link {
    type Error = Error;

    fn try_from(value: &changelog::markdown::Link) -> Result<Self, Self::Error> {
        let markdown_node = &value.definition;
        let position = markdown_node
            .position
            .clone()
            .map(|position| position.into());

        let url = markdown_node.url.clone();
        let anchor = markdown_node.identifier.clone();

        Ok(Link {
            position,
            url,
            anchor,
        })
    }
}
