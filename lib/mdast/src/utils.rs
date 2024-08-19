use crate::{InvalidMarkdownError, IoError};
use markdown::mdast as native;
use std::path::Path;

/// A convenient function to read from a file and turn the error into an [IoError] on failure.
pub fn read_file<T: AsRef<Path>>(file: T) -> Result<String, IoError> {
    std::fs::read_to_string(&file).map_err(|err| IoError::while_reading_file(file, err))
}

/// A convenient function to parse a markdown string into an [native::Node] and turn the error into an [InvalidMarkdownError] on failure.
pub fn parse_markdown(markdown: &str) -> Result<native::Node, InvalidMarkdownError> {
    let constructs = markdown::Constructs {
        math_flow: true,
        math_text: true,
        ..markdown::Constructs::gfm()
    };
    Ok(markdown::to_mdast(
        markdown,
        &markdown::ParseOptions {
            constructs,
            ..markdown::ParseOptions::default()
        },
    )?)
}
