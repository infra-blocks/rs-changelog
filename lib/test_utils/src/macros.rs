// TODO: take in nodes and node single node. Make it optionalcito or somssssh.

#[macro_export]
macro_rules! fails_for_empty_nodes {
    ($self:ty) => {
        #[test]
        fn fails_for_empty_nodes() {
            use crate::node::{Nodes, TryFromNodes};

            let nodes_vec = vec![];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let error = <$self>::try_from_nodes(&mut nodes).unwrap_err();
            assert!(error.is_missing_node_error());
        }
    };
}

#[macro_export]
macro_rules! fails_for_wrong_node {
    ($self:ty, $node:expr) => {
        #[test]
        fn fails_for_wrong_node() {
            use crate::node::{Nodes, TryFromNodes};

            let node = $node;
            let node_position = node.unwrap_position();
            let nodes_vec = vec![node];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let error = <$self>::try_from_nodes(&mut nodes).unwrap_err();
            assert!(error.is_invalid_node_error());
            assert_eq!(node_position, error.unwrap_position());
            assert_eq!(nodes.len(), 1);
            assert!(std::ptr::eq(nodes.take_first().unwrap(), &nodes_vec[0]))
        }
    };
}

#[macro_export]
macro_rules! fails_for_invalid_heading_depth {
    ($self:ty, $depth:expr) => {
        #[test]
        fn fails_for_invalid_heading_depth() {
            use crate::node::{Nodes, TryFromNodes};
            use $crate::heading_node;

            let depth: u8 = $depth;
            let node = heading_node(depth);
            let node_position = node.unwrap_position();
            let nodes_vec = vec![node];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let error = <$self>::try_from_nodes(&mut nodes).unwrap_err();
            assert!(error.is_invalid_node_error());
            assert_eq!(node_position, error.unwrap_position());
            assert_eq!(nodes.len(), 1);
            assert!(std::ptr::eq(nodes.take_first().unwrap(), &nodes_vec[0]))
        }
    };
}

#[macro_export]
macro_rules! fails_for_invalid_text {
    ($self:ty, $node:expr) => {
        #[test]
        fn fails_for_invalid_text() {
            use crate::node::{Nodes, TryFromNodes};

            let node = $node;
            let node_position = node.unwrap_position();
            let nodes_vec = vec![node];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let error = <$self>::try_from_nodes(&mut nodes).unwrap_err();
            assert!(error.is_invalid_text_error());
            assert_eq!(node_position, error.unwrap_position());
            assert_eq!(nodes.len(), 1);
            assert!(std::ptr::eq(nodes.take_first().unwrap(), &nodes_vec[0]))
        }
    };
}

#[macro_export]
macro_rules! works_with_valid_node {
    ($self:ty, $node:expr, $asserts:expr) => {
        #[test]
        fn works_with_valid_node() {
            use crate::node::{Nodes, TryFromNodes};

            let node = $node;
            let nodes_vec = vec![node];
            let node = &nodes_vec[0];
            let mut nodes = Nodes::from(nodes_vec.as_slice());
            let effective = <$self>::try_from_nodes(&mut nodes).unwrap();
            assert!(nodes.is_empty());
            $asserts(effective, node);
        }
    };
}
