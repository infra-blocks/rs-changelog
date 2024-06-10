use std::error::Error;
use std::fmt::Display;

use crate::changelog::position::Position;

#[derive(Debug)]
pub struct ChangelogParseError<T: Error> {
    pub position: Option<Position>,
    pub error: Box<T>,
}

pub trait IntoChangelogParseError<T: Error> {
    fn wrap_with_position(self, position: Option<Position>) -> ChangelogParseError<T>;
}

impl<T: Error> IntoChangelogParseError<T> for T {
    fn wrap_with_position(self, position: Option<Position>) -> ChangelogParseError<T> {
        ChangelogParseError::wrap(self, position)
    }
}

impl<T: Error> ChangelogParseError<T> {
    fn format_position(&self) -> String {
        match &self.position {
            Some(position) => format!(
                "({}:{}, {}:{}): ",
                position.start.line, position.start.column, position.end.line, position.end.column
            ),
            None => String::new(),
        }
    }

    pub fn wrap(error: T, position: Option<Position>) -> Self {
        Self {
            position,
            error: Box::new(error),
        }
    }
}

impl<T: Error> Display for ChangelogParseError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.format_position(), self.error)
    }
}

impl<T: Error> Error for ChangelogParseError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

#[derive(Debug)]
pub struct RuntimeError<T: Error> {
    message: String,
    source: Option<Box<T>>,
}

impl<T: Error> Display for RuntimeError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<T: Error + 'static> Error for RuntimeError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|source| source as &dyn Error)
    }
}

impl<T: Error> RuntimeError<T> {
    pub fn new(message: String, source: Option<T>) -> Self {
        Self {
            message,
            source: match source {
                Some(err) => Some(Box::new(err)),
                None => None,
            },
        }
    }

    pub fn from_source(message: String, source: T) -> Self {
        Self::new(message, Some(source))
    }
}
