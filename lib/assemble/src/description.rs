use location::Location;
use stack::Stack;

use crate::{clast_utils::NodeValidation, ComponentError};

/// The changelog's description.
///
/// The description is the block of text that is immediately under the title.
/// This is a non optional component of the changelog and can span multiple paragraphs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description {
    /// The desription's [Location].
    pub location: Location,
    /// The text of the description.
    pub text: String,
}

impl Description {
    /// Constructs a new instance of [Description].
    ///
    /// Note that the usual flow of construction is through the [TryFrom] trait.
    pub fn new<T: Into<Location>, U: Into<String>>(location: T, text: U) -> Self {
        Self {
            location: location.into(),
            text: text.into(),
        }
    }
}

impl TryFrom<&mut Stack<clast::NodeKind>> for Description {
    type Error = ComponentError;

    fn try_from(nodes: &mut Stack<clast::NodeKind>) -> Result<Self, Self::Error> {
        // We expect at least one paragraph, otherwise we can't construct a description.
        let node = nodes
            .pop()
            .ok_or(ComponentError::missing_nodes("description"))?;
        if let Err(err) = node.validate_paragraph() {
            nodes.push(node);
            return Err(ComponentError::invalid_node("description", err));
        }
        let paragraph = node.unwrap_paragraph();

        // The location of the description is the location of the first paragraph.
        // If there are more paragraphs, then the location is extended to cover them.
        let mut location = paragraph.location;
        // We collect all the paragraph text.
        let mut paragraphs = vec![paragraph.text];
        while let Some(node) = nodes.pop() {
            // If the node isn't a paragraph here, it's not an error. It simply needs we can't
            // extend the description anymore.
            if let Err(_) = node.validate_paragraph() {
                nodes.push(node);
                break;
            }
            let paragraph = node.unwrap_paragraph();
            location = location.extended_to(paragraph.location.unwrap_span().end());
            paragraphs.push(paragraph.text);
        }
        // And join them with a newline.
        let text = paragraphs.join("\n");
        Ok(Self::new(location, text))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from {
        use super::*;
        use crate::test_utils::{test_invalid_node, test_missing_nodes};

        #[test]
        fn should_work_with_a_paragraph() {
            let location = Location::span(((3, 1), (4, 1)));
            let text = "Upsum lorem whatever the fuck.\n";
            let node = clast::NodeKind::paragraph(location.clone(), text);
            let mut nodes = vec![node].into_iter().collect();
            let description = Description::try_from(&mut nodes).unwrap();
            assert_eq!(description.location, location);
            assert_eq!(description.text, text);
        }

        #[test]
        fn should_work_with_multiple_paragraphs() {
            let first_location = Location::span(((3, 1), (4, 1)));
            let first_text = "Upsum lorem whatever the fuck.\n";
            let first_paragaph =
                clast::NodeKind::paragraph(first_location.clone(), first_text.to_string());

            let second_location = Location::span(((5, 1), (6, 1)));
            let second_text = "This ain't no place for no hero.\n";
            let second_paragraph = clast::NodeKind::paragraph(second_location.clone(), second_text);

            let third_location = Location::span(((7, 1), (8, 1)));
            let third_text = "This ain't no place for no, better man.\n";
            let third_paragraph = clast::NodeKind::paragraph(third_location.clone(), third_text);

            let fourth_node_location = Location::span(((9, 1), (9, 20)));
            let fourth_node =
                clast::NodeKind::heading(fourth_node_location.clone(), 1, "Big Heading");

            let mut nodes = vec![
                first_paragaph,
                second_paragraph,
                third_paragraph,
                fourth_node.clone(),
            ]
            .into_iter()
            .collect();
            let description = Description::try_from(&mut nodes).unwrap();

            // We start by checking that not all nodes were consumed, but all paragraphs were.
            assert_eq!(nodes.len(), 1);
            assert_eq!(nodes.pop().unwrap(), fourth_node);

            assert_eq!(
                description.location,
                first_location.extended_to(third_location.unwrap_span().end())
            );
            assert_eq!(description.text, "Upsum lorem whatever the fuck.\n\nThis ain't no place for no hero.\n\nThis ain't no place for no, better man.\n");
        }

        test_invalid_node!(
            Description,
            "description",
            clast::NodeKind::heading(Location::span(((1, 1), (2, 1))), 1, "Big Heading")
        );

        test_missing_nodes!(Description, "description");
    }
}
