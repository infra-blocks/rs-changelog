use crate::block::BlankLine;

use super::DisplayHtml;

impl<'a> DisplayHtml for BlankLine<'a> {
    fn display_html(&self, _: &mut String, _: &[crate::block::LinkReferenceDefinition]) {
        // Blank lines are ignored.
    }
}
