#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    /// A markdown heading with a depth within [1-3].
    Heading(Heading),
    // TODO: rename to text.
    /// A textual paragraph.
    Paragraph(Paragraph),
    /// A markdown list.
    List(List),
    /// A markdown definion
    Definition(Definition),
}

impl NodeKind {
    pub fn heading<T: Into<String>>(depth: u8, text: T) -> Self {
        Self::Heading(Heading {
            depth,
            text: text.into(),
        })
    }

    pub fn paragraph<T: Into<String>>(text: T) -> Self {
        Self::Paragraph(Paragraph { text: text.into() })
    }

    pub fn list<T: Into<Vec<ListItem>>>(items: T) -> Self {
        Self::List(List {
            items: items.into(),
        })
    }

    pub fn list_item<T: Into<location::Span>, U: Into<String>>(location: T, text: U) -> ListItem {
        ListItem {
            location: location.into(),
            text: text.into(),
        }
    }

    pub fn definition<T: Into<String>, U: Into<String>>(label: T, destination: U) -> Self {
        Self::Definition(Definition {
            label: label.into(),
            destination: destination.into(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    depth: u8,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph {
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    items: Vec<ListItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    location: location::Span,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    label: String,
    destination: String,
}
