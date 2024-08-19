use location::Location;
use stack::Stack;

use crate::{clast_utils::NodeValidation, ComponentError};

/// The changelog title.
///
/// This has to be constructed from a heading of depth 1.
/// Its text will typically hold the literal "Changelog", although
/// it is not enforced at this level.
#[derive(Debug, Clone)]
pub struct Title {
    /// The location of the title.
    pub location: Location,
    /// The text of the title.
    pub text: String,
}

impl Title {
    /// Constructs a new instance of [Title].
    ///
    /// Note that the usual flow of construction is through the TryFrom trait
    /// from a [clast::Node].
    pub fn new<T: Into<Location>, U: Into<String>>(location: T, text: U) -> Self {
        Self {
            location: location.into(),
            text: text.into(),
        }
    }
}

impl TryFrom<&mut Stack<clast::NodeKind>> for Title {
    type Error = ComponentError;

    fn try_from(nodes: &mut Stack<clast::NodeKind>) -> Result<Self, Self::Error> {
        let node = nodes.pop().ok_or(ComponentError::missing_nodes("title"))?;
        if let Err(err) = node.validate_heading_with_depth(1) {
            nodes.push(node);
            return Err(ComponentError::invalid_node("title", err));
        }
        let heading = node.unwrap_heading();
        Ok(Self::new(heading.location, heading.text))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from {
        use super::*;
        use crate::clast_utils::InvalidNodeErrorKind;
        use crate::test_utils::{test_invalid_node, test_missing_nodes};

        #[test]
        fn should_work_with_valid_node() {
            let location = Location::span(((1, 1), (1, 10)));
            let node = clast::NodeKind::heading(location.clone(), 1, "Changelog");
            let mut nodes = vec![node].into_iter().collect();
            let title = Title::try_from(&mut nodes).unwrap();
            assert_eq!(title.location, location);
            assert_eq!(title.text, "Changelog");
        }

        #[test]
        fn should_fail_with_invalid_heading() {
            let location = Location::span(((1, 1), (1, 10)));
            let node = clast::NodeKind::heading(location.clone(), 2, "Changelog");
            let mut nodes = vec![node].into_iter().collect();
            let error = Title::try_from(&mut nodes).unwrap_err();
            assert_eq!(error.component, "title");
            let invalid_node_error = error.kind.unwrap_invalid_node_error();
            assert_eq!(invalid_node_error.location, location);
            assert!(matches!(
                invalid_node_error.kind,
                InvalidNodeErrorKind::InvalidHeadingDepth(1, 2)
            ));
        }

        test_invalid_node!(
            Title,
            "title",
            clast::NodeKind::paragraph(
                Location::span(((1, 1), (2, 1))),
                "Ipsum lorem whatever the fuck."
            )
        );

        test_missing_nodes!(Title, "title");
    }
}
