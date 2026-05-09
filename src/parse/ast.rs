use std::collections::VecDeque;

use changelog_ast::{AstIterator, Node};

pub struct Ast<'source>(VecDeque<Node<'source>>);

impl<'source> Ast<'source> {
    fn new(nodes: VecDeque<Node<'source>>) -> Self {
        Self(nodes)
    }

    pub fn front(&self) -> Option<&Node<'source>> {
        self.0.front()
    }

    pub fn pop_front(&mut self) -> Option<Node<'source>> {
        self.0.pop_front()
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Node<'source>> {
        self.0.get(index)
    }
}

impl<'source> FromIterator<Node<'source>> for Ast<'source> {
    fn from_iter<T: IntoIterator<Item = Node<'source>>>(iter: T) -> Self {
        let nodes: VecDeque<_> = iter.into_iter().collect();
        Self::new(nodes)
    }
}

impl<'source> From<&'source str> for Ast<'source> {
    fn from(value: &'source str) -> Self {
        AstIterator::new(value).collect()
    }
}
