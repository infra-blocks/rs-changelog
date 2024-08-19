use crate::{utils::read_file, Error, Node};
use std::{path, str};

/// The main structure exported by this library.
///
/// The [Mdast] represents an Markdown Abstract Syntax tree. It is typically obtained by parsing a markdown string or file, using
/// the [FromStr] and [Mdast::try_from_file] implementations respectively.
pub struct Mdast {
    /// The [Root] of the AST.
    pub root: Node,
}

impl Mdast {
    /// Attempts to parse the content of a file into a [Mdast].
    ///
    /// It first reads the content of the file, then parses it into a [Mdast].
    ///
    /// # Errors
    /// If the file cannot be read, an [crate::error::IoError] is returned.
    /// If the contents cannot be parsed, an [crate::error::InvalidMarkdownError] is returned. This can occur when,
    /// for example, unsupported markdown features are found.
    pub fn try_from_file<T: AsRef<path::Path>>(file: T) -> Result<Mdast, Error> {
        let content = read_file(file)?;
        content.parse()
    }

    fn new<T: Into<Node>>(root: T) -> Self {
        Self { root: root.into() }
    }
}

impl str::FromStr for Mdast {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Node::try_from_str(&s)?;
        Ok(Self::new(root))
    }
}
