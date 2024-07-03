use crate::change::change_kind::ChangeKind;
use crate::change::ChangeSet;
use crate::node::Nodes;
use crate::node::TryFromNodes;
use crate::{ChangelogNode, ChangelogNodeKind, ParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Changes {
    pub added: Option<ChangeSet>,
    pub changed: Option<ChangeSet>,
    pub deprecated: Option<ChangeSet>,
    pub removed: Option<ChangeSet>,
    pub fixed: Option<ChangeSet>,
    pub security: Option<ChangeSet>,
}

impl Default for Changes {
    fn default() -> Self {
        Changes {
            added: None,
            changed: None,
            deprecated: None,
            removed: None,
            fixed: None,
            security: None,
        }
    }
}

impl ChangelogNode for Changes {
    fn node_kind() -> ChangelogNodeKind {
        ChangelogNodeKind::Changes
    }
}

impl Changes {
    fn check_changeset_is_none(
        change_set: Option<ChangeSet>,
        new_change_set: ChangeSet,
    ) -> Result<Option<ChangeSet>, ParseError> {
        match change_set {
            // TODO: review error message here and add position or sum'.
            Some(_) => Err(ParseError::invalid_node(
                "a changeset is already defined, but found duplicate",
            )),
            None => Ok(Some(new_change_set)),
        }
    }
}

impl TryFromNodes for Changes {
    fn try_from_nodes(nodes: &mut Nodes) -> Result<Self, ParseError> {
        let mut changes = Changes::default();

        loop {
            let result = ChangeSet::try_from_nodes(nodes);

            match result {
                // We can't parse change sets anymore.
                Err(_) => return Ok(changes),
                Ok(change_set) => match change_set.kind {
                    ChangeKind::Added => {
                        changes.added = Self::check_changeset_is_none(changes.added, change_set)?
                    }
                    ChangeKind::Changed => {
                        changes.changed =
                            Self::check_changeset_is_none(changes.changed, change_set)?
                    }
                    ChangeKind::Deprecated => {
                        changes.deprecated =
                            Self::check_changeset_is_none(changes.deprecated, change_set)?
                    }
                    ChangeKind::Removed => {
                        changes.removed =
                            Self::check_changeset_is_none(changes.removed, change_set)?
                    }
                    ChangeKind::Fixed => {
                        changes.fixed = Self::check_changeset_is_none(changes.fixed, change_set)?
                    }
                    ChangeKind::Security => {
                        changes.security =
                            Self::check_changeset_is_none(changes.security, change_set)?
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from_nodes {
        use super::*;
        use crate::markdown::NodeUtils;
        use test_utils::{heading_node, list_item_node, list_node};

        fn assert_empty(changes: &Changes) {
            assert!(changes.added.is_none());
            assert!(changes.changed.is_none());
            assert!(changes.deprecated.is_none());
            assert!(changes.removed.is_none());
            assert!(changes.fixed.is_none());
            assert!(changes.security.is_none());
        }

        #[test]
        fn returns_empty_changes_on_empty_nodes() {
            let nodes_vec = vec![];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let changes = Changes::try_from_nodes(&mut nodes).unwrap();
            assert_empty(&changes);
        }

        #[test]
        fn returns_empty_changes_on_partial_change_set() {
            // The list node has no children, which is an error.
            let nodes_vec = vec![heading_node((3, "Removed")), list_node(())];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let changes = Changes::try_from_nodes(&mut nodes).unwrap();
            assert_empty(&changes);
        }

        #[test]
        fn returns_empty_changes_on_wrong_node() {
            let nodes_vec = vec![
                heading_node((1, "Hello Big Changelog Here")),
                heading_node((2, "Small Release")),
            ];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let changes = Changes::try_from_nodes(&mut nodes).unwrap();
            assert_empty(&changes);
            // Check that the nodes were put back.
            assert!(nodes
                .take_first()
                .unwrap()
                .validate_heading_with_depth(1)
                .is_ok());
            assert!(nodes
                .take_first()
                .unwrap()
                .validate_heading_with_depth(2)
                .is_ok());
        }

        #[test]
        fn works_with_a_change_set() {
            let nodes_vec = vec![
                heading_node((3, "Fixed")),
                list_node(
                    [
                        list_item_node("big change here"),
                        list_item_node("small change here"),
                    ]
                    .as_slice(),
                ),
            ];
            let position = nodes_vec[0].unwrap_position();
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let changes = Changes::try_from_nodes(&mut nodes).unwrap();
            assert!(changes.added.is_none());
            assert!(changes.changed.is_none());
            assert!(changes.deprecated.is_none());
            assert!(changes.removed.is_none());
            assert!(changes.security.is_none());
            let fixes = changes.fixed.unwrap();
            assert_eq!(fixes.changes.len(), 2);
            assert_eq!(fixes.changes[0].text, "big change here");
            assert_eq!(fixes.changes[1].text, "small change here");
        }

        #[test]
        fn works_with_multiple_change_sets() {
            let nodes_vec = vec![
                heading_node((3, "Added")),
                list_node([list_item_node("adding some shiets")].as_slice()),
                heading_node((3, "Security")),
                list_node([list_item_node("some secure")].as_slice()),
            ];
            let position = nodes_vec[0].unwrap_position();
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let changes = Changes::try_from_nodes(&mut nodes).unwrap();
            let added = changes.added.unwrap();
            assert_eq!(added.changes.len(), 1);
            assert_eq!(added.changes[0].text, "adding some shiets");
            assert!(changes.changed.is_none());
            assert!(changes.deprecated.is_none());
            assert!(changes.removed.is_none());
            let security = changes.security.unwrap();
            assert_eq!(security.changes.len(), 1);
            assert_eq!(security.changes[0].text, "some secure");
            assert!(changes.fixed.is_none());
        }

        #[test]
        fn fails_with_duplicate_change_set() {
            let nodes_vec = vec![
                heading_node((3, "Added")),
                list_node([list_item_node("Big Change")].as_slice()),
                heading_node((3, "Added")),
                list_node([list_item_node("Big Fuck Up")].as_slice()),
            ];
            let position = nodes_vec[0].unwrap_position();
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let error = Changes::try_from_nodes(&mut nodes).unwrap_err();
            assert!(error.is_invalid_node_error());
        }
    }
}
