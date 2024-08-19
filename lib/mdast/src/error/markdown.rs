use crate::convert::IntoLocation;
use location::Location;
use maybe_display::MaybeDisplay;
use std::{error, fmt};

/// An error that occurs when parsing a markdown string.
///
/// This error can occur when unsupported markdown features are found, for example.
/// It is a wrapper on top of the [markdown::message::Message] error type.
#[derive(Debug, Clone)]
pub struct InvalidMarkdownError {
    /// The location of the error in the markdown string.
    pub location: Option<Location>,
    /// The error message.
    pub message: String,
}

impl From<markdown::message::Message> for InvalidMarkdownError {
    fn from(message: markdown::message::Message) -> Self {
        Self {
            location: match message.place {
                Some(place) => match *place {
                    markdown::message::Place::Position(position) => Some(position.into_location()),
                    markdown::message::Place::Point(point) => Some(point.into_location()),
                },
                None => None,
            },
            message: message.reason,
        }
    }
}

impl fmt::Display for InvalidMarkdownError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.location.maybe_fmt(f)?;
        write!(f, "{}", self.message)
    }
}

impl error::Error for InvalidMarkdownError {}
