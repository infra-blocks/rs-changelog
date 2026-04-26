mod changelog;
mod parser;
mod rules;

pub use crate::parse::parser::ParseError;
pub use changelog::Changelog;
use parser::ChangelogParser;
pub use rules::Rules;

pub fn parse(source: &str, rules: Rules) -> Result<Changelog<'_>, ParseError> {
    let parser = ChangelogParser::new(rules);
    parser.parse(source)
}
