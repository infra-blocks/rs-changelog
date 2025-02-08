use crate::block::FencedCode;
use segment::{LineSegment, SegmentLike};

use super::DisplayHtml;

trait PushContentSegment {
    fn push_content_segment(&mut self, segment: &LineSegment);
}

impl PushContentSegment for String {
    // TODO: add the unindent.
    fn push_content_segment(&mut self, segment: &LineSegment) {
        for char in segment.text().chars() {
            // Escape html chars!
            match char {
                '&' => self.push_str("&amp;"),
                '<' => self.push_str("&lt;"),
                '>' => self.push_str("&gt;"),
                '"' => self.push_str("&quot;"),
                '\'' => self.push_str("&#x27;"),
                '/' => self.push_str("&#x2F;"),
                _ => self.push(char),
            }
        }
    }
}

impl<'a> DisplayHtml for FencedCode<'a> {
    fn display_html(
        &self,
        buffer: &mut String,
        _link_reference_definitions: &[crate::block::LinkReferenceDefinition],
    ) {
        buffer.push_str("<pre><code>");
        for segment in self.content_segments() {
            buffer.push_content_segment(segment);
        }
        buffer.push_str("</code></pre>");
    }
}
