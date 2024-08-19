// TODO: seperate between inline and inline container elements.
/// Implementation of Markdown inline elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline {
    Autolink,
    Text,
    Link,
    Emphasis,
    Strong,
    Image,
    Code,
    RawHtml,
    HardLineBreak,
    SoftLineBreak,
}

// TODO: different types for leaf vs container inlines?
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineNode {
    kind: Inline,
    location: location::Span,
    children: Option<Vec<InlineNode>>,
}

impl InlineNode {
    pub fn new(kind: Inline, location: location::Span, children: Option<Vec<InlineNode>>) -> Self {
        InlineNode {
            kind,
            location,
            children,
        }
    }

    pub fn text(location: location::Span) -> Self {
        InlineNode {
            kind: Inline::Text,
            location,
            children: None,
        }
    }
}
