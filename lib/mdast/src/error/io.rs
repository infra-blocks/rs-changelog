use std::{error, fmt, path};

/// An error that occurs when reading a file.
/// This error is a wrapper on top of the standard [std::io::Error] type.
#[derive(Debug, Clone)]
pub struct IoError {
    /// The error message.
    pub message: String,
}

impl IoError {
    /// Augments the error message with the path of the file that could not be read.
    pub fn while_reading_file<T: AsRef<path::Path>>(path: T, error: std::io::Error) -> Self {
        Self {
            message: format!(
                "unable to read from file {}: {error}",
                path.as_ref().to_str().unwrap()
            ),
        }
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for IoError {}
