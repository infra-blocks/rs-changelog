use std::ops::Range;

use pulldown_cmark::{
    Alignment, BlockQuoteKind, CodeBlockKind, CowStr, Event, HeadingLevel, LinkType,
    MetadataBlockKind, OffsetIter, Tag,
};

use crate::{Node, markdown::MarkdownItem};

/// The internal structure of [`Node::Internal`] variants.
#[derive(Debug, Clone, PartialEq)]
pub struct Internal<'source> {
    /// The converted event from [pulldown_cmark].
    pub event: InternalEvent<'source>,
    /// The range, copied directly from the corresponding [`Event::Start`] event of this node.
    pub range: Range<usize>,
    /// The children cumulated within the matching [`Event::Start`] and [`Event::End`] of this node.
    pub children: Vec<Node<'source>>,
}

impl<'source> Internal<'source> {
    pub fn new(
        event: InternalEvent<'source>,
        range: Range<usize>,
        children: Vec<Node<'source>>,
    ) -> Self {
        Self {
            event,
            range,
            children,
        }
    }

    pub(crate) fn try_consume_one(
        head: MarkdownItem<'source>,
        iter: &mut OffsetIter<'source>,
    ) -> Result<Self, MarkdownItem<'source>> {
        let Event::Start(tag) = head.0 else {
            return Err(head);
        };

        let end = tag.to_end();
        Ok(Self::new(
            InternalEvent::from(tag),
            head.1,
            Node::collect_until(end, iter),
        ))
    }
}

/// A convenient flattening of [`Event::Start`] that are guaranteed to have children.
///
/// This type respects the structure of [`pulldown_cmark`]'s own types.
#[derive(Debug, Clone, PartialEq)]
pub enum InternalEvent<'source> {
    BlockQuote(Option<BlockQuoteKind>),
    List(Option<u64>),
    Item,
    Paragraph,
    Heading {
        level: HeadingLevel,
        id: Option<CowStr<'source>>,
        classes: Vec<CowStr<'source>>,
        /// The first item of the tuple is the attr and second one the value.
        attrs: Vec<(CowStr<'source>, Option<CowStr<'source>>)>,
    },
    CodeBlock(CodeBlockKind<'source>),
    HtmlBlock,
    FootnoteDefinition(CowStr<'source>),
    DefinitionList,
    DefinitionListTitle,
    DefinitionListDefinition,
    Table(Vec<Alignment>),
    TableHead,
    TableRow,
    TableCell,
    Emphasis,
    Strong,
    Strikethrough,
    Superscript,
    Subscript,
    Link {
        dest_url: CowStr<'source>,
        id: CowStr<'source>,
        link_type: LinkType,
        title: CowStr<'source>,
    },
    Image {
        dest_url: CowStr<'source>,
        id: CowStr<'source>,
        link_type: LinkType,
        title: CowStr<'source>,
    },
    MetadataBlock(MetadataBlockKind),
}

impl<'source> From<Tag<'source>> for InternalEvent<'source> {
    fn from(value: Tag<'source>) -> Self {
        match value {
            Tag::Paragraph => Self::Paragraph,
            Tag::Heading {
                level,
                id,
                classes,
                attrs,
            } => Self::Heading {
                level,
                id,
                classes,
                attrs,
            },
            Tag::BlockQuote(kind) => Self::BlockQuote(kind),
            Tag::CodeBlock(kind) => Self::CodeBlock(kind),
            Tag::HtmlBlock => Self::HtmlBlock,
            Tag::List(first) => Self::List(first),
            Tag::Item => Self::Item,
            Tag::FootnoteDefinition(label) => Self::FootnoteDefinition(label),
            Tag::DefinitionList => Self::DefinitionList,
            Tag::DefinitionListTitle => Self::DefinitionListTitle,
            Tag::DefinitionListDefinition => Self::DefinitionListDefinition,
            Tag::Table(alignments) => Self::Table(alignments),
            Tag::TableHead => Self::TableHead,
            Tag::TableRow => Self::TableRow,
            Tag::TableCell => Self::TableCell,
            Tag::Emphasis => Self::Emphasis,
            Tag::Strong => Self::Strong,
            Tag::Strikethrough => Self::Strikethrough,
            Tag::Superscript => Self::Superscript,
            Tag::Subscript => Self::Subscript,
            Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            } => Self::Link {
                dest_url,
                id,
                link_type,
                title,
            },
            Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            } => Self::Image {
                dest_url,
                id,
                link_type,
                title,
            },
            Tag::MetadataBlock(kind) => Self::MetadataBlock(kind),
        }
    }
}
