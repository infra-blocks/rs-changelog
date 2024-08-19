mod kind;

pub use kind::*;

use crate::InvalidNodeError;

// TODO: rename to sections or some synonym.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    kind: NodeKind,
    location: location::Span,
}

impl Node {
    fn new<T: Into<NodeKind>, U: Into<location::Span>>(kind: T, location: U) -> Self {
        Self {
            kind: kind.into(),
            location: location.into(),
        }
    }
}

impl TryFrom<mdast::Node> for Node {
    type Error = InvalidNodeError;

    fn try_from(node: mdast::Node) -> Result<Self, Self::Error> {
        match node.kind {
            // TODO: check heading level.
            mdast::NodeKind::Heading(heading) => Ok(Self::new(
                NodeKind::heading(heading.level, node.children_text()),
                node.location,
            )),
            mdast::NodeKind::Paragraph => Ok(Self::new(
                NodeKind::paragraph(node.children_text()),
                node.location,
            )),
            mdast::NodeKind::List(list) => Ok(Self::new(
                // TODO: accumulate errors.
                NodeKind::list(node.children.map(ListItem::try_from).collect()?),
                node.location,
            )),
            mdast::NodeKind::Definition(definition) => Ok(Self::new(
                NodeKind::definition(definition.label, definition.destination),
                node.location,
            )),
            _ => Err(node.into()),
        }
    }
}

#[cfg(test)]
mod test {
    use location::{Location, Position, Span};

    use super::*;

    macro_rules! test_unwrap {
        ($unwrap:ident, $unwrap_ref:ident, $valid:expr, $invalid:expr) => {
            mod $unwrap {
                use super::*;

                #[test]
                fn should_work_with_owned_when_valid() {
                    let node = $valid;
                    node.$unwrap();
                }

                #[test]
                fn should_work_with_ref_when_valid() {
                    let node = $valid;
                    node.$unwrap_ref();
                }

                #[test]
                #[should_panic]
                fn should_fail_otherwise_when_invalid() {
                    let node = $invalid;
                    node.$unwrap();
                }
            }
        };
    }

    fn dummy_location() -> Location {
        Location::span(Span::new(Position::new(1, 1), Position::new(1, 2)))
    }

    test_unwrap!(
        unwrap_heading,
        unwrap_heading_ref,
        NodeKind::Heading(Heading::new(dummy_location(), 1, "Valid Heading")),
        NodeKind::Paragraph(Paragraph::new(dummy_location(), "Invalid Paragraph"))
    );

    test_unwrap!(
        unwrap_paragraph,
        unwrap_paragraph_ref,
        NodeKind::Paragraph(Paragraph::new(dummy_location(), "Valid Paragraph")),
        NodeKind::Heading(Heading::new(dummy_location(), 1, "Invalid Paragraph"))
    );

    test_unwrap!(
        unwrap_list,
        unwrap_list_ref,
        NodeKind::List(List::new(
            dummy_location(),
            vec![
                ListItem::new(dummy_location(), "Item 1"),
                ListItem::new(dummy_location(), "Item 2")
            ]
        )),
        NodeKind::Heading(Heading::new(dummy_location(), 1, "Invalid List"))
    );

    // We are not testing the contents of the nodes here, as this will be redundant with their own tests.
    // We simply check the type of the returned node.
    mod try_from {
        use super::*;
        use test_utils::{
            definition_node, heading_node, list_item_node, list_node, paragraph_node,
        };

        #[test]
        fn should_work_with_heading() {
            let heading = heading_node((1, "Valid Heading"));
            let result = NodeKind::try_from(&heading).unwrap();
            assert!(matches!(result, NodeKind::Heading(_)));
        }

        #[test]
        fn should_work_with_paragraph() {
            let paragraph = paragraph_node("Valid Paragraph");
            let result = NodeKind::try_from(&paragraph).unwrap();
            assert!(matches!(result, NodeKind::Paragraph(_)));
        }

        #[test]
        fn should_work_with_list() {
            let list =
                list_node(vec![list_item_node("Item 1"), list_item_node("Item 2")].as_slice());
            let result = NodeKind::try_from(&list).unwrap();
            assert!(matches!(result, NodeKind::List(_)));
        }

        #[test]
        fn should_work_with_definition() {
            let definition = definition_node(("1.2.3", "https://git.com/my-repo/tags/1.2.3"));
            let result = NodeKind::try_from(&definition).unwrap();
            assert!(matches!(result, NodeKind::Definition(_)));
        }
    }
}
