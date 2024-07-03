use markdown::mdast::Node;
use semver::Version;

use crate::markdown::NodeUtils;
use crate::node::TryFromNode;
use crate::position::Position;
use crate::text::parse_version;
use crate::{ChangelogNode, ChangelogNodeKind, ParseError};

#[derive(Debug, Clone)]
pub struct Link {
    pub position: Position,
    pub url: String,
    pub version: Version,
}

impl ChangelogNode for Link {
    fn node_kind() -> ChangelogNodeKind {
        ChangelogNodeKind::Link
    }
}

impl TryFromNode for Link {
    fn try_from_node(node: &Node) -> Result<Self, ParseError> {
        let link = node
            .validate_definition()
            .map_err(|err| err.at_position(node.unwrap_position()))?;
        let position = node.unwrap_position();
        let url = link.url.clone();
        let version = parse_version(&link.identifier)
            .map_err(|err| err.at_position(node.unwrap_position()))?;
        Ok(Link {
            position,
            url,
            version,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from_nodes {
        use super::*;
        use test_utils::{
            definition_node, fails_for_empty_nodes, fails_for_invalid_text, fails_for_wrong_node,
            list_node, works_with_valid_node,
        };

        fails_for_empty_nodes!(Link);
        fails_for_wrong_node!(Link, list_node(()));
        fails_for_invalid_text!(Link, definition_node(("[x.y.z]", "https://stfu.com")));
        works_with_valid_node!(
            Link,
            definition_node(("[1.2.3]", "https://stfu.com")),
            |effective: Link, node: &Node| {
                assert_eq!(effective.position, node.unwrap_position());
                assert_eq!(effective.url, "https://stfu.com");
                assert_eq!(effective.version, Version::new(1, 2, 3));
            }
        );
    }
}
