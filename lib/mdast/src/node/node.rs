use crate::convert::IntoSpan;
use crate::node::align_kind::AlignKind;
use crate::node::kind::{ListKind, NodeKind};
use crate::{utils::parse_markdown, InvalidMarkdownError};
use markdown::mdast as native;

/// An [crate::Mdast] [Node].
///
/// Every node has a [NodeKind] that identifies the type of node it is, and associates the specific paylod with it.
/// Every node also has a [location::Span] that identifies the span of text from which the node was parsed.
/// A node can have other nodes as children.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    /// The [NodeKind] of this [Node].
    pub kind: NodeKind,
    /// The textual location of the [Node].
    pub location: location::Span,
    /// The children of this [Node], if any.
    pub children: Option<Vec<Node>>,
}

impl Node {
    fn new<T: Into<NodeKind>, U: Into<location::Span>, V: Into<Option<Vec<Node>>>>(
        kind: T,
        location: U,
        children: V,
    ) -> Self {
        Self {
            kind: kind.into(),
            location: location.into(),
            children: children.into(),
        }
    }

    /// Parses the [&str] into a [Node].
    ///
    /// We're not implementing [std::str::FromStr] on purpose here to not export that behavior outside
    /// of the crate. The nodes should be created from the [Mdast] struct.
    pub(crate) fn try_from_str(s: &str) -> Result<Self, InvalidMarkdownError> {
        let native_root = parse_markdown(s)?;
        Ok(Node::from(native_root))
    }
}

impl From<native::Node> for Node {
    fn from(node: native::Node) -> Self {
        match node {
            native::Node::BlockQuote(inner) => inner.into(),
            native::Node::Break(inner) => inner.into(),
            native::Node::Definition(inner) => inner.into(),
            native::Node::Delete(inner) => inner.into(),
            native::Node::Emphasis(inner) => inner.into(),
            native::Node::FootnoteDefinition(inner) => inner.into(),
            native::Node::FootnoteReference(inner) => inner.into(),
            native::Node::Heading(inner) => inner.into(),
            native::Node::Html(inner) => inner.into(),
            native::Node::Image(inner) => inner.into(),
            native::Node::ImageReference(inner) => inner.into(),
            native::Node::InlineCode(inner) => inner.into(),
            native::Node::InlineMath(inner) => inner.into(),
            native::Node::Link(inner) => inner.into(),
            native::Node::LinkReference(inner) => inner.into(),
            native::Node::List(inner) => inner.into(),
            native::Node::ListItem(inner) => inner.into(),
            native::Node::Math(inner) => inner.into(),
            native::Node::Paragraph(inner) => inner.into(),
            native::Node::Root(inner) => inner.into(),
            native::Node::Strong(inner) => inner.into(),
            native::Node::Table(inner) => inner.into(),
            native::Node::TableCell(inner) => inner.into(),
            native::Node::TableRow(inner) => inner.into(),
            native::Node::Text(inner) => inner.into(),
            native::Node::ThematicBreak(inner) => inner.into(),
            _ => panic!("unimplemented for {:?} yet", node),
        }
    }
}

impl From<native::BlockQuote> for Node {
    fn from(block_quote: native::BlockQuote) -> Self {
        Self::new(
            NodeKind::block_quote(),
            block_quote.position.into_span(),
            Some(block_quote.children.into_nodes()),
        )
    }
}

impl From<native::Break> for Node {
    fn from(_break: native::Break) -> Self {
        Self::new(NodeKind::line_break(), _break.position.into_span(), None)
    }
}

impl From<native::Definition> for Node {
    fn from(definition: native::Definition) -> Self {
        Self::new(
            NodeKind::definition(definition.identifier, definition.url, definition.title),
            definition.position.into_span(),
            None,
        )
    }
}

impl From<native::Emphasis> for Node {
    fn from(emphasis: native::Emphasis) -> Self {
        Self::new(
            NodeKind::emphasis(),
            emphasis.position.into_span(),
            Some(emphasis.children.into_nodes()),
        )
    }
}

