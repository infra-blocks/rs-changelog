use std::fmt::{Display, Formatter};

use crate::{error::Error, Position};

/// A struct representing a span between 2 positions in text.
///
/// The start and end positions are different, and the end comes after
/// the start (as returned by the [PartialOrd] implementation of [Position]).
/// This invariant is enforced at construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    /// The inclusive start position.
    start: Position,
    /// The exclusive end position.
    end: Position,
}

impl Span {
    /// Constructs a new [Span] instance between the start and the end specified.
    ///
    /// Both bounds are inclusive.
    ///
    /// # Panics
    /// This function will panic if the start and the end are the same, or if the end comes before the start.
    pub fn new<T: Into<Position>, U: Into<Position>>(start: T, end: U) -> Self {
        Self::try_new(start, end).unwrap()
    }

    /// Constructs a new [Span] instance by extending the current one to the provided position.
    ///
    /// Formally, the new [Span]'s start is the same as this one, but the end is the provided position.
    pub fn extended_to<T: Into<Position>>(&self, end: T) -> Self {
        Self::new(self.start, end)
    }

    /// A safe version of the constructor that returns a [Result] instead of panicking.
    ///
    /// # Errors
    /// This function will return an error if the start and the end are the same, or if the end comes before the start.
    pub fn try_new<T: Into<Position>, U: Into<Position>>(
        start: T,
        end: U,
    ) -> Result<Self, impl std::error::Error> {
        let start = start.into();
        let end = end.into();

        if start == end {
            return Err(Error::new(format!(
                "expected end to be after start, but both are equal to {:?}",
                start
            )));
        }

        if end < start {
            return Err(Error::new(format!(
                "expected start {:?} to be before end {:?}",
                start, end
            )));
        }

        Ok(Span { start, end })
    }

    /// Returns the start position of the span.
    pub fn start(&self) -> Position {
        self.start
    }

    /// Returns the end position of the span.
    pub fn end(&self) -> Position {
        self.end
    }

    /// Selects the text at the corresponding location from the provided source.
    ///
    /// It is assumed that the location was obtained from the same source. Note that
    /// this function only considers the offsets, and not the actual line and column.
    ///
    /// # Panics
    /// This function will panic if the start and the end are not valid indices in the source.
    pub fn select_in<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start.offset..self.end.offset]
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{})-({},{})",
            self.start.line, self.start.column, self.end.line, self.end.column
        )
    }
}

impl<T: Into<Position>, U: Into<Position>> From<(T, U)> for Span {
    fn from((start, end): (T, U)) -> Self {
        Span::new(start, end)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_work_for_valid_start_and_end() {
        let start = Position::new(1, 1, 1);
        let end = Position::new(2, 2, 2);
        let span = Span::new(start, end);

        assert_eq!(span.start(), start);
        assert_eq!(span.end(), end);
    }

    #[test]
    fn should_work_when_extended() {
        let start = Position::new(1, 1, 1);
        let end = Position::new(2, 2, 3);
        let span = Span::new(start, end);
        let new_end = Position::new(3, 3, 6);
        let new_span = span.extended_to(new_end);

        assert_eq!(new_span.start(), start);
        assert_eq!(new_span.end(), new_end);
    }

    #[test]
    #[should_panic]
    fn should_fail_if_start_is_same_as_end() {
        let start = Position::new(1, 1, 1);
        let end = Position::new(1, 1, 1);
        Span::new(start, end);
    }

    #[test]
    #[should_panic]
    fn should_fail_if_end_is_before_start() {
        let start = Position::new(2, 2, 3);
        let end = Position::new(1, 1, 1);
        Span::new(start, end);
    }

    mod select_in {
        use super::*;

        #[test]
        fn should_work_for_valid_source() {
            let source = "Hello, World!";
            let start = Position::new(1, 1, 0);
            let end = Position::new(1, 6, 5);
            let span = Span::new(start, end);

            assert_eq!(span.select_in(source), "Hello");
        }

        #[test]
        #[should_panic]
        fn should_throw_if_start_is_invalid() {
            let source = "Hello, World!";
            // If the start is invalid, then the end as well. This is an invariant of the span.
            let start = Position::new(1, 35, 34);
            let end = Position::new(1, 36, 35);
            let span = Span::new(start, end);
            span.select_in(source);
        }

        #[test]
        #[should_panic]
        fn should_fail_if_end_is_invalid() {
            let source = "Hello, World!";
            let start = Position::new(1, 6, 5);
            let end = Position::new(1, 15, 14);
            let span = Span::new(start, end);
            span.select_in(source);
        }
    }
}
