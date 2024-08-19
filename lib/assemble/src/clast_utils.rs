use location::Location;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidNodeError {
    pub location: Location,
    pub kind: InvalidNodeErrorKind,
}

impl InvalidNodeError {
    pub fn invalid_node_kind(node: &clast::NodeKind) -> Self {
        Self {
            location: node.location().clone(),
            kind: InvalidNodeErrorKind::InvalidNodeKind(node.into()),
        }
    }

    pub fn invalid_heading_depth(heading: &clast::Heading, expected: u8) -> Self {
        Self {
            location: heading.location.clone(),
            kind: InvalidNodeErrorKind::InvalidHeadingDepth(expected, heading.depth),
        }
    }
}

impl Display for InvalidNodeError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self.kind {
            InvalidNodeErrorKind::InvalidNodeKind(kind) => {
                write!(f, "unexpected markdown {}", kind)
            }
            InvalidNodeErrorKind::InvalidHeadingDepth(expected, actual) => {
                write!(
                    f,
                    "unexpected heading depth {}, expected {}",
                    actual, expected
                )
            }
        }
    }
}

impl std::error::Error for InvalidNodeError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidNodeErrorKind {
    InvalidNodeKind(InvalidNodeKind),
    InvalidHeadingDepth(u8, u8),
}

impl InvalidNodeErrorKind {
    pub fn unwrap_invalid_node_kind(self) -> InvalidNodeKind {
        match self {
            Self::InvalidNodeKind(kind) => kind,
            _ => panic!("cannot unwrap invalid node kind from {:?}", self),
        }
    }

    pub fn unwrap_invalid_heading_depth(self) -> (u8, u8) {
        match self {
            Self::InvalidHeadingDepth(expected, actual) => (expected, actual),
            _ => panic!("cannot unwrap invalid heading depth from {:?}", self),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidNodeKind {
    Heading,
    Paragraph,
    List,
    Definition,
}

impl From<&clast::NodeKind> for InvalidNodeKind {
    fn from(node: &clast::NodeKind) -> Self {
        match node {
            clast::NodeKind::Heading(_) => Self::Heading,
            clast::NodeKind::Paragraph(_) => Self::Paragraph,
            clast::NodeKind::List(_) => Self::List,
            clast::NodeKind::Definition(_) => Self::Definition,
        }
    }
}

impl Display for InvalidNodeKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Heading => write!(f, "heading"),
            Self::Paragraph => write!(f, "paragraph"),
            Self::List => write!(f, "list"),
            Self::Definition => write!(f, "definition"),
        }
    }
}

pub trait NodeValidation {
    fn validate_heading_with_depth(&self, depth: u8) -> Result<&clast::Heading, InvalidNodeError>;
    fn validate_paragraph(&self) -> Result<&clast::Paragraph, InvalidNodeError>;
}

impl NodeValidation for clast::NodeKind {
    fn validate_heading_with_depth(&self, depth: u8) -> Result<&clast::Heading, InvalidNodeError> {
        match self {
            clast::NodeKind::Heading(heading) => {
                if heading.depth == depth {
                    Ok(heading)
                } else {
                    Err(InvalidNodeError::invalid_heading_depth(heading, depth))
                }
            }
            _ => Err(InvalidNodeError::invalid_node_kind(self)),
        }
    }

    fn validate_paragraph(&self) -> Result<&clast::Paragraph, InvalidNodeError> {
        match self {
            clast::NodeKind::Paragraph(paragraph) => Ok(paragraph),
            _ => Err(InvalidNodeError::invalid_node_kind(self)),
        }
    }
}
