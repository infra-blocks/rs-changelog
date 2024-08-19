use std::sync;

use crate::InlineNode;

use super::{
    builder::{BlockNodeBuilder, BuildResult},
    BlockNode,
};

// TODO: a type aggregating location::Span + 'a str for convenience?. Maybe as a crate feature.
pub struct ParagraphBuilder {
    location: Option<location::Span>,
    children: Option<Vec<InlineNode>>,
}

impl ParagraphBuilder {
    pub fn new() -> Self {
        ParagraphBuilder {
            location: None,
            children: None,
        }
    }

    fn is_building(&self) -> bool {
        match self.location {
            Some(_) => true,
            None => false,
        }
    }

    fn append_child(&mut self, child: InlineNode) {
        match &mut self.children {
            Some(children) => children.push(child),
            None => self.children = Some(vec![child]),
        }
    }
}

// Paragraphs require at least one non whitespace character.
static REGEX: sync::LazyLock<regex::Regex> =
    sync::LazyLock::new(|| regex::Regex::new(r"(^\s{0,3})(\S+.*?)(\s*)$").unwrap());

impl BlockNodeBuilder for ParagraphBuilder {
    fn consume_line(&mut self, current: location::Position, line: &str) -> BuildResult {
        match REGEX.captures(line) {
            Some(captures) => {
                self.location = Some(location::Span::new(
                    current,
                    location::Position::new(
                        current.line,
                        current.column + line.chars().count(),
                        current.offset + line.len(),
                    ),
                ));

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
                self.append_child(child);
                BuildResult::Building
            }
            None => {
                if self.is_building() {
                    BuildResult::Success(BlockNode::paragraph(
                        self.location.take().unwrap(),
                        self.children.take().unwrap(),
                    ))
                } else {
                    BuildResult::Incompatible
                }
            }
        }
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

    #[test]
    fn should_work_for_single_line_paragraph() {
        let mut builder = ParagraphBuilder::new();
        let source = "This is a paragraph

";
        let mut lines = source.split_inclusive("\n");
        let result = builder.consume_line(location::Position::first(), lines.next().unwrap());
        assert_eq!(result, BuildResult::Building);
        let result = builder.consume_line(location::Position::new(2, 1, 20), lines.next().unwrap());
        assert_eq!(
            result,
            BuildResult::Success(BlockNode::paragraph(
                location::Span::new(
                    location::Position::first(),
                    location::Position::new(1, 21, 20)
                ),
                vec![InlineNode::text(location::Span::new(
                    location::Position::first(),
                    location::Position::new(1, 20, 19)
                ))]
            ))
        );
    }

    // TODO: tests with whitespaces before and at the end of the line.
}
