use std::iter::FusedIterator;

use crate::Node;

/// An iterator over the children of a node.
///
/// Can be one of two variants: for an internal node, or for a leaf node.
pub enum NodeChildrenIterator<'node, 'source> {
    /// Internal node iterator.
    Internal(std::slice::Iter<'node, Node<'source>>),
    /// Leaf node iterator. Always yields [None].
    Leaf,
}

impl<'node, 'source> FusedIterator for NodeChildrenIterator<'node, 'source> {}

impl<'node, 'source> Iterator for NodeChildrenIterator<'node, 'source> {
    type Item = &'node Node<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            NodeChildrenIterator::Internal(iter) => iter.next(),
            NodeChildrenIterator::Leaf => None,
        }
    }
}

impl<'node, 'source> From<&'node Node<'source>> for NodeChildrenIterator<'node, 'source> {
    fn from(value: &'node Node<'source>) -> Self {
        match value {
            // Internal nodes
            Node::BlockQuote(inner) => Self::Internal(inner.children.iter()),
            Node::CodeBlock(inner) => Self::Internal(inner.children.iter()),
            Node::DefinitionList(inner) => Self::Internal(inner.children.iter()),
            Node::DefinitionListTitle(inner) => Self::Internal(inner.children.iter()),
            Node::DefinitionListDefinition(inner) => Self::Internal(inner.children.iter()),
            Node::Emphasis(inner) => Self::Internal(inner.children.iter()),
            Node::FootnoteDefinition(inner) => Self::Internal(inner.children.iter()),
            Node::Heading(inner) => Self::Internal(inner.children.iter()),
            Node::HtmlBlock(inner) => Self::Internal(inner.children.iter()),
            Node::Image(inner) => Self::Internal(inner.children.iter()),
            Node::Item(inner) => Self::Internal(inner.children.iter()),
            Node::Link(inner) => Self::Internal(inner.children.iter()),
            Node::List(inner) => Self::Internal(inner.children.iter()),
            Node::MetadataBlock(inner) => Self::Internal(inner.children.iter()),
            Node::Paragraph(inner) => Self::Internal(inner.children.iter()),
            Node::Strong(inner) => Self::Internal(inner.children.iter()),
            Node::Strikethrough(inner) => Self::Internal(inner.children.iter()),
            Node::Subscript(inner) => Self::Internal(inner.children.iter()),
            Node::Superscript(inner) => Self::Internal(inner.children.iter()),
            Node::Table(inner) => Self::Internal(inner.children.iter()),
            Node::TableCell(inner) => Self::Internal(inner.children.iter()),
            Node::TableHead(inner) => Self::Internal(inner.children.iter()),
            Node::TableRow(inner) => Self::Internal(inner.children.iter()),
            // Leaf nodes
            Node::Code(_) => Self::Leaf,
            Node::DisplayMath(_) => Self::Leaf,
            Node::FootnoteReference(_) => Self::Leaf,
            Node::HardBreak(_) => Self::Leaf,
            Node::Html(_) => Self::Leaf,
            Node::InlineHtml(_) => Self::Leaf,
            Node::InlineMath(_) => Self::Leaf,
            Node::Rule(_) => Self::Leaf,
            Node::SoftBreak(_) => Self::Leaf,
            Node::TaskListMarker(_) => Self::Leaf,
            Node::Text(_) => Self::Leaf,
        }
    }
}

impl<'source> Node<'source> {
    /// Returns an iterator over the node's chidlren.
    pub fn children(&self) -> NodeChildrenIterator<'_, 'source> {
        self.into()
    }
}

#[cfg(test)]
mod test {
    use pulldown_cmark::CowStr;

    use crate::{HardBreak, Paragraph, Text};

    use super::*;

    #[test]
    fn should_yield_none_for_leaf() {
        assert!(
            Node::HardBreak(HardBreak::new(0..1))
                .children()
                .next()
                .is_none()
        )
    }

    #[test]
    fn should_dispatch_to_inner_for_internal() {
        let children = vec![
            Node::Text(Text::new(0..5, CowStr::from("hello"))),
            Node::Text(Text::new(6..11, CowStr::from("world"))),
        ];
        let node = Node::Paragraph(Paragraph::new(0..11, children.clone()));
        let effective: Vec<_> = node.children().cloned().collect();
        assert_eq!(children, effective);
    }
}
