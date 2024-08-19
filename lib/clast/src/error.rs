use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct InvalidNodeError {
    pub location: location::Span,
    pub kind: InvalidNodeKind,
}

impl InvalidNodeError {
    pub fn new<T: Into<location::Span>, U: Into<InvalidNodeKind>>(location: T, kind: U) -> Self {
        Self {
            location: location.into(),
            kind: kind.into(),
        }
    }
}

impl Display for InvalidNodeError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.location)?;
        write!(f, "invalid node: ")?;
        match self.kind {
            InvalidNodeKind::Root => write!(f, "root"),
            InvalidNodeKind::BlockQuote => write!(f, "block quote"),
            InvalidNodeKind::FootnoteDefinition => write!(f, "footnote definition"),
            InvalidNodeKind::FootnoteReference => write!(f, "footnote reference"),
            InvalidNodeKind::LineBreak => write!(f, "break"),
            InvalidNodeKind::InlineCode => write!(f, "inline code"),
            InvalidNodeKind::InlineMath => write!(f, "inline math"),
            InvalidNodeKind::StrikeThrough => write!(f, "delete"),
            InvalidNodeKind::Emphasis => write!(f, "emphasis"),
            InvalidNodeKind::Html => write!(f, "html"),
            InvalidNodeKind::Image => write!(f, "image"),
            InvalidNodeKind::ImageReference => write!(f, "image reference"),
            InvalidNodeKind::Link => write!(f, "link"),
            InvalidNodeKind::LinkReference => write!(f, "link reference"),
            InvalidNodeKind::Strong => write!(f, "strong"),
            InvalidNodeKind::Text => write!(f, "text"),
            InvalidNodeKind::Code => write!(f, "code"),
            InvalidNodeKind::Math => write!(f, "math"),
            InvalidNodeKind::Table => write!(f, "table"),
            InvalidNodeKind::ThematicBreak => write!(f, "thematic break"),
            InvalidNodeKind::TableRow => write!(f, "table row"),
            InvalidNodeKind::TableCell => write!(f, "table cell"),
            InvalidNodeKind::HeadingWithLevelOver3(depth) => {
                write!(f, "heading with depth {} out of valid [1-3] range", depth)
            }
            InvalidNodeKind::OrderedList => write!(f, "ordered list"),
        }
    }
}

impl std::error::Error for InvalidNodeError {}

// TODO: from borrow?
impl From<&mdast::Node> for InvalidNodeError {
    fn from(node: &mdast::Node) -> Self {
        Self {
            location: node.location,
            kind: (&node.kind).into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvalidNodesErrors {
    pub errors: Vec<InvalidNodeError>,
}

impl Display for InvalidNodesErrors {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for error in &self.errors {
            Display::fmt(error, f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::error::Error for InvalidNodesErrors {}

impl From<Vec<InvalidNodeError>> for InvalidNodesErrors {
    fn from(errors: Vec<InvalidNodeError>) -> Self {
        Self { errors: errors }
    }
}

#[derive(Debug, Clone)]
pub enum InvalidNodeKind {
    // Always invalid nodes.
    BlockQuote,
    Code,
    Emphasis,
    FootnoteDefinition,
    FootnoteReference,
    Html,
    InlineCode,
    InlineMath,
    Image,
    ImageReference,
    LineBreak,
    Link,
    LinkReference,
    Math,
    Root,
    StrikeThrough,
    Strong,
    Table,
    TableCell,
    TableRow,
    Text,
    ThematicBreak,
    // Conditionally invalid nodes.
    HeadingWithLevelOver3(u8),
    OrderedList,
}

impl From<&mdast::NodeKind> for InvalidNodeKind {
    fn from(kind: &mdast::NodeKind) -> Self {
        match kind {
            mdast::NodeKind::BlockQuote => InvalidNodeKind::BlockQuote,
            mdast::NodeKind::Code => InvalidNodeKind::Code,
            mdast::NodeKind::Emphasis => InvalidNodeKind::Emphasis,
            mdast::NodeKind::FootnoteDefinition(_) => InvalidNodeKind::FootnoteDefinition,
            mdast::NodeKind::FootnoteReference(_) => InvalidNodeKind::FootnoteReference,
            mdast::NodeKind::Html(_) => InvalidNodeKind::Html,
            mdast::NodeKind::InlineCode(_) => InvalidNodeKind::InlineCode,
            mdast::NodeKind::InlineMath(_) => InvalidNodeKind::InlineMath,
            mdast::NodeKind::Image(_) => InvalidNodeKind::Image,
            mdast::NodeKind::ImageReference(_) => InvalidNodeKind::ImageReference,
            mdast::NodeKind::LineBreak => InvalidNodeKind::LineBreak,
            mdast::NodeKind::Link(_) => InvalidNodeKind::Link,
            mdast::NodeKind::LinkReference(_) => InvalidNodeKind::LinkReference,
            mdast::NodeKind::Math(_) => InvalidNodeKind::Math,
            mdast::NodeKind::Root => InvalidNodeKind::Root,
            mdast::NodeKind::StrikeThrough => InvalidNodeKind::StrikeThrough,
            mdast::NodeKind::Strong => InvalidNodeKind::Strong,
            mdast::NodeKind::Table(_) => InvalidNodeKind::Table,
            mdast::NodeKind::TableCell => InvalidNodeKind::TableCell,
            mdast::NodeKind::TableRow => InvalidNodeKind::TableRow,
            mdast::NodeKind::Text(_) => InvalidNodeKind::Text,
            mdast::NodeKind::ThematicBreak => InvalidNodeKind::ThematicBreak,
            // Those could be invalid nodes.
            mdast::NodeKind::Heading(heading) => heading.into(),
            mdast::NodeKind::List(list) => list.into(),
            // Those are definitely not invalid nodes.
            mdast::NodeKind::Definition(_)
            | mdast::NodeKind::ListItem(_)
            | mdast::NodeKind::Paragraph => {
                panic!(
                    "cannot create invalid node kind from a valid node: {:?}",
                    kind
                )
            }
        }
    }
}

impl From<&mdast::Heading> for InvalidNodeKind {
    fn from(heading: &mdast::Heading) -> Self {
        if heading.level > 3 {
            InvalidNodeKind::HeadingWithLevelOver3(heading.level)
        } else {
            panic!(
                "cannot create invalid node kind from a valid heading: {:?}",
                heading
            )
        }
    }
}

impl From<&mdast::List> for InvalidNodeKind {
    fn from(list: &mdast::List) -> Self {
        match list.kind {
            mdast::ListKind::Ordered(_) => InvalidNodeKind::OrderedList,
            _ => panic!(
                "cannot create invalid node kind from a valid list: {:?}",
                list
            ),
        }
    }
}

pub type Error = InvalidNodeError;
