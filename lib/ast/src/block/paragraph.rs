use std::sync;

use crate::InlineNode;

use super::{
    builder::{AddResult, BlockNodeBuilder, InitResult},
    BlockNode,
};

// TODO: a type aggregating location::Span + 'a str for convenience?. Maybe as a crate feature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphBuilder {
    location: location::Span,
    children: Vec<InlineNode>,
}

// Paragraphs require at least one non whitespace character.
static REGEX: sync::LazyLock<regex::Regex> =
    sync::LazyLock::new(|| regex::Regex::new(r"(^\s{0,3})(\S+.*?)(\s*)$").unwrap());

impl BlockNodeBuilder for ParagraphBuilder {
    fn init_from_line(current: location::Position, line: &str) -> InitResult<Self>
    where
        Self: Sized,
    {
        match REGEX.captures(line) {
            Some(captures) => {
                let location = location::Span::new(
                    current,
                    location::Position::new(
                        current.line,
                        current.column + line.chars().count(),
                        current.offset + line.len(),
                    ),
                );

                let child_location_start =
                    increment_position_by_content(current, captures.get(1).unwrap().as_str());
                let child_location_end = increment_position_by_content(
                    child_location_start,
                    captures.get(2).unwrap().as_str(),
                );

                let child = InlineNode::text(location::Span::new(
                    child_location_start,
                    child_location_end,
                ));
                let children = vec![child];
                InitResult::Building(Self { location, children })
            }
            None => InitResult::Incompatible,
        }
    }

    fn maybe_add_line(&mut self, current: location::Position, line: &str) -> AddResult {
        match REGEX.captures(line) {
            Some(captures) => {
                // TODO: extend current position.
                todo!("implement me");
            }
            None => AddResult::,
        }
    }

    fn finish(self) -> BlockNode {
        BlockNode::paragraph(self.location, self.children)
    }
}

// TODO in location. What's a good name though?
fn increment_position_by_content(
    position: location::Position,
    content: &str,
) -> location::Position {
    location::Position::new(
        position.line,
        position.column + content.chars().count(),
        position.offset + content.len(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    mod single_line_inputs {
        use super::*;

        #[test]
        fn should_work_for_single_line_paragraph() {
            let source = "This is a paragraph";
            let mut lines = source.split_inclusive("\n");
            let result = ParagraphBuilder::init_from_line(
                location::Position::first(),
                lines.next().unwrap(),
            );
            let InitResult::Building(builder) = result else {
                panic!("unexpected result: {:?}", result);
            };
            let node = builder.finish();
            assert_eq!(
                node,
                BlockNode::paragraph(
                    location::Span::new(
                        location::Position::first(),
                        location::Position::new(1, 21, 20)
                    ),
                    vec![InlineNode::text(location::Span::new(
                        location::Position::first(),
                        location::Position::new(1, 20, 19)
                    ))]
                )
            );
        }

        // TODO: tests with whitespaces before and at the end of the line.
    }
}
