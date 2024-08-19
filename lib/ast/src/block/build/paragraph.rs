use std::sync::LazyLock;

use crate::ParagraphNode;

use super::builder::{AddResult, BuilderState, InitResult, NodeBuilder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParagraphNodeBuilder {
    location: location::Span,
    // The sequence of locations in the source that identify the paragraph's children.
    children: Vec<location::Span>,
    state: BuilderState,
}

impl ParagraphNodeBuilder {
    fn check_not_finished(&self) {
        if self.state == BuilderState::Finished {
            panic!("builder is already finished");
        }
    }
}

// Paragraphs require at least one non whitespace character.
// The trailing whitespaces are not part of the "raw content" of the paragraph.
// TODO: include line break in regex.
static REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(^\s{0,3})(\S+.*?)(\s*)$").unwrap());

// TODO: store children as Segments of text? Segment = (start_position, text);
impl NodeBuilder for ParagraphNodeBuilder {
    type Node = ParagraphNode;

    fn init_from_line(current: location::Position, line: &str) -> InitResult<Self>
    where
        Self: Sized,
    {
        match REGEX.captures(line) {
            Some(captures) => {
                let location = location::Span::across(current, line);

                let child_location_start = current.across(captures.get(1).unwrap().as_str());
                let child_location =
                    location::Span::across(child_location_start, captures.get(2).unwrap().as_str());
                let children = vec![child_location];
                InitResult::Builder(Self {
                    location,
                    children,
                    state: BuilderState::InProgress,
                })
            }
            None => InitResult::Incompatible,
        }
    }

    fn maybe_add_line(&mut self, current: location::Position, line: &str) -> AddResult {
        // It is an error to mutate a builder that's already finished.
        self.check_not_finished();

        match REGEX.captures(line) {
            Some(captures) => {
                let new_end = location::Position::new(
                    current.line,
                    current.column + line.chars().count(),
                    current.offset + line.len(),
                );
                self.location = self.location.extended_to(new_end);

                let child_location_start =
                    increment_position_by_content(current, captures.get(1).unwrap().as_str());
                let child_location_end = increment_position_by_content(
                    child_location_start,
                    captures.get(2).unwrap().as_str(),
                );

                let child_location = location::Span::new(child_location_start, child_location_end);
                self.children.push(child_location);
                AddResult::Ok
            }
            None => {
                self.state = BuilderState::Finished;
                AddResult::Incompatible
            }
        }
    }

    fn state(&self) -> BuilderState {
        self.state
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
