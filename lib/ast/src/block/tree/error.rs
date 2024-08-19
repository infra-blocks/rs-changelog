use std::{error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidSegment,
}

impl Error {
    pub fn invalid_segment() -> Self {
        Error::InvalidSegment
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid segment")
    }
}

impl error::Error for Error {}
