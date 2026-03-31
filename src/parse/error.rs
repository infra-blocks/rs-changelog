use crate::parse::invalid_heading::InvalidHeading;

#[derive(Debug)]
pub enum ParseError {
    InvalidHeading(Vec<InvalidHeading>),
    IoError(std::io::Error),
}

impl From<Vec<InvalidHeading>> for ParseError {
    fn from(value: Vec<InvalidHeading>) -> Self {
        Self::InvalidHeading(value)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHeading(errors) => {
                for err in errors {
                    write!(f, "{}", err)?
                }
                Ok(())
            }
            Self::IoError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ParseError {}