impl From<native::FootnoteDefinition> for Node {
    fn from(footnote_definition: native::FootnoteDefinition) -> Self {
        Self::new(
            NodeKind::footnote_definition(footnote_definition.identifier),
            footnote_definition.position.into_span(),
            Some(footnote_definition.children.into_nodes()),
        )
    }
}

impl From<native::FootnoteReference> for Node {
    fn from(footnote_reference: native::FootnoteReference) -> Self {
        Self::new(
            NodeKind::footnote_reference(footnote_reference.identifier),
            footnote_reference.position.into_span(),
            None,
        )
    }
}

impl From<native::Heading> for Node {
    fn from(heading: native::Heading) -> Self {
        Self::new(
            NodeKind::heading(heading.depth),
            heading.position.into_span(),
            Some(heading.children.into_nodes()),
        )
    }
}

impl From<native::Html> for Node {
    fn from(html: native::Html) -> Self {
        Self::new(NodeKind::html(html.value), html.position.into_span(), None)
    }
}

impl From<native::Image> for Node {
    fn from(image: native::Image) -> Self {
        Self::new(
            NodeKind::image(image.alt, image.url, image.title),
            image.position.into_span(),
            None,
        )
    }
}

impl From<native::ImageReference> for Node {
    fn from(image_reference: native::ImageReference) -> Self {
        Self::new(
            NodeKind::image_reference(
                image_reference.alt,
                image_reference.reference_kind,
                image_reference.identifier,
            ),
            image_reference.position.into_span(),
            None,
        )
    }
}

impl From<native::InlineCode> for Node {
    fn from(inline_code: native::InlineCode) -> Self {
        Self::new(
            NodeKind::inline_code(inline_code.value),
            inline_code.position.into_span(),
            None,
        )
    }
}

impl From<native::InlineMath> for Node {
    fn from(inline_math: native::InlineMath) -> Self {
        Self::new(
            NodeKind::inline_math(inline_math.value),
            inline_math.position.into_span(),
            None,
        )
    }
}

impl From<native::Link> for Node {
    fn from(link: native::Link) -> Self {
        Self::new(
            NodeKind::link(link.url, link.title),
            link.position.into_span(),
            Some(link.children.into_nodes()),
        )
    }
}

impl From<native::LinkReference> for Node {
    fn from(link_reference: native::LinkReference) -> Self {
        Self::new(
            NodeKind::link_reference(link_reference.identifier, link_reference.reference_kind),
            link_reference.position.into_span(),
            Some(link_reference.children.into_nodes()),
        )
    }
}

impl From<native::List> for Node {
    fn from(list: native::List) -> Self {
        // TODO: conversion function for that?.
        let kind = match list.start {
            Some(start) => ListKind::Ordered(start),
            None => ListKind::Unordered,
        };

        Self::new(
            NodeKind::list(kind),
            list.position.into_span(),
            Some(list.children.into_nodes()),
        )
    }
}

impl From<native::ListItem> for Node {
    fn from(list_item: native::ListItem) -> Self {
        Self::new(
            NodeKind::list_item(list_item.checked),
            list_item.position.into_span(),
            Some(list_item.children.into_nodes()),
        )
    }
}

impl From<native::Math> for Node {
    fn from(math: native::Math) -> Self {
        Self::new(NodeKind::math(math.value), math.position.into_span(), None)
    }
}

impl From<native::Paragraph> for Node {
    fn from(paragraph: native::Paragraph) -> Self {
        Self::new(
            NodeKind::paragraph(),
            paragraph.position.into_span(),
            Some(paragraph.children.into_nodes()),
        )
    }
}

impl From<native::Root> for Node {
    fn from(root: native::Root) -> Self {
        Self::new(
            NodeKind::root(),
            root.position.into_span(),
            Some(root.children.into_nodes()),
        )
    }
}

impl From<native::Strong> for Node {
    fn from(strong: native::Strong) -> Self {
        Self::new(
            NodeKind::strong(),
            strong.position.into_span(),
            Some(strong.children.into_nodes()),
        )
    }
}

