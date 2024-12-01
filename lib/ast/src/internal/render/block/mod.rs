mod leaf;

use super::DisplayHtml;
use crate::block::{Block, LinkReferenceDefinition};

impl<'a> DisplayHtml for Block<'a> {
    fn display_html(
        &self,
        buffer: &mut String,
        link_reference_definitions: &[LinkReferenceDefinition],
    ) {
        match self {
            Block::Leaf(leaf) => leaf.display_html(buffer, link_reference_definitions),
        }
    }
}
