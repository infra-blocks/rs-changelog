use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::changelog::error::{ChangelogParseError, IntoChangelogParseError};
use crate::changelog::position::Position;
use markdown::mdast;
use semver::Op;

#[derive(Debug, PartialEq)]
pub enum ChangelogNode {
    Title,
    Description,
    Release,
    Changes,
    Change,
    Link,
}

/// This is a small 1 to 1 mapping between mdast::Node and a lighter version without
/// fields to be used with this module.
#[derive(Debug)]
pub enum MarkdownNode {
    Root,
    BlockQuote,
    FootnoteDefinition,
    MdxJsxFlowElement,
    List,
    MdxJsEsm,
    Toml,
    Yaml,
    Break,
    InlineCode,
    InlineMath,
    Delete,
    Emphasis,
    MdxTextExpression,
    FootnoteReference,
    Html,
    Image,
    ImageReference,
    MdxJsxTextElement,
    Link,
    LinkReference,
    Strong,
    Text,
    Code,
    Math,
    MdxFlowExpression,
    Heading,
    Table,
    ThematicBreak,
    TableRow,
    TableCell,
    ListItem,
    Definition,
    Paragraph,
}

impl MarkdownNode {
    pub fn from_mdast_node(node: &mdast::Node) -> Self {
        match node {
            mdast::Node::Root(_) => MarkdownNode::Root,
            mdast::Node::BlockQuote(_) => MarkdownNode::BlockQuote,
            mdast::Node::FootnoteDefinition(_) => MarkdownNode::FootnoteDefinition,
            mdast::Node::MdxJsxFlowElement(_) => MarkdownNode::MdxJsxFlowElement,
            mdast::Node::List(_) => MarkdownNode::List,
            mdast::Node::MdxjsEsm(_) => MarkdownNode::MdxJsEsm,
            mdast::Node::Toml(_) => MarkdownNode::Toml,
            mdast::Node::Yaml(_) => MarkdownNode::Yaml,
            mdast::Node::Break(_) => MarkdownNode::Break,
            mdast::Node::InlineCode(_) => MarkdownNode::InlineCode,
            mdast::Node::InlineMath(_) => MarkdownNode::InlineMath,
            mdast::Node::Delete(_) => MarkdownNode::Delete,
            mdast::Node::Emphasis(_) => MarkdownNode::Emphasis,
            mdast::Node::MdxTextExpression(_) => MarkdownNode::MdxTextExpression,
            mdast::Node::FootnoteReference(_) => MarkdownNode::FootnoteReference,
            mdast::Node::Html(_) => MarkdownNode::Html,
            mdast::Node::Image(_) => MarkdownNode::Image,
            mdast::Node::ImageReference(_) => MarkdownNode::ImageReference,
            mdast::Node::MdxJsxTextElement(_) => MarkdownNode::MdxJsxTextElement,
            mdast::Node::Link(_) => MarkdownNode::Link,
            mdast::Node::LinkReference(_) => MarkdownNode::LinkReference,
            mdast::Node::Strong(_) => MarkdownNode::Strong,
            mdast::Node::Text(_) => MarkdownNode::Text,
            mdast::Node::Code(_) => MarkdownNode::Code,
            mdast::Node::Math(_) => MarkdownNode::Math,
            mdast::Node::MdxFlowExpression(_) => MarkdownNode::MdxFlowExpression,
            mdast::Node::Heading(_) => MarkdownNode::Heading,
            mdast::Node::Table(_) => MarkdownNode::Table,
            mdast::Node::ThematicBreak(_) => MarkdownNode::ThematicBreak,
            mdast::Node::TableRow(_) => MarkdownNode::TableRow,
            mdast::Node::TableCell(_) => MarkdownNode::TableCell,
            mdast::Node::ListItem(_) => MarkdownNode::ListItem,
            mdast::Node::Definition(_) => MarkdownNode::Definition,
            mdast::Node::Paragraph(_) => MarkdownNode::Paragraph,
        }
    }
}

impl From<&mdast::Node> for MarkdownNode {
    fn from(node: &mdast::Node) -> Self {
        MarkdownNode::from_mdast_node(node)
    }
}

