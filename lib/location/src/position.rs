use crate::error::Error;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

/// This struct represents a single position in text.
///
/// It is composed of a line number and a column number. Both start at 1
/// and this invariant is enforced by the constructor.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord)]
pub struct Position {
    /// 1-indexed integer representing a line in a source file.
    pub line: usize,
    /// 1-indexed integer representing a column in a source file.
    pub column: usize,
    /// 0-indexed character offset in the source file.
    pub offset: usize,
}

impl Position {
    /// Constructs a new `Position` instance at the line and the column specified.
    ///
    /// # Panics
    /// This function will panic if either the line or the column is less than 1.
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self::try_new(line, column, offset).unwrap()
    }

    /// Returns the first possible position in a file.
    pub fn first() -> Self {
        Self::new(1, 1, 0)
    }

    /// A safe version of the constructor that returns a [Result] instead of panicking.
    ///
    /// # Errors
    /// This function will return an error if either the line or the column is less than 1.
    pub fn try_new(
        line: usize,
        column: usize,
        offset: usize,
    ) -> Result<Self, impl std::error::Error> {
        if line == 0 {
            return Err(Error::from(format!(
                "expected line to be greater than 0, got {}",
                line
            )));
        } else if column == 0 {
            return Err(Error::from(format!(
                "expected column to be greater than 0, got {}",
                column
            )));
        } else {
            Ok(Self {
                line,
                column,
                offset,
            })
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.offset.partial_cmp(&other.offset)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}:{}): ", self.line, self.column)
    }
}

impl From<(usize, usize, usize)> for Position {
    fn from((line, column, offset): (usize, usize, usize)) -> Self {
        Self::new(line, column, offset)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod constructor {
        use super::*;

        #[test]
        fn should_work_for_valid_line_and_column() {
            let position = Position::new(1, 10, 10);
            assert_eq!(position.line, 1);
            assert_eq!(position.column, 10);
            assert_eq!(position.offset, 10);
        }

        #[test]
        #[should_panic]
        fn should_fail_for_invalid_line() {
            Position::new(0, 10, 4);
        }

        #[test]
        #[should_panic]
        fn should_fail_for_invalid_column() {
            Position::new(10, 0, 5);
        }
    }

    mod first {
        use super::*;

        #[test]
        fn should_return_first_position() {
            let position = Position::first();
            assert_eq!(position.line, 1);
            assert_eq!(position.column, 1);
            assert_eq!(position.offset, 0);
        }
    }

    mod ordering {
        use super::*;

        #[test]
        fn should_work_with_equal_positions() {
            let position = Position::new(1, 1, 1);
            assert_eq!(
                position.partial_cmp(&position),
                Some(std::cmp::Ordering::Equal)
            );
        }

        #[test]
        fn should_work_with_bigger_offset() {
            let left = Position::new(1, 1, 1);
            let right = Position::new(1, 1, 2);
            assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));
        }

        #[test]
        fn should_work_with_smaller_offset() {
            let left = Position::new(1, 1, 2);
            let right = Position::new(1, 1, 1);
            assert_eq!(left.partial_cmp(&right), Some(Ordering::Greater));
        }
    }
}
