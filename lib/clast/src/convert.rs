use crate::{InvalidNodesErrors, Node};

pub trait TryIntoNodes {
    type Error;

    fn try_into_nodes(self) -> Result<Vec<Node>, Self::Error>;
}

impl TryIntoNodes for Vec<mdast::Node> {
    type Error = InvalidNodesErrors;

    fn try_into_nodes(self) -> Result<Vec<Node>, Self::Error> {
        let mut nodes = Vec::new();
        let mut errors = Vec::new();

        for node in self {
            match Node::try_from(node) {
                Ok(node) => nodes.push(node),
                Err(err) => errors.push(err),
            }
        }

        if errors.is_empty() {
            Ok(nodes)
        } else {
            Err(errors.into())
        }
    }
}
