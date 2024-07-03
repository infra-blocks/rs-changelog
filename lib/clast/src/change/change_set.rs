use crate::change::change_kind::ChangeKind;
use crate::markdown::NodeUtils;
use crate::node::{Nodes, TryFromNodes};
use crate::{Change, ChangelogNode, ChangelogNodeKind, ParseError, Position};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSet {
    pub kind: ChangeKind,
    pub position: Position,
    pub changes: Vec<Change>,
}

impl ChangelogNode for ChangeSet {
    fn node_kind() -> ChangelogNodeKind {
        ChangelogNodeKind::ChangeSet
    }
}

impl TryFromNodes for ChangeSet {
    fn try_from_nodes(nodes: &mut Nodes) -> Result<Self, ParseError> {
        let heading_node = nodes.take_first()?;
        let position = heading_node.unwrap_position();
        nodes.put_back(heading_node);
        let change_kind = ChangeKind::try_from_nodes(nodes)?;

        let list_node = nodes.take_first()?;
        let list_position = list_node.unwrap_position();
        let list = list_node.validate_list().map_err(|err| {
            nodes.put_back(list_node);
            nodes.put_back(heading_node);
            err.at_position(list_position)
        })?;

        // TODO: an invalid node in this vector should result in a parsing error. If a list
        // is made of other nodes than list items, it should be an error.
        let mut children_nodes: Nodes = list.children.as_slice().into();

        // We don't expect this to happen in the normal situation. The mdast should correctly
        // populate a list node with at least one list item. But we put it there for
        // posterity. In case things change.
        if children_nodes.is_empty() {
            nodes.put_back(list_node);
            nodes.put_back(heading_node);
            return Err(ParseError::missing_node().at_position(list_position));
        }

        // TODO: return last error in try_from_nodes.
        let changes = Vec::try_from_nodes(&mut children_nodes)?;

        // Every changeset should include at least one change.
        if changes.is_empty() {
            children_nodes.put_back(list_node);
            children_nodes.put_back(heading_node);
            return Err(
                ParseError::invalid_node("a changeset should include at least one change")
                    .at_position(list_position),
            );
        }

        Ok(ChangeSet {
            kind: change_kind,
            position,
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

        fails_for_empty_nodes!(ChangeSet);
        fails_for_wrong_node!(ChangeSet, list_node(()));
        fails_for_invalid_heading_depth!(ChangeSet, 1);

        // TODO: invalid text macro.
        #[test]
        fn fails_if_heading_is_not_valid_change_kind() {
            let node = heading_node((3, "Big TOTO"));
            let nodes_vec = vec![node];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let error = ChangeSet::try_from_nodes(&mut nodes).unwrap_err();
            assert!(error.is_invalid_text_error());
            assert_eq!(nodes.len(), 1);
        }

        #[test]
        fn fails_with_wrong_second_node() {
            let heading = heading_node((3, "Added"));
            let paragraph = paragraph_node("stuff");
            let paragraph_position = paragraph.unwrap_position();
            let nodes_vec = vec![heading, paragraph];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let error = ChangeSet::try_from_nodes(&mut nodes).unwrap_err();
            assert!(error.is_invalid_node_error());
            // Check that the error is located on the second node.
            assert_eq!(error.unwrap_position(), paragraph_position);
            // Should put back both nodes, in the correct order.
            assert_eq!(nodes.len(), 2);
            assert!(nodes
                .take_first()
                .unwrap()
                .validate_heading_with_depth(3)
                .is_ok());
            assert!(nodes.take_first().unwrap().validate_paragraph().is_ok());
        }

        #[test]
        fn fails_when_no_changes_included() {
            let heading = heading_node((3, "Added"));
            let heading_position = heading.unwrap_position();
            let list_node = list_node(());
            let nodes_vec = vec![heading, list_node];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let error = ChangeSet::try_from_nodes(&mut nodes).unwrap_err();
            assert!(error.is_missing_node_error());
            assert!(nodes
                .take_first()
                .unwrap()
                .validate_heading_with_depth(3)
                .is_ok());
            assert!(nodes.take_first().unwrap().validate_list().is_ok());
        }

        #[test]
        fn works_with_one_change() {
            let heading = heading_node((3, "Added"));
            let heading_position = heading.unwrap_position();
            let list_node = list_node([list_item_node("the single item")].as_slice());
            let nodes_vec = vec![heading, list_node];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let change_set = ChangeSet::try_from_nodes(&mut nodes).unwrap();
            assert!(nodes.is_empty());
            assert_eq!(change_set.position, heading_position);
            assert_eq!(change_set.kind, ChangeKind::Added);
            assert_eq!(change_set.changes.len(), 1);
            assert_eq!(change_set.changes[0].text, "the single item");
        }

        #[test]
        fn works_with_multiple_changes() {
            let heading = heading_node((3, "Added"));
            let heading_position = heading.unwrap_position();
            let list_node = list_node(
                [
                    list_item_node("the first"),
                    list_item_node("the second"),
                    list_item_node("the third"),
                ]
                .as_slice(),
            );
            let nodes_vec = vec![heading, list_node];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let change_set = ChangeSet::try_from_nodes(&mut nodes).unwrap();
            assert!(nodes.is_empty());
            assert_eq!(change_set.position, heading_position);
            assert_eq!(change_set.kind, ChangeKind::Added);
            assert_eq!(change_set.changes.len(), 3);
            assert_eq!(change_set.changes[0].text, "the first");
            assert_eq!(change_set.changes[1].text, "the second");
            assert_eq!(change_set.changes[2].text, "the third");
        }
    }
}
