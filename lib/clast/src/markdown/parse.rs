use crate::markdown::error::MarkdownParseError;
use markdown::mdast::Node;

pub fn parse_markdown(markdown: &str) -> Result<Node, MarkdownParseError> {
    markdown::to_mdast(
        markdown,
        &markdown::ParseOptions {
            constructs: markdown::Constructs::gfm(),
            ..markdown::ParseOptions::default()
        },
    )
    .map_err(MarkdownParseError::from)
}
