use std::ops::Range;

use changelog_ast::CowStr;

#[derive(Debug, Clone)]
pub struct ReferenceDefinition<'source> {
    label: String,
    dest: CowStr<'source>,
    range: Range<usize>,
}

impl<'source> ReferenceDefinition<'source> {
    pub(crate) fn new(label: String, dest: CowStr<'source>, range: Range<usize>) -> Self {
        Self { label, dest, range }
    }

    // TODO: check terminology from CMARK specticules
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn dest(&self) -> &str {
        &self.dest
    }

    pub fn range(&self) -> &Range<usize> {
        &self.range
    }
}
