use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::changelog::error::{ChangelogParseError, IntoChangelogParseError};
use crate::changelog::position::Position;

#[derive(Debug)]
pub enum ParsedNodeKind {
    Changelog,
    Title,
    Description,
    Release,
    Changes,
    ChangeSet,
    Change,
    Link,
}

impl Display for ParsedNodeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsedNodeKind::Changelog => write!(f, "changelog"),
            ParsedNodeKind::Title => write!(f, "title"),
            ParsedNodeKind::Description => write!(f, "description"),
            ParsedNodeKind::Release => write!(f, "release"),
            ParsedNodeKind::Changes => write!(f, "changes"),
            ParsedNodeKind::ChangeSet => write!(f, "change set"),
            ParsedNodeKind::Change => write!(f, "change"),
            ParsedNodeKind::Link => write!(f, "link"),
        }
    }
}

#[derive(Debug)]
pub struct InvalidFormatError<T: Error> {
    kind: ParsedNodeKind,
    text: String,
    source: T,
}

impl<T: Error> Display for InvalidFormatError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid format for {} \"{}\"", self.kind, self.text)
    }
}

impl<T: Error + 'static> Error for InvalidFormatError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

pub fn invalid_format_error<P: Into<Position>, E: Error + 'static>(
    position: Option<P>,
    kind: ParsedNodeKind,
    text: String,
    source: E,
) -> ChangelogParseError<InvalidFormatError<E>> {
    InvalidFormatError { kind, text, source }.wrap_with_position(position.map(|p| p.into()))
}
