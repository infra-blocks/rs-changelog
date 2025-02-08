use crate::Lines;

/// An extension trait to add the `by_lines` method to string slices.
pub trait ByLines<'a> {
    /// Splits inclusively the text on newline characters, and returns an iterator
    /// producing [crate::Line]s.
    fn by_lines(self) -> Lines<'a>;
}

impl<'a> ByLines<'a> for &'a str {
    fn by_lines(self) -> Lines<'a> {
        Lines::from(self)
    }
}

#[cfg(test)]
mod test {
    use crate::LineSlice;

    use super::*;

    // Not redoing all the [Lines] tests here, just showing that the dispatch works.
    #[test]
    fn should_work_with_multiline_text_ending_with_newline() {
        let text = r"This is the first line.
This is the second line.
This is the third line.
";
        let lines = text.by_lines().collect::<Vec<_>>();
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
