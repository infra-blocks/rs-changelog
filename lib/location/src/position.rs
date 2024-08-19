use crate::error::Error;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

/// This struct represents a single position in text.
///
/// It is composed of a line number, and a column number (counted by characters) and a byte offset.
/// Both the line and colum numbers are 1-indexed, whereas the byte offset is 0-indexed.
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

    /// Returns the first position after crossing the provided text and starting at
    /// the current position.
    pub fn across<T: AsRef<str>>(&self, text: T) -> Self {
        let text = text.as_ref();
        if text.is_empty() {
            return *self;
        }

        // The offset is always calculated the same way: this position's offset plus the
        // length of the text.
        let offset = self.offset + text.len();
        let newline_count = text.chars().filter(|c| *c == '\n').count();
        // If there is no newline, we are on the same line, otherwise we increment by the
        // number of new lines.
        let line = self.line + newline_count;

        // The column number is given by the amount of characters in the last segment.
        // If there is no newline, then we simply add the amount of characters in the text.
        // If there is at least one new line, than the column is reset to 1 and added
        // with the characters count of the last line.
        let column = match text.rfind('\n') {
            Some(index) => 1 + &text[index + 1..].chars().count(),
            None => self.column + text.chars().count(),
        };
        Self::new(line, column, offset)
    }

    /// A safe version of the constructor that returns a [Result] instead of panicking.
    ///
    /// # Errors
    /// This function will return an error if the start and the end are the same, or if the end comes before the start.
    fn try_new(line: usize, column: usize, offset: usize) -> Result<Self, impl std::error::Error> {
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

    mod across {
        use super::*;

        #[test]
        fn should_work_for_empty_segment() {
            let position = Position::new(2, 3, 4);
            let new_position = position.across("");
            assert_eq!(new_position.line, 2);
            assert_eq!(new_position.column, 3);
            assert_eq!(new_position.offset, 4);
        }

        #[test]
        fn should_work_with_singe_whitespace() {
            let position = Position::first();
            let new_position = position.across(" ");
            assert_eq!(new_position.line, 1);
            assert_eq!(new_position.column, 2);
            assert_eq!(new_position.offset, 1);
        }

        #[test]
        fn should_work_with_single_newline() {
            let position = Position::first();
            let new_position = position.across("\n");
            assert_eq!(new_position.line, 2);
            assert_eq!(new_position.column, 1);
            assert_eq!(new_position.offset, 1);
        }
    }
}
