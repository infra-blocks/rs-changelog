use markdown::mdast::Node;

#[derive(Debug)]
pub struct Nodes<'a> {
    nodes: Vec<&'a Node>,
}

#[derive(Debug)]
pub enum NodesError {
    Empty,
}

impl<'a> Nodes<'a> {
    pub fn take_first(&mut self) -> Result<&'a Node, NodesError> {
        match self.nodes.pop() {
            Some(node) => Ok(node),
            None => Err(NodesError::Empty),
        }
    }

    pub fn put_back(&mut self, node: &'a Node) {
        self.nodes.push(node);
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

/// The nodes are going to come from mdast by moving the <node>.children field.
impl<'a> From<&'a [Node]> for Nodes<'a> {
    fn from(nodes: &'a [Node]) -> Self {
        let mut reversed: Vec<&Node> = nodes.iter().collect();
        reversed.reverse();

        /*
        We reverse the vector because we are going to treat it as a LIFO queue.
        We're going to pop from the head and possibly put back at the head. Because this is the
        worst type of operation on a vector, we're going to reverse it and treat the end as the head.
         */
        Nodes { nodes: reversed }
    }
}
