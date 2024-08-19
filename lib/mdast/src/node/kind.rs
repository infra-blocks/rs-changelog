use super::{AlignKind, ReferenceKind};

/// The kind of a [crate::node::Node].
///
/// It identifies the kind of the node, and contains an extra payload for certain kinds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    /// A block quote.
    /// ```text
    /// > The quote.
    /// ^^^^^^^^^^^^
    /// ```
    /// The location spans the entire block quote, including the `>`.
    BlockQuote,
    /// A code block.
    /// ~~~text
    /// ```lang
    /// code
    /// ```
    /// ~~~
    /// or
    /// ```text
    /// ~~~lang
    /// code
    /// ~~~
    /// ```
    /// The location spans across the whole block, including the symbols.
    Code,
    /// A link reference definition.
    /// ```text
    /// [label]: destination "title"
    /// ```
    // See [here](https://github.github.com/gfm/#link-reference-definition) for terminology.
    /// The location spans across all the definition, from the opening `[` to the ending `"`, if any.
    Definition(Definition),
    /// Emphasis marker.
    /// ```text
    /// *text*
    /// ```
    /// The location spans across the whole block, including symbols.
    Emphasis,
    /// A footnote definition.
    /// ```text
    /// [^label]: note
    /// ```.
    FootnoteDefinition(FootnoteDefinition),
    /// A footnote reference.
    /// ```text
    /// Some bullshit text [^a].
    ///                    ^^^^
    /// ```
    FootnoteReference(FootnoteReference),
    /// A heading.
    /// ```text
    /// # Heading 1
    /// ## Heading 2
    /// ### Heading 3
    /// #### Heading 4
    /// ##### Heading 5
    /// ###### Heading 6
    /// ```
    Heading(Heading),
    /// Raw HTML.
    /// ```text
    /// <div>bullchit</div>
    /// ```
    Html(Html),
    /// An image: `![image url](alternative text "title")`.
    Image(Image),
    /// An image reference.
    /// ```text
    /// ![identifier]
    /// ```
    ImageReference(ImageReference),
    /// Inline code.
    /// ```text
    /// `code`
    /// ```
    InlineCode(InlineCode),
    /// Inline math.
    /// ```text
    /// $math$
    /// ```
    InlineMath(InlineMath),
    /// A line break.
    /// ```text
    /// Ipsum lorem whatever the fuck \
    ///                               ^
    /// ```
    LineBreak,
    /// A link.
    /// ```text
    /// [link text](url "title")
    /// ```
    Link(Link),
    /// A link reference.
    /// ```text
    /// [identifier]
    /// ```
    LinkReference(LinkReference),
    /// A list.
    /// ```text
    /// - item 1
    /// * item 2
    /// + item 3
    /// 1. Ordered item 1
    /// 1. Ordered item 2
    /// ```
    List(List),
    /// A list item.
    /// ```text
    /// - item 1
    ///   ^^^^^^
    /// ```
    ListItem(ListItem),
    /// Math block.
    /// ```text
    /// $$
    /// math
    /// $$
    /// ```
    Math(Math),
    /// A paragraph.
    Paragraph,
    /// The root of the Markdown AST.
    Root,
    /// Strong marker.
    /// ```text
    /// **text**
    /// ```
    Strong,
    /// Strike through marker.
    /// ```text
    ///  ~~text~~
    /// ```
    StrikeThrough,
    /// A table.
    /// ```text
    /// | header |
    /// | ------ |
    /// | cell   |
    /// ```
    Table(Table),
    /// A table cell.
    /// ```text
    /// | cell_1 | cell_2 |
    /// ^^^^^^^^^^
    /// ```
    TableCell,
    /// A table row.
    /// ```text
    /// | cell_1 | cell_2 |
    /// ^^^^^^^^^^^^^^^^^^^
    /// ```
    TableRow,
    /// Some text.
    /// ```text
    /// text
    /// ```
    /// Typically nested within a [NodeKind::Paragraph].
    Text(Text),
    /// A thematic break.
    /// ```text
    /// ---
    /// ```
    /// or
    /// ```text
    /// ***
    /// ```
    /// or
    /// ```text
    /// ___
    /// ```
    /// Spaces can interleave the characters.
    ThematicBreak,
}

impl NodeKind {
    pub(crate) fn block_quote() -> Self {
        Self::BlockQuote
    }

    pub(crate) fn line_break() -> Self {
        Self::LineBreak
    }

    pub(crate) fn definition<T: Into<String>, U: Into<String>, V: Into<Option<String>>>(
        label: T,
        destination: U,
        title: V,
    ) -> Self {
        Self::Definition(Definition {
            label: label.into(),
            destination: destination.into(),
            title: title.into(),
        })
    }

    pub(crate) fn emphasis() -> Self {
        Self::Emphasis
    }

    pub(crate) fn footnote_definition<T: Into<String>>(label: T) -> Self {
        Self::FootnoteDefinition(FootnoteDefinition {
            label: label.into(),
        })
    }

    pub(crate) fn footnote_reference<T: Into<String>>(label: T) -> Self {
        Self::FootnoteReference(FootnoteReference {
            label: label.into(),
        })
    }

    pub(crate) fn heading<T: Into<u8>>(level: T) -> Self {
        Self::Heading(Heading {
            level: level.into(),
        })
    }

    pub(crate) fn html<T: Into<String>>(value: T) -> Self {
        Self::Html(Html {
            value: value.into(),
        })
    }

