use crate::Segment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtxHeading<'a> {
    /// The whole line segment of the heading.
    pub segment: Segment<'a>,
    /// The subsegment containing the title of the heading. Possibly empty.
    pub title: Option<&'a str>,
    /// The level of the heading, from 1 to 6 inclusive.
    pub level: u8,
}
