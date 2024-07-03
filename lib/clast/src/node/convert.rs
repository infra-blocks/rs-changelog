use markdown::mdast::Node;

use crate::node::Nodes;
use crate::ParseError;

/// This trait is for nodes that can be parsed from a single node.
pub trait TryFromNode
where
    Self: Sized,
{
    fn try_from_node(node: &Node) -> Result<Self, ParseError>;
}

pub trait TryFromNodes
where
    Self: Sized,
{
    fn try_from_nodes(nodes: &mut Nodes) -> Result<Self, ParseError>;
}

// TODO: put the changelog node info into the error at that level.
impl<T: TryFromNodes> TryFromNodes for Vec<T> {
    fn try_from_nodes(nodes: &mut Nodes) -> Result<Self, ParseError> {
        let mut vec = Vec::new();

        loop {
            // TODO: propagate the error if it isn't an invalid node error.
            // If it cannot be parsed, it will already be put back into the nodes.
            match T::try_from_nodes(nodes) {
                Ok(value) => vec.push(value),
                // Abandon as soon as we can't parse anymore.
                Err(_) => return Ok(vec),
            }
        }
    }
}

/// Nodes that can be parsed from a single node automatically get an implementation
/// to parse from multiple nodes.
impl<T: TryFromNode> TryFromNodes for T {
    fn try_from_nodes(nodes: &mut Nodes) -> Result<Self, ParseError> {
        let node = nodes.take_first()?;
        Ok(Self::try_from_node(&node).or_put_back(nodes, node)?)
    }
}

pub trait OrPutNodeBack<T> {
    fn or_put_back<'a>(self, nodes: &mut Nodes<'a>, node: &'a Node) -> Result<T, ParseError>;
}

impl<T> OrPutNodeBack<T> for Result<T, ParseError> {
    fn or_put_back<'a>(self, nodes: &mut Nodes<'a>, node: &'a Node) -> Result<T, ParseError> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                nodes.put_back(node);
                Err(err)
            }
        }
    }
}
