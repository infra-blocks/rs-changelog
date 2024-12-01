use crate::{block::LinkReferenceDefinition, Document};

use super::DisplayHtml;

impl<'a> DisplayHtml for Document<'a> {
    fn display_html(
        &self,
        buffer: &mut String,
        link_reference_definitions: &[LinkReferenceDefinition],
    ) {
        // TODO: use some form of fold or sum'
        for block in &self.blocks {
            block.display_html(buffer, link_reference_definitions)
        }
    }
}
