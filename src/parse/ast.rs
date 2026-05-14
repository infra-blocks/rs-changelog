use std::collections::VecDeque;

use changelog_ast::{AstIterator, Node, RefDefs};

#[derive(Debug)]
pub struct Ast<'source> {
    /// The eagerly collected nodes sift through for parsing.
    nodes: VecDeque<Node<'source>>,
    /// The exhausted iterator to extract reference definitions from.
    iter: AstIterator<'source>,
}

impl<'source> Ast<'source> {
    pub fn front(&self) -> Option<&Node<'source>> {
        self.nodes.front()
    }

    pub fn pop_front(&mut self) -> Option<Node<'source>> {
        self.nodes.pop_front()
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Node<'source>> {
        self.nodes.get(index)
    }

    pub fn reference_definitions(&self) -> &RefDefs<'source> {
        self.iter.reference_definitions()
    }

    fn new(nodes: VecDeque<Node<'source>>, iter: AstIterator<'source>) -> Self {
        Self { nodes, iter }
    }
}

impl<'source> From<AstIterator<'source>> for Ast<'source> {
    fn from(mut iter: AstIterator<'source>) -> Self {
        let nodes = iter.by_ref().collect();
        Self::new(nodes, iter)
    }
}

impl<'source> From<&'source str> for Ast<'source> {
    fn from(value: &'source str) -> Self {
        AstIterator::new(value).into()
    }
}
