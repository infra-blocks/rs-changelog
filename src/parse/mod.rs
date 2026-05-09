mod ast;
mod changelog;
mod node_ext;
mod releases;
mod title;

pub use changelog::{Changelog, ChangelogParseError};
// TODO: reexport relevant types.

pub fn parse(source: &str) -> Result<Changelog<'_>, ChangelogParseError> {
    Changelog::parse(source)
}
