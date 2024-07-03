use crate::markdown::{MarkdownNodeKind, NodeUtils};
use crate::node::{Nodes, NodesError};
use crate::Position;
use markdown::mdast::Node;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseErrorKind {
    InvalidMarkdownNode(String),
    // TODO: add the text that's bullshat and the reason. Or just keep the m'fking text.
    InvalidText(String),
    MissingNode,
    TrailingNodesError(String),
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::InvalidMarkdownNode(reason) => {
                write!(f, "{reason}")
            }
            ParseErrorKind::InvalidText(reason) => {
                write!(f, "{reason}")
            }
            ParseErrorKind::MissingNode => {
                write!(f, "missing node")
            }
            ParseErrorKind::TrailingNodesError(formatted_nodes) => {
                write!(f, "unexpected trailing nodes: {}", formatted_nodes)
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    position: Option<Position>,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
    changelog_node_kind: Option<ChangelogNodeKind>,
}

impl ParseError {
    fn new(kind: ParseErrorKind) -> Self {
        Self {
            position: None,
            kind,
            source: None,
            changelog_node_kind: None,
        }
    }

    pub fn invalid_node<T: Into<String>>(reason: T) -> ParseError {
        Self::new(ParseErrorKind::InvalidMarkdownNode(reason.into()))
    }

    pub fn invalid_node_type<L: Into<MarkdownNodeKind>>(
        expected: L,
        effective: &Node,
    ) -> ParseError {
        Self::invalid_node(format!(
            "expected a {} markdown node, but got {}",
            expected.into(),
            MarkdownNodeKind::from(effective)
        ))
        .at_position(effective.unwrap_position())
    }

    // TODO: this shouldn't be an into thing, but rather can be expressed a a stru.
    pub fn invalid_text<T: Into<String>, U: Into<String>>(text: T, reason: U) -> ParseError {
        Self::new(ParseErrorKind::InvalidText(format!(
            "invalid text \"{}\": {}",
            text.into(),
            reason.into()
        )))
    }

    pub fn missing_node() -> Self {
        Self::new(ParseErrorKind::MissingNode)
    }

    pub fn trailing_nodes(nodes: &Nodes) -> Self {
        Self::new(ParseErrorKind::TrailingNodesError(format!("{:?}", nodes)))
    }

    pub fn at_position<P: Into<Position>>(mut self, position: P) -> Self {
        self.position = Some(position.into());
        self
    }

    pub fn with_source<E: Error + Send + Sync + 'static>(mut self, source: E) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    pub fn for_changelog_node(mut self, kind: ChangelogNodeKind) -> Self {
        self.changelog_node_kind = Some(kind);
        self
    }

    pub fn is_missing_node_error(&self) -> bool {
        match self.kind {
            ParseErrorKind::MissingNode => true,
            _ => false,
        }
    }

    pub fn is_invalid_node_error(&self) -> bool {
        match self.kind {
            ParseErrorKind::InvalidMarkdownNode(_) => true,
            _ => false,
        }
    }

    pub fn is_invalid_text_error(&self) -> bool {
        match self.kind {
            ParseErrorKind::InvalidText(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_position(&self) -> Position {
        self.position.unwrap()
    }
}

impl<T> From<ParseError> for Result<T, ParseError> {
    fn from(err: ParseError) -> Self {
        Err(err)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}parse error{}: {}",
            Position::option_to_string(&self.position),
            match &self.changelog_node_kind {
                Some(kind) => format!(" for {}", kind),
                None => "".to_string(),
            },
            self.kind.to_string()
        )
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|e| e as &(dyn Error + 'static))
    }
}

#[derive(Debug, PartialEq)]
pub enum ChangelogNodeKind {
    Changelog,
    Title,
    Description,
    Release,
    Changes,
    ChangeSet,
    Change,
    Link,
    ChangeKind,
}

impl Display for ChangelogNodeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangelogNodeKind::Changelog => write!(f, "changelog"),
            ChangelogNodeKind::Title => write!(f, "title"),
            ChangelogNodeKind::Description => write!(f, "description"),
            ChangelogNodeKind::Release => write!(f, "release"),
            ChangelogNodeKind::ChangeKind => write!(f, "change kind"),
            ChangelogNodeKind::Changes => write!(f, "changes"),
            ChangelogNodeKind::ChangeSet => write!(f, "change set"),
            ChangelogNodeKind::Change => write!(f, "change"),
            ChangelogNodeKind::Link => write!(f, "link"),
        }
    }
}

// TODO: replace with Display implementation.
pub trait ChangelogNode {
    fn node_kind() -> ChangelogNodeKind;
}

impl From<NodesError> for ParseError {
    fn from(err: NodesError) -> Self {
        match err {
            NodesError::Empty => ParseError::missing_node(),
        }
    }
}