impl From<native::Delete> for Node {
    fn from(strike_through: native::Delete) -> Self {
        Self::new(
            NodeKind::strike_through(),
            strike_through.position.into_span(),
            Some(strike_through.children.into_nodes()),
        )
    }
}

impl From<native::Table> for Node {
    fn from(table: native::Table) -> Self {
        let alignments: Vec<_> = table.align.into_iter().map(AlignKind::from).collect();

        Self::new(
            NodeKind::table(alignments),
            table.position.into_span(),
            Some(table.children.into_nodes()),
        )
    }
}

impl From<native::TableCell> for Node {
    fn from(table_cell: native::TableCell) -> Self {
        Self::new(
            NodeKind::table_cell(),
            table_cell.position.into_span(),
            Some(table_cell.children.into_nodes()),
        )
    }
}

impl From<native::TableRow> for Node {
    fn from(table_row: native::TableRow) -> Self {
        Self::new(
            NodeKind::table_row(),
            table_row.position.into_span(),
            Some(table_row.children.into_nodes()),
        )
    }
}

impl From<native::Text> for Node {
    fn from(text: native::Text) -> Self {
        Self::new(NodeKind::text(text.value), text.position.into_span(), None)
    }
}

impl From<native::ThematicBreak> for Node {
    fn from(thematic_break: native::ThematicBreak) -> Self {
        Self::new(
            NodeKind::thematic_break(),
            thematic_break.position.into_span(),
            None,
        )
    }
}

trait IntoNodes {
    fn into_nodes(self) -> Vec<Node>;
}

