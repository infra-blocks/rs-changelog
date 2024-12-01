use crate::{
    link::{LinkDestination, LinkLabel, LinkTitle},
    Segment,
};

//TODO: to_html() for this struct won't produce anything.
// Only its usages in the rest of the document would.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkReferenceDefinition<'a> {
    pub segments: Vec<Segment<'a>>,
    pub label: LinkLabel<'a>,
    pub destination: LinkDestination<'a>,
    pub title: Option<LinkTitle<'a>>,
}

impl<'a> LinkReferenceDefinition<'a> {
    pub(crate) fn new(
        segments: Vec<Segment<'a>>,
        label: LinkLabel<'a>,
        destination: LinkDestination<'a>,
        title: Option<LinkTitle<'a>>,
    ) -> Self {
        Self {
            segments,
            label,
            destination,
            title,
        }
    }

    pub(crate) fn without_title(
        segments: Vec<Segment<'a>>,
        label: LinkLabel<'a>,
        destination: LinkDestination<'a>,
    ) -> Self {
        Self::new(segments, label, destination, None)
    }

    pub(crate) fn with_title(
        segments: Vec<Segment<'a>>,
        label: LinkLabel<'a>,
        destination: LinkDestination<'a>,
        title: LinkTitle<'a>,
    ) -> Self {
        Self::new(segments, label, destination, Some(title))
    }
}
