use markdown::mdast::Node;
use std::fmt::{Display, Formatter};

/// This is a 1 to 1 mapping with mdast::Node to identify the type of node without
/// moving around the fields.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MarkdownNodeKind {
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

impl MarkdownNodeKind {
    pub fn from_mdast_node(node: &Node) -> Self {
        match node {
            Node::Root(_) => MarkdownNodeKind::Root,
            Node::BlockQuote(_) => MarkdownNodeKind::BlockQuote,
            Node::FootnoteDefinition(_) => MarkdownNodeKind::FootnoteDefinition,
            Node::MdxJsxFlowElement(_) => MarkdownNodeKind::MdxJsxFlowElement,
            Node::List(_) => MarkdownNodeKind::List,
            Node::MdxjsEsm(_) => MarkdownNodeKind::MdxJsEsm,
            Node::Toml(_) => MarkdownNodeKind::Toml,
            Node::Yaml(_) => MarkdownNodeKind::Yaml,
            Node::Break(_) => MarkdownNodeKind::Break,
            Node::InlineCode(_) => MarkdownNodeKind::InlineCode,
            Node::InlineMath(_) => MarkdownNodeKind::InlineMath,
            Node::Delete(_) => MarkdownNodeKind::Delete,
            Node::Emphasis(_) => MarkdownNodeKind::Emphasis,
            Node::MdxTextExpression(_) => MarkdownNodeKind::MdxTextExpression,
            Node::FootnoteReference(_) => MarkdownNodeKind::FootnoteReference,
            Node::Html(_) => MarkdownNodeKind::Html,
            Node::Image(_) => MarkdownNodeKind::Image,
            Node::ImageReference(_) => MarkdownNodeKind::ImageReference,
            Node::MdxJsxTextElement(_) => MarkdownNodeKind::MdxJsxTextElement,
            Node::Link(_) => MarkdownNodeKind::Link,
            Node::LinkReference(_) => MarkdownNodeKind::LinkReference,
            Node::Strong(_) => MarkdownNodeKind::Strong,
            Node::Text(_) => MarkdownNodeKind::Text,
            Node::Code(_) => MarkdownNodeKind::Code,
            Node::Math(_) => MarkdownNodeKind::Math,
            Node::MdxFlowExpression(_) => MarkdownNodeKind::MdxFlowExpression,
            Node::Heading(_) => MarkdownNodeKind::Heading,
            Node::Table(_) => MarkdownNodeKind::Table,
            Node::ThematicBreak(_) => MarkdownNodeKind::ThematicBreak,
            Node::TableRow(_) => MarkdownNodeKind::TableRow,
            Node::TableCell(_) => MarkdownNodeKind::TableCell,
            Node::ListItem(_) => MarkdownNodeKind::ListItem,
            Node::Definition(_) => MarkdownNodeKind::Definition,
            Node::Paragraph(_) => MarkdownNodeKind::Paragraph,
        }
    }
}

impl From<&Node> for MarkdownNodeKind {
    fn from(node: &Node) -> Self {
        MarkdownNodeKind::from_mdast_node(node)
    }
}

impl Display for MarkdownNodeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkdownNodeKind::Root => write!(f, "root"),
            MarkdownNodeKind::BlockQuote => write!(f, "block quote"),
            MarkdownNodeKind::FootnoteDefinition => write!(f, "footnote definition"),
            MarkdownNodeKind::MdxJsxFlowElement => write!(f, "mdx jsx flow element"),
            MarkdownNodeKind::List => write!(f, "list"),
            MarkdownNodeKind::MdxJsEsm => write!(f, "mdx js esm"),
            MarkdownNodeKind::Toml => write!(f, "toml"),
            MarkdownNodeKind::Yaml => write!(f, "yaml"),
            MarkdownNodeKind::Break => write!(f, "break"),
            MarkdownNodeKind::InlineCode => write!(f, "inline code"),
            MarkdownNodeKind::InlineMath => write!(f, "inline math"),
            MarkdownNodeKind::Delete => write!(f, "delete"),
            MarkdownNodeKind::Emphasis => write!(f, "emphasis"),
            MarkdownNodeKind::MdxTextExpression => write!(f, "mdx text expression"),
            MarkdownNodeKind::FootnoteReference => write!(f, "footnote reference"),
            MarkdownNodeKind::Html => write!(f, "html"),
            MarkdownNodeKind::Image => write!(f, "image"),
            MarkdownNodeKind::ImageReference => write!(f, "image reference"),
            MarkdownNodeKind::MdxJsxTextElement => write!(f, "mdx jsx text element"),
            MarkdownNodeKind::Link => write!(f, "link"),
            MarkdownNodeKind::LinkReference => write!(f, "link reference"),
            MarkdownNodeKind::Strong => write!(f, "strong"),
            MarkdownNodeKind::Text => write!(f, "text"),
            MarkdownNodeKind::Code => write!(f, "code"),
            MarkdownNodeKind::Math => write!(f, "math"),
            MarkdownNodeKind::MdxFlowExpression => write!(f, "mdx flow expression"),
            MarkdownNodeKind::Heading => write!(f, "heading"),
            MarkdownNodeKind::Table => write!(f, "table"),
            MarkdownNodeKind::ThematicBreak => write!(f, "thematic break"),
            MarkdownNodeKind::TableRow => write!(f, "table row"),
            MarkdownNodeKind::TableCell => write!(f, "table cell"),
            MarkdownNodeKind::ListItem => write!(f, "list item"),
            MarkdownNodeKind::Definition => write!(f, "definition"),
            MarkdownNodeKind::Paragraph => write!(f, "paragraph"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod from_node {
        use super::*;

        #[test]
        fn works_for_root() {
            let node = Node::Root(markdown::mdast::Root {
                position: None,
                children: vec![],
            });
            assert_eq!(MarkdownNodeKind::Root, (&node).into());
        }
    }
}
