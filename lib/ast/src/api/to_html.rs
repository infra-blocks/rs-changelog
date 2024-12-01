/// Produces an HTML string from a reference to the implementer.
pub trait ToHtml {
    /// Produce a valid HTML string from this instance.
    fn to_html(&self) -> String;
}