impl<T: IntoIterator<Item = native::Node>> IntoNodes for T {
    fn into_nodes(self) -> Vec<Node> {
        self.into_iter().map(Node::from).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::node::kind::{
        Definition, FootnoteDefinition, FootnoteReference, Heading, Html, Image, ImageReference,
        InlineCode, InlineMath, LinkReference, List, ListItem, Math, Table, Text,
    };

    impl Node {
        fn unwrap_definition(self) -> Definition {
            match self.kind {
                NodeKind::Definition(inner) => inner,
                _ => panic!("cannot unwrap definition on {:?}", self),
            }
        }

        fn unwrap_footnote_definition(self) -> FootnoteDefinition {
            match self.kind {
                NodeKind::FootnoteDefinition(inner) => inner,
                _ => panic!("cannot unwrap footnote definition on {:?}", self),
            }
        }

        fn unwrap_footnote_reference(self) -> FootnoteReference {
            match self.kind {
                NodeKind::FootnoteReference(inner) => inner,
                _ => panic!("cannot unwrap footnote reference on {:?}", self),
            }
        }

        fn unwrap_heading(self) -> Heading {
            match self.kind {
                NodeKind::Heading(inner) => inner,
                _ => panic!("cannot unwrap heading on {:?}", self),
            }
        }

        fn unwrap_html(self) -> Html {
            match self.kind {
                NodeKind::Html(inner) => inner,
                _ => panic!("cannot unwrap html on {:?}", self),
            }
        }

        fn unwrap_image(self) -> Image {
            match self.kind {
                NodeKind::Image(inner) => inner,
                _ => panic!("cannot unwrap image on {:?}", self),
            }
        }

        fn unwrap_image_reference(self) -> ImageReference {
            match self.kind {
                NodeKind::ImageReference(inner) => inner,
                _ => panic!("cannot unwrap image reference on {:?}", self),
            }
        }

        fn unwrap_inline_code(self) -> InlineCode {
            match self.kind {
                NodeKind::InlineCode(inner) => inner,
                _ => panic!("cannot unwrap inline code on {:?}", self),
            }
        }

        fn unwrap_inline_math(self) -> InlineMath {
            match self.kind {
                NodeKind::InlineMath(inner) => inner,
                _ => panic!("cannot unwrap inline math on {:?}", self),
            }
        }

        fn unwrap_link_reference(self) -> LinkReference {
            match self.kind {
                NodeKind::LinkReference(inner) => inner,
                _ => panic!("cannot unwrap link reference on {:?}", self),
            }
        }

        fn unwrap_list(self) -> List {
            match self.kind {
                NodeKind::List(inner) => inner,
                _ => panic!("cannot unwrap list on {:?}", self),
            }
        }

        fn unwrap_list_item(self) -> ListItem {
            match self.kind {
                NodeKind::ListItem(inner) => inner,
                _ => panic!("cannot unwrap list item on {:?}", self),
            }
        }

        fn unwrap_math(self) -> Math {
            match self.kind {
                NodeKind::Math(inner) => inner,
                _ => panic!("cannot unwrap math on {:?}", self),
            }
        }

        fn unwrap_table(self) -> Table {
            match self.kind {
                NodeKind::Table(inner) => inner,
                _ => panic!("cannot unwrap table on {:?}", self),
            }
        }

        fn unwrap_text(self) -> Text {
            match self.kind {
                NodeKind::Text(inner) => inner,
                _ => panic!("cannot unwrap text on {:?}", self),
            }
        }
    }

    #[test]
    fn test_block_quote() {
        let markdown = "> The quote.";
        let root = Node::try_from_str(markdown).unwrap();
        let block_quote = root.children.unwrap().remove(0);
        assert_eq!(block_quote.kind, NodeKind::BlockQuote);
        assert_eq!(
            block_quote.location,
            location::Span::new((1, 1, 0), (1, 13, 12))
        );
    }

    #[test]
    fn test_break() {
        let markdown = "This is some text\\\nWith a line break.";
        let root = Node::try_from_str(markdown).unwrap();
        let paragraph = root.children.unwrap().remove(0);
        // Wrapped in an outer paragraph.
        let mut _break = &paragraph.children.unwrap()[1];
        assert_eq!(_break.kind, NodeKind::LineBreak);
        assert_eq!(
            _break.location,
            location::Span::new((1, 18, 17), (2, 1, 19))
        );
    }

    #[test]
    fn test_definition() {
        let markdown = r#"[my-definition]: https://www.definition.com "big def""#;
        let root = Node::try_from_str(markdown).unwrap();
        let node = root.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 54, 53)));
        let definition = node.unwrap_definition();
        assert_eq!(definition.label, "my-definition");
        assert_eq!(definition.destination, "https://www.definition.com");
        assert_eq!(definition.title, Some("big def".to_string()));
    }

    #[test]
    fn test_emphasis() {
        let markdown = "*italic fucking text*";
        let root = Node::try_from_str(markdown).unwrap();
        let paragraph = root.children.unwrap().remove(0);
        let emphasis = paragraph.children.unwrap().pop().unwrap();
        assert_eq!(
            emphasis.location,
            location::Span::new((1, 1, 0), (1, 22, 21))
        );
        assert_eq!(emphasis.kind, NodeKind::Emphasis);
    }

    #[test]
    fn test_footnote_definition() {
        // TODO: test without the note and check the note's content (stored in children) when implementation for text exists.
        let markdown = "[^footnote]: This is the text yo.";
        let root = Node::try_from_str(markdown).unwrap();
        let node = root.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 34, 33)));
        let footnote_definition = node.unwrap_footnote_definition();
        assert_eq!(footnote_definition.label, "footnote");
    }

    #[test]
    fn test_footnote_reference() {
        let markdown = "[^1]

[^1]: This is the footnote.";
        let root = Node::try_from_str(markdown).unwrap();
        let paragraph = root.children.unwrap().remove(0);
        let node = paragraph.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 5, 4)));
        let reference = node.unwrap_footnote_reference();
        assert_eq!(reference.label, "1");
    }

    mod heading {
        use super::*;

        #[test]
        fn test_heading_1() {
            let root = Node::try_from_str("# Heading 1").unwrap();
            let node = root.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 12, 11)));
            let heading = node.unwrap_heading();
            assert_eq!(heading.level, 1);
        }

        #[test]
        fn test_heading_2() {
            let root = Node::try_from_str("## Heading 2").unwrap();
            let node = root.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 13, 12)));
            let heading = node.unwrap_heading();
            assert_eq!(heading.level, 2);
        }

        #[test]
        fn test_heading_3() {
            let root = Node::try_from_str("### Heading 3").unwrap();
            let node = root.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 14, 13)));
            let heading = node.unwrap_heading();
            assert_eq!(heading.level, 3);
        }

        #[test]
        fn test_heading_4() {
            let root = Node::try_from_str("#### Heading 4").unwrap();
            let node = root.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 15, 14)));
            let heading = node.unwrap_heading();
            assert_eq!(heading.level, 4);
        }

        #[test]
        fn test_heading_5() {
            let root = Node::try_from_str("##### Heading 5").unwrap();
            let node = root.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 16, 15)));
            let heading = node.unwrap_heading();
            assert_eq!(heading.level, 5);
        }

        #[test]
        fn test_heading_6() {
            let root = Node::try_from_str("###### Heading 6").unwrap();
            let node = root.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 17, 16)));
            let heading = node.unwrap_heading();
            assert_eq!(heading.level, 6);
        }
    }

    #[test]
    fn test_html() {
        let markdown = "<div>Some html</div>";
        let root = Node::try_from_str(markdown).unwrap();
        let node = root.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 21, 20)));
        let html = node.unwrap_html();
        assert_eq!(html.value, "<div>Some html</div>");
    }

    #[test]
    fn test_image() {
        let markdown = "![A big fucking cat](https://cat.com \"big kot\")";
        let root = Node::try_from_str(markdown).unwrap();
        let paragraph = root.children.unwrap().remove(0);
        let node = paragraph.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 48, 47)));
        let image = node.unwrap_image();
        assert_eq!(image.url, "https://cat.com");
        assert_eq!(image.alternate_text, "A big fucking cat");
        assert_eq!(image.title, Some("big kot".to_string()));
    }

    mod image_reference {
        use super::*;
        use crate::ReferenceKind;

        #[test]
        fn test_shortcut_reference() {
            let markdown = "![big-kot]

[big-kot]: https://cat.com \"Title is me\"";
            let root = Node::try_from_str(markdown).unwrap();
            let paragraph = root.children.unwrap().remove(0);
            let node = paragraph.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 11, 10)));
            let rererence = node.unwrap_image_reference();
            assert_eq!(rererence.alternative_text, "big-kot");
            assert_eq!(rererence.label, "big-kot");
            assert_eq!(rererence.reference_kind, ReferenceKind::Shortcut);
        }

        #[test]
        fn test_collapsed_reference() {
            let markdown = "![cat][]

[cat]: https://cat.com \"Title is me\"";
            let root = Node::try_from_str(markdown).unwrap();
            let paragraph = root.children.unwrap().remove(0);
            let node = paragraph.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 9, 8)));
            let rererence = node.unwrap_image_reference();
            assert_eq!(rererence.alternative_text, "cat");
            assert_eq!(rererence.label, "cat");
            assert_eq!(rererence.reference_kind, ReferenceKind::Collapsed);
        }

        #[test]
        fn test_full_reference() {
            let markdown = "![A big fucking cat][big-kot]

[big-kot]: https://cat.com \"Title is me\"";
            let root = Node::try_from_str(markdown).unwrap();
            let paragraph = root.children.unwrap().remove(0);
            let node = paragraph.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 30, 29)));
            let rererence = node.unwrap_image_reference();
            assert_eq!(rererence.alternative_text, "A big fucking cat");
            assert_eq!(rererence.label, "big-kot");
            assert_eq!(rererence.reference_kind, ReferenceKind::Full);
        }
    }

    #[test]
    fn test_inline_code() {
        let markdown = "`code-here`";
        let root = Node::try_from_str(markdown).unwrap();
        let paragraph = root.children.unwrap().remove(0);
        let node = paragraph.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 12, 11)));
        let inline_code = node.unwrap_inline_code();
        assert_eq!(inline_code.value, "code-here");
    }

    #[test]
    fn test_inline_math() {
        let markdown = "$\\sqrt{3x-1}$";
        let root = Node::try_from_str(markdown).unwrap();
        let paragraph = root.children.unwrap().remove(0);
        let node = paragraph.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 14, 13)));
        let inline_math = node.unwrap_inline_math();
        assert_eq!(inline_math.value, "\\sqrt{3x-1}");
    }

    #[test]
    fn test_link() {
        let markdown = "[big cat](https://cat.com \"big kot\")";
        let root = Node::try_from_str(markdown).unwrap();
        let paragraph = root.children.unwrap().remove(0);
        let node = paragraph.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 37, 36)));
        match node.kind {
            NodeKind::Link(link) => {
                assert_eq!(link.url, "https://cat.com");
                assert_eq!(link.title, Some("big kot".to_string()));
            }
            _ => panic!("expected link, got {:?}", node.kind),
        }
        let link_text = node.children.unwrap().remove(0);
        let text = link_text.unwrap_text();
        assert_eq!(text.value, "big cat");
    }

    mod link_reference {
        use super::*;
        use crate::ReferenceKind;

        #[test]
        fn test_shortcut_reference() {
            let markdown = "[kot]

[kot]: https://cat.com";
            let root = Node::try_from_str(markdown).unwrap();
            let paragraph = root.children.unwrap().remove(0);
            let node = paragraph.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 6, 5)));
            let rererence = node.unwrap_link_reference();
            assert_eq!(rererence.label, "kot");
            assert_eq!(rererence.reference_kind, ReferenceKind::Shortcut);
        }

        #[test]
        fn test_collapsed_reference() {
            let markdown = "[kot][]

[kot]: https://cat.com";
            let root = Node::try_from_str(markdown).unwrap();
            let paragraph = root.children.unwrap().remove(0);
            let node = paragraph.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 8, 7)));
            let rererence = node.unwrap_link_reference();
            assert_eq!(rererence.label, "kot");
            assert_eq!(rererence.reference_kind, ReferenceKind::Collapsed);
        }

        #[test]
        fn test_full_reference() {
            let markdown = "[nasty cat][kot]

[kot]: https://cat.com";
            let root = Node::try_from_str(markdown).unwrap();
            let paragraph = root.children.unwrap().remove(0);
            let node = paragraph.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 17, 16)));
            let rererence = node.unwrap_link_reference();
            assert_eq!(rererence.label, "kot");
            assert_eq!(rererence.reference_kind, ReferenceKind::Full);
        }
    }

    mod list {
        use super::*;

        #[test]
        fn test_ordered() {
            let markdown = "1. First item";
            let root = Node::try_from_str(markdown).unwrap();
            let node = root.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 14, 13)));
            let list = node.unwrap_list();
            assert_eq!(list.kind, ListKind::Ordered(1));
        }

        #[test]
        fn test_starting_at_5() {
            let markdown = "5. First item";
            let root = Node::try_from_str(markdown).unwrap();
            let node = root.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 14, 13)));
            let list = node.unwrap_list();
            assert_eq!(list.kind, ListKind::Ordered(5));
        }

        #[test]
        fn test_unordered() {
            let markdown = "- Item";
            let root = Node::try_from_str(markdown).unwrap();
            let node = root.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 7, 6)));
            let list = node.unwrap_list();
            assert_eq!(list.kind, ListKind::Unordered);
        }
    }

    mod list_item {
        use super::*;

        #[test]
        fn test_without_checkbox() {
            let markdown = "- Item";
            let root = Node::try_from_str(markdown).unwrap();
            let list = root.children.unwrap().remove(0);
            let node = list.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 7, 6)));
            let list_item = node.unwrap_list_item();
            assert_eq!(list_item.checked, None);
        }

        #[test]
        fn test_unchecked() {
            let markdown = "- [ ] Item";
            let root = Node::try_from_str(markdown).unwrap();
            let list = root.children.unwrap().remove(0);
            let node = list.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 11, 10)));
            let list_item = node.unwrap_list_item();
            assert_eq!(list_item.checked, Some(false));
        }

        #[test]
        fn test_checked() {
            let markdown = "- [x] Item";
            let root = Node::try_from_str(markdown).unwrap();
            let list = root.children.unwrap().remove(0);
            let node = list.children.unwrap().remove(0);
            assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 11, 10)));
            let list_item = node.unwrap_list_item();
            assert_eq!(list_item.checked, Some(true));
        }
    }

    #[test]
    fn test_math() {
        let markdown = "$$
\\sqrt{3x-1}
$$";
        let root = Node::try_from_str(markdown).unwrap();
        let node = root.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (3, 3, 17)));
        let math = node.unwrap_math();
        assert_eq!(math.value, "\\sqrt{3x-1}");
    }

    #[test]
    fn test_strong() {
        let markdown = "**strong text**";
        let root = Node::try_from_str(&markdown).unwrap();
        let paragraph = root.children.unwrap().remove(0);
        let node = paragraph.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 16, 15)));
        assert_eq!(node.kind, NodeKind::Strong);
    }

    #[test]
    fn test_strike_through() {
        let markdown = "~~strike through~~";
        let root = Node::try_from_str(&markdown).unwrap();
        let paragraph = root.children.unwrap().remove(0);
        let node = paragraph.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 19, 18)));
        assert_eq!(node.kind, NodeKind::StrikeThrough);
    }

    mod table {
        use super::*;

        #[test]
        fn test_no_alignment() {
            let markdown = "
| header |
| ------ |
";
            let root = Node::try_from_str(markdown).unwrap();
            let table = root.children.unwrap().remove(0);
            assert_eq!(table.location, location::Span::new((2, 1, 1), (3, 11, 22)));
            let table = table.unwrap_table();
            assert_eq!(table.alignments, vec![AlignKind::None]);
        }

        #[test]
        fn test_align_left() {
            let markdown = "
| header |
| :----- |
";
            let root = Node::try_from_str(markdown).unwrap();
            let table = root.children.unwrap().remove(0);
            assert_eq!(table.location, location::Span::new((2, 1, 1), (3, 11, 22)));
            let table = table.unwrap_table();
            assert_eq!(table.alignments, vec![AlignKind::Left]);
        }

        #[test]
        fn test_align_right() {
            let markdown = "
| header |
| -----: |
";
            let root = Node::try_from_str(markdown).unwrap();
            let table = root.children.unwrap().remove(0);
            assert_eq!(table.location, location::Span::new((2, 1, 1), (3, 11, 22)));
            let table = table.unwrap_table();
            assert_eq!(table.alignments, vec![AlignKind::Right]);
        }

        #[test]
        fn test_align_center() {
            let markdown = "
| header |
| :----: |
";
            let root = Node::try_from_str(markdown).unwrap();
            let table = root.children.unwrap().remove(0);
            assert_eq!(table.location, location::Span::new((2, 1, 1), (3, 11, 22)));
            let table = table.unwrap_table();
            assert_eq!(table.alignments, vec![AlignKind::Center]);
        }
    }

    #[test]
    fn test_text() {
        let markdown = "Some text here.";
        let root = Node::try_from_str(markdown).unwrap();
        let paragraph = root.children.unwrap().remove(0);
        let node = paragraph.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 16, 15)));
        let text = node.unwrap_text();
        assert_eq!(text.value, "Some text here.");
    }

    #[test]
    fn test_thematic_break() {
        let markdown = "---";
        let root = Node::try_from_str(markdown).unwrap();
        let node = root.children.unwrap().remove(0);
        assert_eq!(node.location, location::Span::new((1, 1, 0), (1, 4, 3)));
        assert_eq!(node.kind, NodeKind::ThematicBreak);
    }
}