impl Display for MarkdownNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkdownNode::Root => write!(f, "root"),
            MarkdownNode::BlockQuote => write!(f, "block quote"),
            MarkdownNode::FootnoteDefinition => write!(f, "footnote definition"),
            MarkdownNode::MdxJsxFlowElement => write!(f, "mdx jsx flow element"),
            MarkdownNode::List => write!(f, "list"),
            MarkdownNode::MdxJsEsm => write!(f, "mdx js esm"),
            MarkdownNode::Toml => write!(f, "toml"),
            MarkdownNode::Yaml => write!(f, "yaml"),
            MarkdownNode::Break => write!(f, "break"),
            MarkdownNode::InlineCode => write!(f, "inline code"),
            MarkdownNode::InlineMath => write!(f, "inline math"),
            MarkdownNode::Delete => write!(f, "delete"),
            MarkdownNode::Emphasis => write!(f, "emphasis"),
            MarkdownNode::MdxTextExpression => write!(f, "mdx text expression"),
            MarkdownNode::FootnoteReference => write!(f, "footnote reference"),
            MarkdownNode::Html => write!(f, "html"),
            MarkdownNode::Image => write!(f, "image"),
            MarkdownNode::ImageReference => write!(f, "image reference"),
            MarkdownNode::MdxJsxTextElement => write!(f, "mdx jsx text element"),
            MarkdownNode::Link => write!(f, "link"),
            MarkdownNode::LinkReference => write!(f, "link reference"),
            MarkdownNode::Strong => write!(f, "strong"),
            MarkdownNode::Text => write!(f, "text"),
            MarkdownNode::Code => write!(f, "code"),
            MarkdownNode::Math => write!(f, "math"),
            MarkdownNode::MdxFlowExpression => write!(f, "mdx flow expression"),
            MarkdownNode::Heading => write!(f, "heading"),
            MarkdownNode::Table => write!(f, "table"),
            MarkdownNode::ThematicBreak => write!(f, "thematic break"),
            MarkdownNode::TableRow => write!(f, "table row"),
            MarkdownNode::TableCell => write!(f, "table cell"),
            MarkdownNode::ListItem => write!(f, "list item"),
            MarkdownNode::Definition => write!(f, "definition"),
            MarkdownNode::Paragraph => write!(f, "paragraph"),
        }
    }
}

impl Display for ChangelogNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangelogNode::Title => write!(f, "title"),
            ChangelogNode::Description => write!(f, "description"),
            ChangelogNode::Release => write!(f, "release"),
            ChangelogNode::Changes => write!(f, "changes"),
            ChangelogNode::Change => write!(f, "change"),
            ChangelogNode::Link => write!(f, "link"),
        }
    }
}

#[derive(Debug)]
pub enum MarkdownError {
    InvalidHeadingDepthError(ChangelogNode, u8, u8),
    InvalidNodeError(ChangelogNode, MarkdownNode, mdast::Node),
    MissingNodeError(ChangelogNode),
    TrailingNodesError(String),
}

impl Display for MarkdownError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkdownError::MissingNodeError(node_type) => {
                write!(f, "error parsing {}: missing node", node_type)
            }
            MarkdownError::InvalidNodeError(node_type, expected, effective) => write!(
                f,
                "error parsing {}: node type expected to be {}, found {:?}",
                node_type, expected, effective
            ),
            MarkdownError::InvalidHeadingDepthError(node_type, expected, effective) => write!(
                f,
                "error parsing {}: heading depth expected to be {}, found {}",
                node_type, expected, effective
            ),
            MarkdownError::TrailingNodesError(formatted_nodes) => {
                write!(
                    f,
                    "unexpected trailing nodes in changelog: {}",
                    formatted_nodes
                )
            }
        }
    }
}

impl Error for MarkdownError {}

pub fn invalid_heading_depth_error(
    position: Option<Position>,
    node_type: ChangelogNode,
    expected: u8,
    effective: u8,
) -> ChangelogParseError<MarkdownError> {
    MarkdownError::InvalidHeadingDepthError(node_type, expected, effective)
        .wrap_with_position(position)
}

pub fn invalid_node_error(
    position: Option<Position>,
    node_type: ChangelogNode,
    expected: MarkdownNode,
    effective: mdast::Node,
) -> ChangelogParseError<MarkdownError> {
    MarkdownError::InvalidNodeError(node_type, expected, effective).wrap_with_position(position)
}

pub fn missing_node_error(node_type: ChangelogNode) -> ChangelogParseError<MarkdownError> {
    MarkdownError::MissingNodeError(node_type).wrap_with_position(None)
}

pub fn trailing_nodes_error(
    position: Option<Position>,
    formatted_nodes: String,
) -> ChangelogParseError<MarkdownError> {
    MarkdownError::TrailingNodesError(formatted_nodes).wrap_with_position(position)
}

pub fn check_heading_with_depth(
    node_type: ChangelogNode,
    node: &mdast::Node,
    expected_depth: u8,
) -> Result<mdast::Heading, ChangelogParseError<MarkdownError>> {
    match node {
        mdast::Node::Heading(heading) => {
            if heading.depth != expected_depth {
                Err(invalid_heading_depth_error(
                    node.position().map(Position::from),
                    node_type,
                    expected_depth,
                    heading.depth,
                ))
            } else {
                Ok(heading.clone())
            }
        }
        _ => Err(invalid_node_error(
            node.position().map(Position::from),
            node_type,
            node.into(),
            node.clone(),
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn missing_node_error() {
        let error = MarkdownError::missing_node(ChangelogNode::Title);
        assert!(
            matches!(error, MarkdownError::MissingNodeError(payload) if payload.node_type == ChangelogNode::Title)
        );
    }

    #[test]
    pub fn invalid_node_error() {
        let error = MarkdownError::invalid_node(InvalidNodeError {
            node_type: ChangelogNode::Title,
            expected: MarkdownNode::Heading,
            effective: mdast::Node::Paragraph(mdast::Paragraph {
                children: vec![],
                position: None,
            }),
        });
        // TODO: test the content of the payload.
        assert!(matches!(error, MarkdownError::InvalidNodeError(_)));
    }
}
