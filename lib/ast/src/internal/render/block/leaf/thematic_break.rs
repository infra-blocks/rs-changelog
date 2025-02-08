use segment::SegmentLike;

use crate::block::ThematicBreak;

use super::DisplayHtml;

impl<'a> DisplayHtml for ThematicBreak<'a> {
    fn display_html(&self, buffer: &mut String, _: &[super::LinkReferenceDefinition]) {
        buffer.push_str("<hr />");
        if self.segment().text().ends_with('\n') {
            buffer.push('\n');
        }
    }
}
