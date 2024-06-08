use eyre::{Context, Result};
use std::str::FromStr;

// TODO: trait to convert from MarkDown AST.
pub struct Changelog {}

impl FromStr for Changelog {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        todo!()
    }
}

impl Changelog {
    pub fn parse_from_file<T: Into<String>>(file: T) -> Result<Changelog> {
        let file = file.into();
        let content = std::fs::read_to_string(&file)
            .wrap_err(format!("Error reading changelog file '{}'", file))?;

        content.parse()
    }
}
