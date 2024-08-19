use crate::{convert::TryIntoNodes, InvalidNodesErrors, Node};

/// The main struct produced by this crate.
///
/// A changelog is a collection of nodes representing its markdown structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Changelog {
    /// The nodes forming the changelog in the order they appear.
    pub nodes: Vec<Node>,
}

impl Changelog {
    fn new<T: Into<Vec<Node>>>(nodes: T) -> Self {
        Self {
            nodes: nodes.into(),
        }
    }
}

impl TryFrom<mdast::Mdast> for Changelog {
    type Error = InvalidNodesErrors;

    fn try_from(mdast: mdast::Mdast) -> Result<Self, Self::Error> {
        mdast.root.try_into()
    }
}

impl TryFrom<mdast::Node> for Changelog {
    type Error = InvalidNodesErrors;

    fn try_from(node: mdast::Node) -> Result<Self, Self::Error> {
        match node.kind {
            mdast::NodeKind::Root => match node.children {
                None => Ok(Changelog::new(Vec::new())),
                Some(children) => Ok(Changelog::new(children.try_into_nodes()?)),
            },
            _ => panic!("expected root node, got {:?}", node),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod from_str {
        use super::*;

        #[test]
        fn test_empty() {
            let changelog = Changelog::from_str("").unwrap();
            assert!(changelog.nodes.is_empty());
        }

        #[test]
        fn test_big_heading() {
            let changelog = Changelog::from_str("# Big Heading\n").unwrap();
            assert_eq!(changelog.nodes.len(), 1);
            let heading = changelog.nodes[0].unwrap_heading_ref();
            assert_eq!(heading.depth, 1);
            assert_eq!(heading.text, "Big Heading");
        }

        #[test]
        fn test_medium_heading() {
            let changelog = Changelog::from_str("## Medium Heading\n").unwrap();
            assert_eq!(changelog.nodes.len(), 1);
            let heading = changelog.nodes[0].unwrap_heading_ref();
            assert_eq!(heading.depth, 2);
            assert_eq!(heading.text, "Medium Heading");
        }

        #[test]
        fn test_small_heading() {
            let changelog = Changelog::from_str("### Small Heading\n").unwrap();
            assert_eq!(changelog.nodes.len(), 1);
            let heading = changelog.nodes[0].unwrap_heading_ref();
            assert_eq!(heading.depth, 3);
            assert_eq!(heading.text, "Small Heading");
        }

        #[test]
        fn test_paragraph() {
            let changelog = Changelog::from_str("\nIpsum Lorem Whatever The Fuck\n").unwrap();
            assert_eq!(changelog.nodes.len(), 1);
            let text = changelog.nodes[0].unwrap_paragraph_ref();
            assert_eq!(text.text, "Ipsum Lorem Whatever The Fuck");
        }

        // Showcases that each paragraph is treated as a single node.
        #[test]
        fn test_double_paragraph() {
            let changelog = Changelog::from_str(
                r"
Paragraph 1

Paragraph 2
",
            )
            .unwrap();
            assert_eq!(changelog.nodes.len(), 2);
            let text = changelog.nodes[0].unwrap_paragraph_ref();
            assert_eq!(text.text, "Paragraph 1");
            let text = changelog.nodes[1].unwrap_paragraph_ref();
            assert_eq!(text.text, "Paragraph 2");
        }

        #[test]
        fn test_list() {
            let changelog = Changelog::from_str(
                r"
- Item 1
- Item 2
- Item 3
            ",
            )
            .unwrap();
            assert_eq!(changelog.nodes.len(), 1);
            let list = changelog.nodes[0].unwrap_list_ref();
            assert_eq!(list.items.len(), 3);
            assert_eq!(list.items[0].text, "Item 1");
            assert_eq!(list.items[1].text, "Item 2");
            assert_eq!(list.items[2].text, "Item 3");
        }

        #[test]
        fn test_definition() {
            let changelog =
                Changelog::from_str("[the-label]: https://the-destination.com").unwrap();
            assert_eq!(changelog.nodes.len(), 1);
            let definition = changelog.nodes[0].unwrap_definition_ref();
            assert_eq!(definition.label, "the-label");
            assert_eq!(definition.destination, "https://the-destination.com");
        }
    }
}
