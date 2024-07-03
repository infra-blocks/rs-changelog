use date::NaiveDate;
use semver::Version;

use crate::markdown::NodeUtils;
use crate::node::{Nodes, TryFromNodes};
use crate::{ChangelogNode, ChangelogNodeKind, ParseError};
use crate::{Changes, Position};

// TODO: support [Unreleased] in the changelog.
#[derive(Debug, Clone)]
pub struct Release {
    pub version: Version,
    pub date: NaiveDate,
    pub position: Position,
    pub changes: Changes,
}

impl ChangelogNode for Release {
    fn node_kind() -> ChangelogNodeKind {
        ChangelogNodeKind::Release
    }
}

type ParsedText = (Version, NaiveDate);

fn parse_version(text: &str) -> Result<Version, ParseError> {
    Version::parse(&text.replace(&['[', ']'], ""))
        .map_err(|err| ParseError::invalid_text(text, "unable to parse version").with_source(err))
}

fn parse_date(text: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::from_ymd_str(text)
        .ok_or_else(|| ParseError::invalid_text(text, "unable to parse date"))
}

impl Release {
    fn parse_text(text: &str) -> Result<ParsedText, ParseError> {
        let tokens = text.split_once("-");

        match tokens {
            Some((left, right)) => {
                let left = left.trim();
                let right = right.trim();
                let version = parse_version(left)?;
                let date = parse_date(right)?;
                Ok((version, date))
            }
            None => ParseError::invalid_text(text, "expected to be able to split around '-' once")
                .into(),
        }
    }
}

impl TryFromNodes for Release {
    fn try_from_nodes(nodes: &mut Nodes) -> Result<Self, ParseError> {
        let node = nodes.take_first()?;
        let node_position = node.unwrap_position();
        node.validate_heading_with_depth(2).map_err(|err| {
            nodes.put_back(node);
            err.at_position(node_position)
        })?;
        let position = node.unwrap_position();
        let text = node.children_text();

        let (version, date) = Self::parse_text(&text).map_err(|err| {
            nodes.put_back(node);
            err.at_position(node_position)
        })?;

        let changes = Changes::try_from_nodes(nodes)?;
        Ok(Release {
            position,
            date,
            version,
            changes,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from_nodes {
        use super::*;
        use test_utils::{
            fails_for_empty_nodes, fails_for_invalid_heading_depth, fails_for_wrong_node,
            heading_node, list_item_node, list_node, paragraph_node,
        };

        use crate::node::Nodes;

        fails_for_empty_nodes!(Release);
        fails_for_wrong_node!(Release, paragraph_node("nope"));
        fails_for_invalid_heading_depth!(Release, 3);

        #[test]
        fn fails_if_version_is_invalid() {
            let node = heading_node((2, "[x.y.z] - 2024-01-01"));
            let nodes_vec = vec![node];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let error = Release::try_from_nodes(&mut nodes).unwrap_err();
            assert!(error.is_invalid_text_error());
            assert_eq!(nodes.len(), 1);
        }

        #[test]
        fn fails_if_date_is_invalid() {
            let node = heading_node((2, "[1.0.0] - big-azz-date"));
            let nodes_vec = vec![node];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let error = Release::try_from_nodes(&mut nodes).unwrap_err();
            assert!(error.is_invalid_text_error());
            assert_eq!(nodes.len(), 1);
        }

        // TODO: Make sure this is coherent with change_sets.
        #[test]
        fn works_without_changes() {
            let node = heading_node((2, "[1.0.0] - 2024-01-01"));
            let node_position = node.unwrap_position();
            let nodes_vec = vec![node];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let release = Release::try_from_nodes(&mut nodes).unwrap();
            assert_eq!(release.version, Version::new(1, 0, 0));
            assert_eq!(release.date, NaiveDate::from_ymd(2024, 1, 1).unwrap());
            assert_eq!(release.position, node_position);
        }

        #[test]
        fn works_with_one_change() {
            let release_heading = heading_node((2, "[1.1.3] - 2024-03-21"));
            let release_position = release_heading.unwrap_position();
            let change_set_heading = heading_node((3, "Added"));
            let change_set_list =
                list_node([list_item_node("Big changes came for y'all")].as_slice());
            let unrelated_node = paragraph_node("unrelated");
            let nodes_vec = vec![
                release_heading,
                change_set_heading,
                change_set_list,
                unrelated_node,
            ];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let release = Release::try_from_nodes(&mut nodes).unwrap();
            assert_eq!(release.version, Version::new(1, 1, 3));
            assert_eq!(release.date, NaiveDate::from_ymd(2024, 3, 21).unwrap());
            assert_eq!(release.position, release_position);
            let added = release.changes.added.unwrap().changes;
            assert_eq!(added.len(), 1);
            assert_eq!(added[0].text, "Big changes came for y'all");
            assert_eq!(nodes.len(), 1);
            assert!(nodes.take_first().unwrap().validate_paragraph().is_ok());
        }
    }
}
