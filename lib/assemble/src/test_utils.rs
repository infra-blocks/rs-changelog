macro_rules! test_missing_nodes {
    ($self:ty, $component:expr) => {
        #[test]
        fn should_fail_with_missing_nodes() {
            let mut nodes = stack::Stack::empty();
            let error = <$self>::try_from(&mut nodes).unwrap_err();
            assert_eq!(error.component, $component);
            assert!(error.kind.is_missing_nodes());
        }
    };
}

macro_rules! test_invalid_node {
    ($self:ty, $component:expr, $node:expr) => {
        #[test]
        fn should_fail_with_invalid_node() {
            let node = $node;
            let mut nodes = vec![node.clone()].into_iter().collect();
            let error = <$self>::try_from(&mut nodes).unwrap_err();
            assert_eq!(error.component, $component);
            let invalid_node_error = error.kind.unwrap_invalid_node_error();
            assert_eq!(invalid_node_error.location, *node.location());
            let invalid_node_kind = invalid_node_error.kind.unwrap_invalid_node_kind();
            assert_eq!(
                invalid_node_kind,
                $crate::clast_utils::InvalidNodeKind::from(&node)
            );
            // Make sure the node has been put back.
            assert_eq!(nodes.len(), 1);
            assert_eq!(nodes.pop().unwrap(), node);
        }
    };
}

pub(crate) use test_invalid_node;
pub(crate) use test_missing_nodes;
