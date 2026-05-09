mod changelog;
mod node_ext;
mod parser;
mod releases;
mod title;

pub use crate::parse::parser::ParseError;
pub use changelog::Changelog;
use parser::ChangelogParser;

pub fn parse(source: &str) -> Result<Changelog<'_>, ParseError> {
    let parser = ChangelogParser::new();
    parser.parse(source)
}
