#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum TitleRule {
    /// Enforces the presence of a title heading.
    HeadingOnly,
    /// Enforces the presence of a title heading followed by paragraphs (could be just one) of text.
    #[default]
    HeadingAndText,
    /// Enforces that no title is present. Only the absence of a heading is checked.
    None,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Rules {
    /// Requires the presence of a title and its accompanying description.
    /// Defaults to true.
    pub title: TitleRule,
}

impl Rules {}
