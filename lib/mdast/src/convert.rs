use markdown::unist;

/// A trait to convert a type into a [location::Location].
///
/// Implementations are provided for the [unist::Position], [Option<unist::Position>], and [unist::Point] types.
pub trait IntoLocation {
    fn into_location(self) -> location::Location;
}

impl IntoLocation for unist::Position {
    fn into_location(self) -> location::Location {
        self.into_span().into()
    }
}

impl IntoLocation for Option<unist::Position> {
    fn into_location(self) -> location::Location {
        self.unwrap().into_location()
    }
}

impl IntoLocation for unist::Point {
    fn into_location(self) -> location::Location {
        self.into_position().into()
    }
}

pub trait IntoSpan {
    fn into_span(self) -> location::Span;
}

impl IntoSpan for unist::Position {
    fn into_span(self) -> location::Span {
        location::Span::new(self.start.into_position(), self.end.into_position())
    }
}

impl IntoSpan for Option<unist::Position> {
    fn into_span(self) -> location::Span {
        self.unwrap().into_span()
    }
}

pub trait IntoPosition {
    fn into_position(self) -> location::Position;
}

impl IntoPosition for unist::Point {
    fn into_position(self) -> location::Position {
        location::Position::new(self.line, self.column, self.offset)
    }
}
