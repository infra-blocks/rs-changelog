mod leaf;
pub use leaf::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block<'a> {
    /* Container(ContainerNode<'a>), */
    Leaf(Leaf<'a>),
}

impl<'a> From<Leaf<'a>> for Block<'a> {
    fn from(leaf_node: Leaf<'a>) -> Self {
        Block::Leaf(leaf_node)
    }
}

// Those are test utils.
impl<'a> Block<'a> {
    pub fn unwrap_leaf(self) -> Leaf<'a> {
        match self {
            Block::Leaf(leaf_node) => leaf_node,
        }
    }
}
