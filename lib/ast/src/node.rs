use std::ops::Range;

use pulldown_cmark::{
    Alignment, BlockQuoteKind, CodeBlockKind, CowStr, Event, HeadingLevel, LinkType,
    MetadataBlockKind, OffsetIter, Tag, TagEnd,
};

use crate::markdown::MarkdownItem;

/// The Ast node type.
///
/// It is a 1-to-1 mapping of [pulldown_cmark::Event], where [Tag] events are flattened
/// and their children eagerly accumulated as [Vec]s.
///
/// Each variant maps to a named struct that can be used in client code so that, for example,
/// a function can receive a [Heading] directly instead of a [Node].
#[derive(Debug, Clone, PartialEq)]
pub enum Node<'source> {
    // Internal nodes. Those map to pulldown_cmark's "Tag" events.
    /// A node corresponding to a [Tag::BlockQuote] event.
    BlockQuote(BlockQuote<'source>),
    /// A node corresponding to a [Tag::CodeBlock] event.
    CodeBlock(CodeBlock<'source>),
    /// A node corresponding to a [Tag::DefinitionList] event.
    DefinitionList(DefinitionList<'source>),
    /// A node corresponding to a [Tag::DefinitionListTitle] event.
    DefinitionListTitle(DefinitionListTitle<'source>),
    /// A node corresponding to a [Tag::DefinitionListDefinition] event.
    DefinitionListDefinition(DefinitionListDefinition<'source>),
    /// A node corresponding to a [Tag::Emphasis] event.
    Emphasis(Emphasis<'source>),
    /// A node corresponding to a [Tag::FootnoteDefinition] event.
    FootnoteDefinition(FootnoteDefinition<'source>),
    /// A node corresponding to a [Tag::Heading] event.
    Heading(Heading<'source>),
    /// A node corresponding to a [Tag::HtmlBlock] event.
    HtmlBlock(HtmlBlock<'source>),
    /// A node corresponding to a [Tag::Image] event.
    Image(Image<'source>),
    /// A node corresponding to a [Tag::Item] event.
    Item(Item<'source>),
    /// A node corresponding to a [Tag::Link] event.
    Link(Link<'source>),
    /// A node corresponding to a [Tag::List] event.
    List(List<'source>),
    /// A node corresponding to a [Tag::MetadataBlock] event.
    MetadataBlock(MetadataBlock<'source>),
    /// A node corresponding to a [Tag::Paragraph] event.
    Paragraph(Paragraph<'source>),
    /// A node corresponding to a [Tag::Strong] event.
    Strong(Strong<'source>),
    /// A node corresponding to a [Tag::Strikethrough] event.
    Strikethrough(Strikethrough<'source>),
    /// A node corresponding to a [Tag::Subscript] event.
    Subscript(Subscript<'source>),
    /// A node corresponding to a [Tag::Superscript] event.
    Superscript(Superscript<'source>),
    /// A node corresponding to a [Tag::Table] event.
    Table(Table<'source>),
    /// A node corresponding to a [Tag::TableCell] event.
    TableCell(TableCell<'source>),
    /// A node corresponding to a [Tag::TableHead] event.
    TableHead(TableHead<'source>),
    /// A node corresponding to a [Tag::TableRow] event.
    TableRow(TableRow<'source>),

    // Leaf nodes. Those map to pulldown_cmark events outside of "Tag"s.
    /// A leaf node corresponding to an [Event::Code] event.
    Code(Code<'source>),
    /// A leaf node corresponding to an [Event::DisplayMath] event.
    DisplayMath(DisplayMath<'source>),
    /// A leaf node corresponding to an [Event::FootnoteReference] event.
    FootnoteReference(FootnoteReference<'source>),
    /// A leaf node corresponding to an [Event::HardBreak] event.
    HardBreak(HardBreak),
    // Special case event that is always a child of an HtmlBlock.
    /// A leaf node corresponding to an [Event::Html] event.
    Html(Html<'source>),
    /// A leaf node corresponding to an [Event::InlineHtml] event.
    InlineHtml(InlineHtml<'source>),
    /// A leaf node corresponding to an [Event::InlineMath] event.
    InlineMath(InlineMath<'source>),
    /// A leaf node corresponding to an [Event::Rule] event.
    Rule(Rule),
    /// A leaf node corresponding to an [Event::SoftBreak] event.
    SoftBreak(SoftBreak),
    /// A leaf node corresponding to an [Event::TaskListMarker] event.
    TaskListMarker(TaskListMarker),
    /// A leaf node corresponding to an [Event::Text] event.
    Text(Text<'source>),
}

impl<'source> Node<'source> {
    /// Returns whether self is [Self::BlockQuote].
    pub fn is_block_quote(&self) -> bool {
        matches!(self, Self::BlockQuote(_))
    }

    /// Returns whether self is [Self::CodeBlock].
    pub fn is_code_block(&self) -> bool {
        matches!(self, Self::CodeBlock(_))
    }

    /// Returns whether self is [Self::DefinitionList].
    pub fn is_definition_list(&self) -> bool {
        matches!(self, Self::DefinitionList(_))
    }

    /// Returns whether self is [Self::DefinitionListTitle].
    pub fn is_definition_list_title(&self) -> bool {
        matches!(self, Self::DefinitionListTitle(_))
    }

    /// Returns whether self is [Self::DefinitionListDefinition].
    pub fn is_definition_list_definition(&self) -> bool {
        matches!(self, Self::DefinitionListDefinition(_))
    }

    /// Returns whether self is [Self::Emphasis].
    pub fn is_emphasis(&self) -> bool {
        matches!(self, Self::Emphasis(_))
    }

    /// Returns whether self is [Self::FootnoteDefinition].
    pub fn is_footnote_definition(&self) -> bool {
        matches!(self, Self::FootnoteDefinition(_))
    }

    /// Returns whether self is [Self::Heading].
    pub fn is_heading(&self) -> bool {
        matches!(self, Self::Heading(_))
    }

    /// Returns whether self is [Self::HtmlBlock].
    pub fn is_html_block(&self) -> bool {
        matches!(self, Self::HtmlBlock(_))
    }

    /// Returns whether self is [Self::Image].
    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image(_))
    }

    /// Returns whether self is [Self::Item].
    pub fn is_item(&self) -> bool {
        matches!(self, Self::Item(_))
    }

    /// Returns whether self is [Self::Link].
    pub fn is_link(&self) -> bool {
        matches!(self, Self::Link(_))
    }

    /// Returns whether self is [Self::List].
    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    /// Returns whether self is [Self::MetadataBlock].
    pub fn is_metadata_block(&self) -> bool {
        matches!(self, Self::MetadataBlock(_))
    }

    /// Returns whether self is [Self::Paragraph].
    pub fn is_paragraph(&self) -> bool {
        matches!(self, Self::Paragraph(_))
    }

    /// Returns whether self is [Self::Strong].
    pub fn is_strong(&self) -> bool {
        matches!(self, Self::Strong(_))
    }

    /// Returns whether self is [Self::Strikethrough].
    pub fn is_strikethrough(&self) -> bool {
        matches!(self, Self::Strikethrough(_))
    }

    /// Returns whether self is [Self::Subscript].
    pub fn is_subscript(&self) -> bool {
        matches!(self, Self::Subscript(_))
    }

    /// Returns whether self is [Self::Superscript].
    pub fn is_superscript(&self) -> bool {
        matches!(self, Self::Superscript(_))
    }

    /// Returns whether self is [Self::Table].
    pub fn is_table(&self) -> bool {
        matches!(self, Self::Table(_))
    }

    /// Returns whether self is [Self::TableCell].
    pub fn is_table_cell(&self) -> bool {
        matches!(self, Self::TableCell(_))
    }

    /// Returns whether self is [Self::TableHead].
    pub fn is_table_head(&self) -> bool {
        matches!(self, Self::TableHead(_))
    }

    /// Returns whether self is [Self::TableRow].
    pub fn is_table_row(&self) -> bool {
        matches!(self, Self::TableRow(_))
    }

    /// Returns whether self is [Self::Code].
    pub fn is_code(&self) -> bool {
        matches!(self, Node::Code(_))
    }

    /// Returns whether self is [Self::DisplayMath].
    pub fn is_display_math(&self) -> bool {
        matches!(self, Self::DisplayMath(_))
    }

    /// Returns whether self is [Self::FootnoteReference].
    pub fn is_footnote_reference(&self) -> bool {
        matches!(self, Self::FootnoteReference(_))
    }

    /// Returns whether self is [Self::HardBreak].
    pub fn is_hardbreak(&self) -> bool {
        matches!(self, Self::HardBreak(_))
    }

    /// Returns whether self is [Self::Html].
    pub fn is_html(&self) -> bool {
        matches!(self, Self::Html(_))
    }

    /// Returns whether self is [Self::InlineHtml].
    pub fn is_inline_html(&self) -> bool {
        matches!(self, Self::InlineHtml(_))
    }

    /// Returns whether self is [Self::InlineMath].
    pub fn is_inline_math(&self) -> bool {
        matches!(self, Node::InlineMath(_))
    }

    /// Returns whether self is [Self::Rule].
    pub fn is_rule(&self) -> bool {
        matches!(self, Self::Rule(_))
    }

    /// Returns whether self is [Self::SoftBreak].
    pub fn is_softbreak(&self) -> bool {
        matches!(self, Self::SoftBreak(_))
    }

    /// Returns whether self is [Self::TaskListMarker].
    pub fn is_task_list_marker(&self) -> bool {
        matches!(self, Self::TaskListMarker(_))
    }

    /// Returns whether self is [Self::Text].
    pub fn is_text(&self) -> bool {
        matches!(self, Node::Text(_))
    }

    /// Returns the range that the node's text spans within the source text.
    pub fn range(&self) -> &Range<usize> {
        match self {
            Node::BlockQuote(inner) => &inner.range,
            Node::CodeBlock(inner) => &inner.range,
            Node::DefinitionList(inner) => &inner.range,
            Node::DefinitionListTitle(inner) => &inner.range,
            Node::DefinitionListDefinition(inner) => &inner.range,
            Node::Emphasis(inner) => &inner.range,
            Node::FootnoteDefinition(inner) => &inner.range,
            Node::Heading(inner) => &inner.range,
            Node::HtmlBlock(inner) => &inner.range,
            Node::Image(inner) => &inner.range,
            Node::Item(inner) => &inner.range,
            Node::Link(inner) => &inner.range,
            Node::List(inner) => &inner.range,
            Node::MetadataBlock(inner) => &inner.range,
            Node::Paragraph(inner) => &inner.range,
            Node::Strong(inner) => &inner.range,
            Node::Strikethrough(inner) => &inner.range,
            Node::Subscript(inner) => &inner.range,
            Node::Superscript(inner) => &inner.range,
            Node::Table(inner) => &inner.range,
            Node::TableCell(inner) => &inner.range,
            Node::TableHead(inner) => &inner.range,
            Node::TableRow(inner) => &inner.range,
            Node::Code(inner) => &inner.range,
            Node::DisplayMath(inner) => &inner.range,
            Node::FootnoteReference(inner) => &inner.range,
            Node::HardBreak(inner) => &inner.range,
            Node::Html(inner) => &inner.range,
            Node::InlineHtml(inner) => &inner.range,
            Node::InlineMath(inner) => &inner.range,
            Node::Rule(inner) => &inner.range,
            Node::SoftBreak(inner) => &inner.range,
            Node::TaskListMarker(inner) => &inner.range,
            Node::Text(inner) => &inner.range,
        }
    }

    /// Unwraps the inner [BlockQuote] if self is [Self::BlockQuote].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::BlockQuote].
    pub fn unwrap_block_quote(self) -> BlockQuote<'source> {
        match self {
            Self::BlockQuote(inner) => inner,
            _ => panic!("cannot unwrap block quote on {:?}", self),
        }
    }

    /// Unwraps the inner [CodeBlock] if self is [Self::CodeBlock].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::CodeBlock].
    pub fn unwrap_code_block(self) -> CodeBlock<'source> {
        match self {
            Self::CodeBlock(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [DefinitionList] if self is [Self::DefinitionList].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::DefinitionList].
    pub fn unwrap_definition_list(self) -> DefinitionList<'source> {
        match self {
            Self::DefinitionList(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [DefinitionListTitle] if self is [Self::DefinitionListTitle].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::DefinitionListTitle].
    pub fn unwrap_definition_list_title(self) -> DefinitionListTitle<'source> {
        match self {
            Self::DefinitionListTitle(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [DefinitionListDefinition] if self is [Self::DefinitionListDefinition].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::DefinitionListDefinition].
    pub fn unwrap_definition_list_definition(self) -> DefinitionListDefinition<'source> {
        match self {
            Self::DefinitionListDefinition(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Emphasis] if self is [Self::Emphasis].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Emphasis].
    pub fn unwrap_emphasis(self) -> Emphasis<'source> {
        match self {
            Self::Emphasis(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [FootnoteDefinition] if self is [Self::FootnoteDefinition].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::FootnoteDefinition].
    pub fn unwrap_footnote_definition(self) -> FootnoteDefinition<'source> {
        match self {
            Self::FootnoteDefinition(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Heading] if self is [Self::Heading].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Heading].
    pub fn unwrap_heading(self) -> Heading<'source> {
        match self {
            Self::Heading(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [HtmlBlock] if self is [Self::HtmlBlock].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::HtmlBlock].
    pub fn unwrap_html_block(self) -> HtmlBlock<'source> {
        match self {
            Self::HtmlBlock(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Image] if self is [Self::Image].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Image].
    pub fn unwrap_image(self) -> Image<'source> {
        match self {
            Self::Image(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Item] if self is [Self::Item].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Item].
    pub fn unwrap_item(self) -> Item<'source> {
        match self {
            Self::Item(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Link] if self is [Self::Link].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Link].
    pub fn unwrap_link(self) -> Link<'source> {
        match self {
            Self::Link(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [List] if self is [Self::List].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::List].
    pub fn unwrap_list(self) -> List<'source> {
        match self {
            Self::List(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [MetadataBlock] if self is [Self::MetadataBlock].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::MetadataBlock].
    pub fn unwrap_metadata_block(self) -> MetadataBlock<'source> {
        match self {
            Self::MetadataBlock(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Paragraph] if self is [Self::Paragraph].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Paragraph].
    pub fn unwrap_paragraph(self) -> Paragraph<'source> {
        match self {
            Self::Paragraph(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Strong] if self is [Self::Strong].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Strong].
    pub fn unwrap_strong(self) -> Strong<'source> {
        match self {
            Self::Strong(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Strikethrough] if self is [Self::Strikethrough].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Strikethrough].
    pub fn unwrap_strikethrough(self) -> Strikethrough<'source> {
        match self {
            Self::Strikethrough(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Subscript] if self is [Self::Subscript].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Subscript].
    pub fn unwrap_subscript(self) -> Subscript<'source> {
        match self {
            Self::Subscript(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Superscript] if self is [Self::Superscript].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Superscript].
    pub fn unwrap_superscript(self) -> Superscript<'source> {
        match self {
            Self::Superscript(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Table] if self is [Self::Table].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Table].
    pub fn unwrap_table(self) -> Table<'source> {
        match self {
            Self::Table(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [TableCell] if self is [Self::TableCell].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::TableCell].
    pub fn unwrap_table_cell(self) -> TableCell<'source> {
        match self {
            Self::TableCell(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [TableHead] if self is [Self::TableHead].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::TableHead].
    pub fn unwrap_table_head(self) -> TableHead<'source> {
        match self {
            Self::TableHead(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [TableRow] if self is [Self::TableRow].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::TableRow].
    pub fn unwrap_table_row(self) -> TableRow<'source> {
        match self {
            Self::TableRow(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Code] if self is [Self::Code].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Code].
    pub fn unwrap_code(self) -> Code<'source> {
        match self {
            Node::Code(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [DisplayMath] if self is [Self::DisplayMath].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::DisplayMath].
    pub fn unwrap_display_math(self) -> DisplayMath<'source> {
        match self {
            Self::DisplayMath(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [FootnoteReference] if self is [Self::FootnoteReference].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::FootnoteReference].
    pub fn unwrap_footnote_reference(self) -> FootnoteReference<'source> {
        match self {
            Self::FootnoteReference(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [HardBreak] if self is [Self::HardBreak].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::HardBreak].
    pub fn unwrap_hardbreak(self) -> HardBreak {
        match self {
            Self::HardBreak(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Html] if self is [Self::Html].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Html].
    pub fn unwrap_html(self) -> Html<'source> {
        match self {
            Self::Html(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [InlineHtml] if self is [Self::InlineHtml].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::InlineHtml].
    pub fn unwrap_inline_html(self) -> InlineHtml<'source> {
        match self {
            Self::InlineHtml(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [InlineMath] if self is [Self::InlineMath].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::InlineMath].
    pub fn unwrap_inline_math(self) -> InlineMath<'source> {
        match self {
            Node::InlineMath(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Rule] if self is [Self::Rule].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Rule].
    pub fn unwrap_rule(self) -> Rule {
        match self {
            Self::Rule(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [SoftBreak] if self is [Self::SoftBreak].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::SoftBreak].
    pub fn unwrap_softbreak(self) -> SoftBreak {
        match self {
            Self::SoftBreak(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [TaskListMarker] if self is [Self::TaskListMarker].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::TaskListMarker].
    pub fn unwrap_task_list_marker(self) -> TaskListMarker {
        match self {
            Self::TaskListMarker(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    /// Unwraps the inner [Text] if self is [Self::Text].
    ///
    /// # Panics
    ///
    /// Panics if self is not [Self::Text].
    pub fn unwrap_text(self) -> Text<'source> {
        match self {
            Node::Text(inner) => inner,
            _ => panic!("cannot unwrap X on {:?}", self),
        }
    }

    pub(crate) fn consume_one(head: MarkdownItem<'source>, iter: &mut OffsetIter<'source>) -> Self {
        let range = head.1;
        match head.0 {
            Event::Start(tag) => {
                Self::from_tag_parts(range, Node::collect_until(tag.to_end(), iter), tag)
            }
            Event::Text(text) => Text::new(range, text).into(),
            Event::Code(text) => Code::new(range, text).into(),
            Event::InlineMath(text) => InlineMath::new(range, text).into(),
            Event::DisplayMath(text) => DisplayMath::new(range, text).into(),
            Event::Html(text) => Html::new(range, text).into(),
            Event::InlineHtml(text) => InlineHtml::new(range, text).into(),
            Event::FootnoteReference(text) => FootnoteReference::new(range, text).into(),
            Event::SoftBreak => SoftBreak::new(range).into(),
            Event::HardBreak => HardBreak::new(range).into(),
            Event::Rule => Rule::new(range).into(),
            Event::TaskListMarker(checked) => TaskListMarker::new(range, checked).into(),
            Event::End(end) => panic!("unexpected tag end {:?}", end),
        }
    }

    // TODO: could make a lazy iter instead, and collect outside.
    pub(crate) fn collect_until(until: TagEnd, iter: &mut OffsetIter<'source>) -> Vec<Self> {
        let mut result = vec![];

        while let Some(item) = iter.next() {
            if let Event::End(end) = item.0
                && end == until
            {
                return result;
            }

            result.push(Node::consume_one(item, iter));
        }

        unreachable!(
            "haven't reached expected end {:?} before end of input mfk!!!",
            until
        );
    }

    fn from_tag_parts(
        range: Range<usize>,
        children: Vec<Node<'source>>,
        tag: Tag<'source>,
    ) -> Self {
        match tag {
            Tag::Paragraph => Paragraph::new(range, children).into(),
            Tag::Heading {
                level,
                id,
                classes,
                attrs,
            } => Heading::new_with_attributes(range, children, level, id, classes, attrs).into(),
            Tag::BlockQuote(kind) => BlockQuote::new(range, children, kind).into(),
            Tag::CodeBlock(kind) => CodeBlock::new(range, children, kind).into(),
            Tag::HtmlBlock => HtmlBlock::new(range, children).into(),
            Tag::List(first) => List::new(range, children, first).into(),
            Tag::Item => Item::new(range, children).into(),
            Tag::FootnoteDefinition(label) => {
                FootnoteDefinition::new(range, children, label).into()
            }
            Tag::DefinitionList => DefinitionList::new(range, children).into(),
            Tag::DefinitionListTitle => DefinitionListTitle::new(range, children).into(),
            Tag::DefinitionListDefinition => DefinitionListDefinition::new(range, children).into(),
            Tag::Table(alignments) => Table::new(range, children, alignments).into(),
            Tag::TableHead => TableHead::new(range, children).into(),
            Tag::TableRow => TableRow::new(range, children).into(),
            Tag::TableCell => TableCell::new(range, children).into(),
            Tag::Emphasis => Emphasis::new(range, children).into(),
            Tag::Strong => Strong::new(range, children).into(),
            Tag::Strikethrough => Strikethrough::new(range, children).into(),
            Tag::Superscript => Superscript::new(range, children).into(),
            Tag::Subscript => Subscript::new(range, children).into(),
            Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            } => Link::new(range, children, dest_url, id, link_type, title).into(),
            Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            } => Image::new(range, children, dest_url, id, link_type, title).into(),
            Tag::MetadataBlock(kind) => MetadataBlock::new(range, children, kind).into(),
        }
    }
}

impl<'source> From<BlockQuote<'source>> for Node<'source> {
    fn from(value: BlockQuote<'source>) -> Self {
        Self::BlockQuote(value)
    }
}

impl<'source> From<CodeBlock<'source>> for Node<'source> {
    fn from(value: CodeBlock<'source>) -> Self {
        Self::CodeBlock(value)
    }
}

impl<'source> From<DefinitionList<'source>> for Node<'source> {
    fn from(value: DefinitionList<'source>) -> Self {
        Self::DefinitionList(value)
    }
}

impl<'source> From<DefinitionListTitle<'source>> for Node<'source> {
    fn from(value: DefinitionListTitle<'source>) -> Self {
        Self::DefinitionListTitle(value)
    }
}

impl<'source> From<DefinitionListDefinition<'source>> for Node<'source> {
    fn from(value: DefinitionListDefinition<'source>) -> Self {
        Self::DefinitionListDefinition(value)
    }
}

impl<'source> From<Emphasis<'source>> for Node<'source> {
    fn from(value: Emphasis<'source>) -> Self {
        Self::Emphasis(value)
    }
}

impl<'source> From<FootnoteDefinition<'source>> for Node<'source> {
    fn from(value: FootnoteDefinition<'source>) -> Self {
        Self::FootnoteDefinition(value)
    }
}

impl<'source> From<Heading<'source>> for Node<'source> {
    fn from(value: Heading<'source>) -> Self {
        Self::Heading(value)
    }
}

impl<'source> From<HtmlBlock<'source>> for Node<'source> {
    fn from(value: HtmlBlock<'source>) -> Self {
        Self::HtmlBlock(value)
    }
}

impl<'source> From<Image<'source>> for Node<'source> {
    fn from(value: Image<'source>) -> Self {
        Self::Image(value)
    }
}

impl<'source> From<Item<'source>> for Node<'source> {
    fn from(value: Item<'source>) -> Self {
        Self::Item(value)
    }
}

impl<'source> From<Link<'source>> for Node<'source> {
    fn from(value: Link<'source>) -> Self {
        Self::Link(value)
    }
}

impl<'source> From<List<'source>> for Node<'source> {
    fn from(value: List<'source>) -> Self {
        Self::List(value)
    }
}

impl<'source> From<MetadataBlock<'source>> for Node<'source> {
    fn from(value: MetadataBlock<'source>) -> Self {
        Self::MetadataBlock(value)
    }
}

impl<'source> From<Paragraph<'source>> for Node<'source> {
    fn from(value: Paragraph<'source>) -> Self {
        Self::Paragraph(value)
    }
}

impl<'source> From<Strong<'source>> for Node<'source> {
    fn from(value: Strong<'source>) -> Self {
        Self::Strong(value)
    }
}

impl<'source> From<Strikethrough<'source>> for Node<'source> {
    fn from(value: Strikethrough<'source>) -> Self {
        Self::Strikethrough(value)
    }
}

impl<'source> From<Subscript<'source>> for Node<'source> {
    fn from(value: Subscript<'source>) -> Self {
        Self::Subscript(value)
    }
}

impl<'source> From<Superscript<'source>> for Node<'source> {
    fn from(value: Superscript<'source>) -> Self {
        Self::Superscript(value)
    }
}

impl<'source> From<Table<'source>> for Node<'source> {
    fn from(value: Table<'source>) -> Self {
        Self::Table(value)
    }
}

impl<'source> From<TableCell<'source>> for Node<'source> {
    fn from(value: TableCell<'source>) -> Self {
        Self::TableCell(value)
    }
}

impl<'source> From<TableHead<'source>> for Node<'source> {
    fn from(value: TableHead<'source>) -> Self {
        Self::TableHead(value)
    }
}

impl<'source> From<TableRow<'source>> for Node<'source> {
    fn from(value: TableRow<'source>) -> Self {
        Self::TableRow(value)
    }
}

impl<'source> From<Code<'source>> for Node<'source> {
    fn from(value: Code<'source>) -> Self {
        Self::Code(value)
    }
}

impl<'source> From<DisplayMath<'source>> for Node<'source> {
    fn from(value: DisplayMath<'source>) -> Self {
        Self::DisplayMath(value)
    }
}

impl<'source> From<FootnoteReference<'source>> for Node<'source> {
    fn from(value: FootnoteReference<'source>) -> Self {
        Self::FootnoteReference(value)
    }
}

impl<'source> From<HardBreak> for Node<'source> {
    fn from(value: HardBreak) -> Self {
        Self::HardBreak(value)
    }
}

impl<'source> From<Html<'source>> for Node<'source> {
    fn from(value: Html<'source>) -> Self {
        Self::Html(value)
    }
}

impl<'source> From<InlineHtml<'source>> for Node<'source> {
    fn from(value: InlineHtml<'source>) -> Self {
        Self::InlineHtml(value)
    }
}

impl<'source> From<InlineMath<'source>> for Node<'source> {
    fn from(value: InlineMath<'source>) -> Self {
        Self::InlineMath(value)
    }
}

impl<'source> From<Rule> for Node<'source> {
    fn from(value: Rule) -> Self {
        Self::Rule(value)
    }
}

impl<'source> From<SoftBreak> for Node<'source> {
    fn from(value: SoftBreak) -> Self {
        Self::SoftBreak(value)
    }
}

impl<'source> From<TaskListMarker> for Node<'source> {
    fn from(value: TaskListMarker) -> Self {
        Self::TaskListMarker(value)
    }
}

impl<'source> From<Text<'source>> for Node<'source> {
    fn from(value: Text<'source>) -> Self {
        Self::Text(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod is_block_quote {
        use super::*;

        #[test]
        fn should_return_false_when_not_block_quote() {
            assert!(!Node::from(Text::new(1..5, CowStr::from("toto"))).is_block_quote())
        }

        #[test]
        fn should_return_true_when_block_quote() {
            assert!(
                Node::from(BlockQuote::new(
                    1..5,
                    vec![Text::new(1..5, CowStr::from("toto")).into()],
                    None
                ))
                .is_block_quote()
            )
        }
    }

    mod is_code_block {
        use super::*;

        #[test]
        fn should_return_false_when_not_code_block() {
            assert!(!Node::from(Text::new(1..5, CowStr::from("toto"))).is_code_block())
        }

        #[test]
        fn should_return_true_when_code_block() {
            assert!(
                Node::from(CodeBlock::new(
                    1..5,
                    vec![Text::new(1..5, CowStr::from("toto")).into()],
                    CodeBlockKind::Indented
                ))
                .is_code_block()
            )
        }
    }

    mod is_definition_list {
        use super::*;

        #[test]
        fn should_return_false_when_not_definition_list() {
            assert!(!Node::from(Text::new(1..5, CowStr::from("toto"))).is_definition_list())
        }

        #[test]
        fn should_return_true_when_definition_list() {
            assert!(
                Node::from(DefinitionList::new(
                    1..5,
                    vec![Text::new(1..5, CowStr::from("toto")).into()],
                ))
                .is_definition_list()
            )
        }
    }

    mod is_definition_list_title {
        use super::*;

        #[test]
        fn should_return_false_when_not_definition_list_title() {
            assert!(!Node::from(Text::new(1..5, CowStr::from("toto"))).is_definition_list_title())
        }

        #[test]
        fn should_return_true_when_definition_list_title() {
            assert!(
                Node::from(DefinitionListTitle::new(
                    1..5,
                    vec![Text::new(1..5, CowStr::from("toto")).into()],
                ))
                .is_definition_list_title()
            )
        }
    }

    mod is_definition_list_definition {
        use super::*;

        #[test]
        fn should_return_false_when_not_definition_list_definition() {
            assert!(
                !Node::from(Text::new(1..5, CowStr::from("toto"))).is_definition_list_definition()
            )
        }

        #[test]
        fn should_return_true_when_definition_list_definition() {
            assert!(
                Node::from(DefinitionListDefinition::new(
                    1..5,
                    vec![Text::new(1..5, CowStr::from("toto")).into()],
                ))
                .is_definition_list_definition()
            )
        }
    }

    mod is_emphasis {
        use super::*;

        #[test]
        fn should_return_false_when_not_emphasis() {
            assert!(!Node::from(Text::new(1..5, CowStr::from("toto"))).is_emphasis())
        }

        #[test]
        fn should_return_true_when_emphasis() {
            assert!(
                Node::from(Emphasis::new(
                    1..5,
                    vec![Text::new(1..5, CowStr::from("toto")).into()],
                ))
                .is_emphasis()
            )
        }
    }

    mod is_footnote_definition {
        use super::*;

        #[test]
        fn should_return_false_when_not_footnote_definition() {
            assert!(!Node::from(Text::new(1..5, CowStr::from("toto"))).is_footnote_definition())
        }

        #[test]
        fn should_return_true_when_footnote_definition() {
            assert!(
                Node::from(FootnoteDefinition::new(
                    1..5,
                    vec![Text::new(1..5, CowStr::from("toto")).into()],
                    CowStr::from("label-biatch")
                ))
                .is_footnote_definition()
            )
        }
    }

    mod is_heading {
        use super::*;

        #[test]
        fn should_return_false_when_not_heading() {
            assert!(!Node::from(Text::new(1..5, CowStr::from("toto"))).is_heading())
        }

        #[test]
        fn should_return_true_when_heading() {
            assert!(
                Node::from(Heading::new(
                    1..5,
                    vec![Text::new(1..5, CowStr::from("toto")).into()],
                    HeadingLevel::H1
                ))
                .is_heading()
            )
        }
    }

    // TODO: finish the is_ and unwrap_ tests!!!!

    mod is_code {
        use super::*;

        #[test]
        fn should_return_false_for_not_code() {
            assert!(!Node::Text(Text::new(1..5, CowStr::from("toto"))).is_code())
        }

        #[test]
        fn should_return_true_for_code() {
            assert!(Node::Code(Code::new(1..5, CowStr::from("toto"))).is_code())
        }
    }

    mod is_text {
        use super::*;

        #[test]
        fn should_return_false_for_not_text() {
            assert!(!Node::Code(Code::new(1..5, CowStr::from("toto"))).is_text())
        }

        #[test]
        fn should_return_true_for_text() {
            assert!(Node::Text(Text::new(1..5, CowStr::from("toto"))).is_text())
        }
    }

    mod unwrap_code {
        use super::*;

        #[test]
        #[should_panic]
        fn should_panic_for_not_code() {
            Node::Text(Text::new(5..10, CowStr::from("just please stfu"))).unwrap_code();
        }

        #[test]
        fn should_work_for_code() {
            let code = Code::new(4..8, CowStr::from("hello stfu"));
            let node = Node::Code(code.clone());
            assert_eq!(node.unwrap_code(), code);
        }
    }

    mod unwrap_text {
        use super::*;

        #[test]
        #[should_panic]
        fn should_panic_for_not_text() {
            Node::Code(Code::new(5..10, CowStr::from("just please stfu"))).unwrap_text();
        }

        #[test]
        fn should_work_for_text() {
            let text = Text::new(4..8, CowStr::from("hello stfu"));
            let node = Node::Text(text.clone());
            assert_eq!(node.unwrap_text(), text);
        }
    }
}

// Internal nodes definitions.

#[derive(Debug, Clone, PartialEq)]
pub struct BlockQuote<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
    pub kind: Option<BlockQuoteKind>,
}

impl<'source> BlockQuote<'source> {
    pub fn new(
        range: Range<usize>,
        children: Vec<Node<'source>>,
        kind: Option<BlockQuoteKind>,
    ) -> Self {
        Self {
            range,
            kind,
            children,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock<'source> {
    pub range: Range<usize>,
    pub kind: CodeBlockKind<'source>,
    pub children: Vec<Node<'source>>,
}

impl<'source> CodeBlock<'source> {
    pub fn new(
        range: Range<usize>,
        children: Vec<Node<'source>>,
        kind: CodeBlockKind<'source>,
    ) -> Self {
        Self {
            range,
            kind,
            children,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefinitionList<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> DefinitionList<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefinitionListTitle<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> DefinitionListTitle<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefinitionListDefinition<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> DefinitionListDefinition<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Emphasis<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> Emphasis<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FootnoteDefinition<'source> {
    pub range: Range<usize>,
    pub label: CowStr<'source>,
    pub children: Vec<Node<'source>>,
}

impl<'source> FootnoteDefinition<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>, label: CowStr<'source>) -> Self {
        Self {
            range,
            label,
            children,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Heading<'source> {
    pub range: Range<usize>,
    pub level: HeadingLevel,
    pub id: Option<CowStr<'source>>,
    pub classes: Vec<CowStr<'source>>,
    /// The first item of the tuple is the attr and second one the value.
    pub attrs: Vec<(CowStr<'source>, Option<CowStr<'source>>)>,
    pub children: Vec<Node<'source>>,
}

impl<'source> Heading<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>, level: HeadingLevel) -> Self {
        Self::new_with_attributes(
            range,
            children,
            level,
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }

    pub fn new_with_attributes(
        range: Range<usize>,
        children: Vec<Node<'source>>,
        level: HeadingLevel,
        id: Option<CowStr<'source>>,
        classes: Vec<CowStr<'source>>,
        attrs: Vec<(CowStr<'source>, Option<CowStr<'source>>)>,
    ) -> Self {
        Self {
            range,
            level,
            children,
            id,
            classes,
            attrs,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlBlock<'source> {
    pub range: Range<usize>,
    // Those are all Html varians when constructed by this module.
    pub children: Vec<Node<'source>>,
}

impl<'source> HtmlBlock<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
    pub dest_url: CowStr<'source>,
    pub id: CowStr<'source>,
    pub link_type: LinkType,
    pub title: CowStr<'source>,
}

impl<'source> Image<'source> {
    pub fn new(
        range: Range<usize>,
        children: Vec<Node<'source>>,
        dest_url: CowStr<'source>,
        id: CowStr<'source>,
        link_type: LinkType,
        title: CowStr<'source>,
    ) -> Self {
        Self {
            range,
            children,
            dest_url,
            id,
            link_type,
            title,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> Item<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
    pub dest_url: CowStr<'source>,
    pub id: CowStr<'source>,
    pub link_type: LinkType,
    pub title: CowStr<'source>,
}

impl<'source> Link<'source> {
    pub fn new(
        range: Range<usize>,
        children: Vec<Node<'source>>,
        dest_url: CowStr<'source>,
        id: CowStr<'source>,
        link_type: LinkType,
        title: CowStr<'source>,
    ) -> Self {
        Self {
            range,
            children,
            dest_url,
            id,
            link_type,
            title,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct List<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
    /// When numbered, this value holds the index of the first element.
    pub first: Option<u64>,
}

impl<'source> List<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>, first: Option<u64>) -> Self {
        Self {
            range,
            first,
            children,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetadataBlock<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
    pub kind: MetadataBlockKind,
}

impl<'source> MetadataBlock<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>, kind: MetadataBlockKind) -> Self {
        Self {
            range,
            children,
            kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> Paragraph<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Strong<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> Strong<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Strikethrough<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> Strikethrough<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Subscript<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> Subscript<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Superscript<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> Superscript<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
    pub alignments: Vec<Alignment>,
}

impl<'source> Table<'source> {
    pub fn new(
        range: Range<usize>,
        children: Vec<Node<'source>>,
        alignments: Vec<Alignment>,
    ) -> Self {
        Self {
            range,
            alignments,
            children,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCell<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> TableCell<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableHead<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> TableHead<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableRow<'source> {
    pub range: Range<usize>,
    pub children: Vec<Node<'source>>,
}

impl<'source> TableRow<'source> {
    pub fn new(range: Range<usize>, children: Vec<Node<'source>>) -> Self {
        Self { range, children }
    }
}

// Leaf nodes definitions.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code<'source> {
    pub range: Range<usize>,
    pub text: CowStr<'source>,
}

impl<'source> Code<'source> {
    pub fn new(range: Range<usize>, text: CowStr<'source>) -> Self {
        Self { range, text }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayMath<'source> {
    pub range: Range<usize>,
    pub text: CowStr<'source>,
}

impl<'source> DisplayMath<'source> {
    pub fn new(range: Range<usize>, text: CowStr<'source>) -> Self {
        Self { range, text }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FootnoteReference<'source> {
    pub range: Range<usize>,
    pub text: CowStr<'source>,
}

impl<'source> FootnoteReference<'source> {
    pub fn new(range: Range<usize>, text: CowStr<'source>) -> Self {
        Self { range, text }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardBreak {
    pub range: Range<usize>,
}

impl HardBreak {
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Html<'source> {
    pub range: Range<usize>,
    pub text: CowStr<'source>,
}

impl<'source> Html<'source> {
    pub fn new(range: Range<usize>, text: CowStr<'source>) -> Self {
        Self { range, text }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineHtml<'source> {
    pub range: Range<usize>,
    pub text: CowStr<'source>,
}

impl<'source> InlineHtml<'source> {
    pub fn new(range: Range<usize>, text: CowStr<'source>) -> Self {
        Self { range, text }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineMath<'source> {
    pub range: Range<usize>,
    pub text: CowStr<'source>,
}

impl<'source> InlineMath<'source> {
    pub fn new(range: Range<usize>, text: CowStr<'source>) -> Self {
        Self { range, text }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub range: Range<usize>,
}

impl Rule {
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoftBreak {
    pub range: Range<usize>,
}

impl SoftBreak {
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskListMarker {
    pub range: Range<usize>,
    pub checked: bool,
}

impl TaskListMarker {
    pub fn new(range: Range<usize>, checked: bool) -> Self {
        Self { range, checked }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text<'source> {
    pub range: Range<usize>,
    pub text: CowStr<'source>,
}

impl<'source> Text<'source> {
    pub fn new(range: Range<usize>, text: CowStr<'source>) -> Self {
        Self { range, text }
    }
}
