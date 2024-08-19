use std::fmt::{Display, Formatter, Result};

use crate::clast_utils::InvalidNodeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ComponentError(ComponentError),
}

impl From<ComponentError> for Error {
    fn from(error: ComponentError) -> Self {
        Self::ComponentError(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::ComponentError(error) => Display::fmt(error, f),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentError {
    pub component: &'static str,
    pub kind: ComponentErrorKind,
}

impl ComponentError {
    pub fn new<T: Into<ComponentErrorKind>>(component: &'static str, kind: T) -> Self {
        Self {
            component,
            kind: kind.into(),
        }
    }

    pub fn missing_nodes(component: &'static str) -> Self {
        Self::new(component, ComponentErrorKind::MissingNodes)
    }

    pub fn invalid_node(component: &'static str, error: InvalidNodeError) -> Self {
        Self::new(component, ComponentErrorKind::InvalidNode(error))
    }
}

impl Display for ComponentError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "error while assembling {}: ", self.component)?;
        Display::fmt(&self.kind, f)
    }
}

impl std::error::Error for ComponentError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentErrorKind {
    MissingNodes,
    InvalidNode(InvalidNodeError),
}

impl ComponentErrorKind {
    pub fn unwrap_invalid_node_error(self) -> InvalidNodeError {
        match self {
            Self::InvalidNode(error) => error,
            _ => panic!("cannot unwrap invalid node error on {:?}", self),
        }
    }

    pub fn is_missing_nodes(&self) -> bool {
        matches!(self, Self::MissingNodes)
    }
}

impl Display for ComponentErrorKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::MissingNodes => write!(f, "missing nodes"),
            Self::InvalidNode(error) => Display::fmt(error, f),
        }
    }
}
