use changelog_ast::{Heading, HeadingLevel, Link, LinkType, Node, Text};

pub trait NodeExt {
    fn is_heading_of_level(&self, level: HeadingLevel) -> bool;
    fn is_heading_that<F: FnOnce(&Heading) -> bool>(&self, predicate: F) -> bool;
    fn is_link_that<F: FnOnce(&Link) -> bool>(&self, predicate: F) -> bool;
    fn is_shortcut_link_with_id(&self, id: &str) -> bool;
    fn is_text_equals(&self, expected: &str) -> bool;
    fn is_text_that<F: FnOnce(&Text) -> bool>(&self, predicate: F) -> bool;
}

impl<'source> NodeExt for Node<'source> {
    fn is_heading_of_level(&self, level: HeadingLevel) -> bool {
        self.is_heading_that(|h| h.level == level)
    }

    fn is_heading_that<F: FnOnce(&Heading) -> bool>(&self, predicate: F) -> bool {
        matches!(self, Node::Heading(heading) if predicate(heading))
    }

    fn is_link_that<F: FnOnce(&Link) -> bool>(&self, predicate: F) -> bool {
        matches!(self, Node::Link(link) if predicate(link))
    }

    fn is_shortcut_link_with_id(&self, id: &str) -> bool {
        self.is_link_that(|l| l.link_type == LinkType::Shortcut && l.id.as_ref() == id)
    }

    fn is_text_equals(&self, expected: &str) -> bool {
        self.is_text_that(|t| t.text.as_ref() == expected)
    }

    fn is_text_that<F: FnOnce(&Text) -> bool>(&self, predicate: F) -> bool {
        matches!(self, Node::Text(text) if predicate(text))
    }
}
