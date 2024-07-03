use markdown::{mdast, unist};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub start: Point,
    pub end: Point,
}

impl Position {
    pub fn option_to_string(maybe_position: &Option<Self>) -> String {
        match maybe_position {
            Some(position) => position.to_string(),
            None => "".to_string(),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}:{},{}:{}): ",
            self.start.line, self.start.column, self.end.line, self.end.column
        )
    }
}

impl From<unist::Position> for Position {
    fn from(position: unist::Position) -> Self {
        Position {
            start: position.start.into(),
            end: position.end.into(),
        }
    }
}

impl From<&unist::Position> for Position {
    fn from(position: &unist::Position) -> Self {
        Position {
            start: (&position.start).into(),
            end: (&position.end).into(),
        }
    }
}

impl Position {
    pub fn from_node(node: &mdast::Node) -> Option<Self> {
        node.position().map(|position| position.clone().into())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Point {
    /// 1-indexed integer representing a line in a source file.
    pub line: usize,
    /// 1-indexed integer representing a column in a source file.
    pub column: usize,
}

impl From<unist::Point> for Point {
    fn from(point: unist::Point) -> Self {
        Point {
            line: point.line,
            column: point.column,
        }
    }
}

impl From<&unist::Point> for Point {
    fn from(point: &unist::Point) -> Self {
        Point {
            line: point.line,
            column: point.column,
        }
    }
}
