use markdown::{mdast, unist};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub start: Point,
    pub end: Point,
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
    /// 0-indexed integer representing a character in a source file.
    pub offset: usize,
}

impl From<unist::Point> for Point {
    fn from(point: unist::Point) -> Self {
        Point {
            line: point.line,
            column: point.column,
            offset: point.offset,
        }
    }
}

impl From<&unist::Point> for Point {
    fn from(point: &unist::Point) -> Self {
        Point {
            line: point.line,
            column: point.column,
            offset: point.offset,
        }
    }
}