    pub(crate) fn image<T: Into<String>, U: Into<String>, V: Into<Option<String>>>(
        alternate_text: T,
        url: U,
        title: V,
    ) -> Self {
        Self::Image(Image {
            alternate_text: alternate_text.into(),
            url: url.into(),
            title: title.into(),
        })
    }

    pub(crate) fn image_reference<T: Into<String>, U: Into<ReferenceKind>, V: Into<String>>(
        alternative_text: T,
        reference_kind: U,
        label: V,
    ) -> Self {
        Self::ImageReference(ImageReference {
            alternative_text: alternative_text.into(),
            reference_kind: reference_kind.into(),
            label: label.into(),
        })
    }

    pub(crate) fn inline_code<T: Into<String>>(value: T) -> Self {
        Self::InlineCode(InlineCode {
            value: value.into(),
        })
    }

    pub(crate) fn inline_math<T: Into<String>>(value: T) -> Self {
        Self::InlineMath(InlineMath {
            value: value.into(),
        })
    }

    pub(crate) fn link<T: Into<String>, U: Into<Option<String>>>(url: T, title: U) -> Self {
        Self::Link(Link {
            url: url.into(),
            title: title.into(),
        })
    }

    pub(crate) fn link_reference<T: Into<String>, U: Into<ReferenceKind>>(
        label: T,
        reference_kind: U,
    ) -> Self {
        Self::LinkReference(LinkReference {
            label: label.into(),
            reference_kind: reference_kind.into(),
        })
    }

    pub(crate) fn list<T: Into<ListKind>>(kind: T) -> Self {
        Self::List(List { kind: kind.into() })
    }

    pub(crate) fn list_item<T: Into<Option<bool>>>(checked: T) -> Self {
        Self::ListItem(ListItem {
            checked: checked.into(),
        })
    }

    pub(crate) fn math<T: Into<String>>(value: T) -> Self {
        Self::Math(Math {
            value: value.into(),
        })
    }

    pub(crate) fn paragraph() -> Self {
        Self::Paragraph
    }

    pub(crate) fn root() -> Self {
        Self::Root
    }

    pub(crate) fn strong() -> Self {
        Self::Strong
    }

    pub(crate) fn strike_through() -> Self {
        Self::StrikeThrough
    }

    pub(crate) fn table<T: Into<Vec<AlignKind>>>(alignments: T) -> Self {
        Self::Table(Table {
            alignments: alignments.into(),
        })
    }

    pub(crate) fn table_cell() -> Self {
        Self::TableCell
    }

    pub(crate) fn table_row() -> Self {
        Self::TableRow
    }

    pub(crate) fn text<T: Into<String>>(value: T) -> Self {
        Self::Text(TextContent {
            value: value.into(),
        })
    }

    pub(crate) fn thematic_break() -> Self {
        Self::ThematicBreak
    }
}

/// TODO: common structs for dems.

/// A [NodeKind::Definition] payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    /// The label of the definition.
    pub label: String,
    /// Destination of the referenced resource.
    pub destination: String,
    /// Title of the referenced resource.
    pub title: Option<String>,
}

/// A [NodeKind::FootnoteDefinition] payload.
pub type FootnoteDefinition = Label;

/// A [NodeKind::FootnoteReference] payload.
pub type FootnoteReference = Label;

/// A [NodeKind::Heading] payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    /// Between [1-6].
    pub level: u8,
}

/// Raw HTML.
/// ```text
/// <div>bullchit</div>
/// ```
pub type Html = TextContent;

/// A [NodeKind::Image] payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    /// Alternative text.
    pub alternate_text: String,
    /// URL to the referenced resource.
    pub url: String,
    /// Advisory info for the resource, such as something that would be
    /// appropriate for a tooltip.
    pub title: Option<String>,
}

/// A [NodeKind::ImageReference] payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageReference {
    /// Alternative text.
    pub alternative_text: String,
    /// Reference kind.
    pub reference_kind: ReferenceKind,
    /// The image reference's label.
    pub label: String,
}

/// A [NodeKind::InlineCode] payload.
pub type InlineCode = TextContent;

/// A [NodeKind::InlineMath] payload.
pub type InlineMath = TextContent;

/// A [NodeKind::Link] payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    /// The URL of the link.
    pub url: String,
    /// The title of the link.
    pub title: Option<String>,
}

/// A [NodeKind::LinkReference] payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkReference {
    /// The link reference's label.
    pub label: String,
    /// The [ReferenceKind] of the link reference.
    pub reference_kind: ReferenceKind,
}

/// Small enum listing the possible kinds of a [List].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListKind {
    /// An ordered list with the starting number.
    Ordered(u32),
    /// An unordered list.
    Unordered,
}

/// A [NodeKind::List] payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    /// The [ListKind] of the list.
    pub kind: ListKind,
}

/// A [NodeKind::ListItem] payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    /// GFM: whether the item is done (when `true`), not done (when `false`),
    /// or indeterminate or not applicable (`None`).
    pub checked: Option<bool>,
}

/// A [NodeKind::Math] payload.
pub type Math = TextContent;

/// A [NodeKind::Text] payload.
pub type Text = TextContent;

/// A [NodeKind::Table] payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    /// The alignments of the table columns, from left to right.
    pub alignments: Vec<AlignKind>,
}

/// A common payload for [NodeKind]s that only hold a label as a their payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    /// The node's label.
    pub label: String,
}

/// A common payload for [NodeKind]s that only hold textual value as their payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextContent {
    /// The node's text.
    pub value: String,
}
