mod io;
mod markdown;

pub use io::*;
pub use markdown::*;

use std::fmt;

/// Main crate error type.
///
/// This error type is an enum wrapping all possible error types of this library.
#[derive(Debug, Clone)]
pub enum Error {
    /// This variant can occur when parsing a markdown string. See [InvalidMarkdownError] for more information.
    InvalidMarkdownError(InvalidMarkdownError),
    /// This varian can occur when reading a file. See [IoError] for more information.
    IoError(IoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidMarkdownError(error) => fmt::Display::fmt(error, f),
            Error::IoError(error) => fmt::Display::fmt(error, f),
        }
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl From<InvalidMarkdownError> for Error {
    fn from(error: InvalidMarkdownError) -> Self {
        Self::InvalidMarkdownError(error)
    }
}
