mod atx_heading;
mod blank_line;
mod fenced_code;
mod indented_code;
mod thematic_break;

use crate::block::{Leaf, LinkReferenceDefinition};

use super::DisplayHtml;

impl<'a> DisplayHtml for Leaf<'a> {
    fn display_html(
        &self,
        buffer: &mut String,
        link_reference_definitions: &[LinkReferenceDefinition],
    ) {
        match self {
            Leaf::AtxHeading(atx_heading) => {
                atx_heading.display_html(buffer, link_reference_definitions)
            }
            Leaf::FencedCode(fenced_code) => {
                fenced_code.display_html(buffer, link_reference_definitions)
            }
            Leaf::ThematicBreak(thematic_break) => {
                thematic_break.display_html(buffer, link_reference_definitions)
            }
            Leaf::IndentedCode(indented_code) => {
                indented_code.display_html(buffer, link_reference_definitions)
            }
            Leaf::BlankLine(blank_line) => {
                blank_line.display_html(buffer, link_reference_definitions)
            }
            _ => unimplemented!("diplay html not implemented for {:?}", self),
        }
    }
}
