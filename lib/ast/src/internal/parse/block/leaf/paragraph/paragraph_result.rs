use itertools::put_back_n;

use crate::{
    internal::parse::segment::{ParagraphContinuationSegment, ParagraphSegments},
    IntoSegments,
};

use super::link_reference_definition::LinkReferenceDefinition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph<'a> {
    segments: Vec<ParagraphContinuationSegment<'a>>,
}

impl<'a> Paragraph<'a> {
    pub fn new(segments: Vec<ParagraphContinuationSegment<'a>>) -> Self {
        Self { segments }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphResult<'a> {
    pub link_reference_definitions: Vec<LinkReferenceDefinition<'a>>,
    pub paragraph: Option<Paragraph<'a>>,
}

impl<'a> ParagraphResult<'a> {
    pub fn new(
        link_reference_definitions: Vec<LinkReferenceDefinition<'a>>,
        paragraph: Option<Paragraph<'a>>,
    ) -> Self {
        Self {
            link_reference_definitions,
            paragraph,
        }
    }
}

impl<'a> From<ParagraphSegments<'a>> for ParagraphResult<'a> {
    fn from(value: ParagraphSegments<'a>) -> Self {
        let mut link_reference_definitions = Vec::new();
        let mut segments = put_back_n(value.into_segments());
        while let Some(link_reference_definition) = LinkReferenceDefinition::try_read(&mut segments)
        {
            link_reference_definitions.push(link_reference_definition);
        }
        let remaining: Vec<_> = segments
            .map(|segment| segment.try_into().expect("unexpected invalid paragraph continuation segment after unwinding link reference definitions"))
            .collect();
        if remaining.len() == 0 {
            Self::new(link_reference_definitions, None)
        } else {
            Self::new(link_reference_definitions, Some(Paragraph::new(remaining)))
        }
    }
}
