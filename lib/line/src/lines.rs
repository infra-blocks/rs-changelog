use std::{iter::FusedIterator, str::SplitInclusive};

use crate::LineSlice;

/// An iterator over lines of text.
///
/// It is built on top of the the standard library [SplitInclusive] iterator.
/// Unlike the base iterator, this one returns [LineSlice] instances.
#[derive(Debug, Clone)]
pub struct Lines<'a>(SplitInclusive<'a, char>);

impl<'a> Lines<'a> {
    fn new(inner: SplitInclusive<'a, char>) -> Self {
        Self(inner)
    }
}

impl<'a> From<&'a str> for Lines<'a> {
    fn from(text: &'a str) -> Self {
        Self::new(text.split_inclusive('\n'))
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = LineSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.0.next()?;
        Some(LineSlice::new(text))
    }
}

impl<'a> FusedIterator for Lines<'a> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_text_should_produce_no_lines() {
        assert_eq!(Lines::from("").count(), 0);
    }

    #[test]
    fn text_without_newline_should_produce_one_line() {
        let text = "This is a line!";
        let lines = Lines::from(text).collect::<Vec<_>>();
        assert_eq!(lines, vec![LineSlice::new(text)]);
    }

    #[test]
    fn text_ending_with_newline_should_produce_one_line() {
        let text = "This is a line!\n";
        let lines = Lines::from(text).collect::<Vec<_>>();
        assert_eq!(lines, vec![LineSlice::new(text)]);
    }

    #[test]
    fn should_work_with_multiline_text_ending_without_newline() {
        let text = r"This is the first line.
This is the second line.
This is the third line.";
        let lines = Lines::from(text).collect::<Vec<_>>();
        assert_eq!(
            lines,
            vec![
                LineSlice::new("This is the first line.\n"),
                LineSlice::new("This is the second line.\n"),
                LineSlice::new("This is the third line.")
            ]
        );
    }

    #[test]
    fn should_work_with_multiline_text_ending_with_newline() {
        let text = r"This is the first line.
This is the second line.
This is the third line.
";
        let lines = Lines::from(text).collect::<Vec<_>>();
        assert_eq!(
            lines,
            vec![
                LineSlice::new("This is the first line.\n"),
                LineSlice::new("This is the second line.\n"),
                LineSlice::new("This is the third line.\n")
            ]
        );
    }
}
