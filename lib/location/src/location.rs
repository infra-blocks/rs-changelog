use std::fmt::Display;

use crate::{Position, Span};

/// An enum identifying a location in text.
///
/// There exists 2 variants of a location:
/// - [Location::Position]: a single position in text.
/// - [Location::Span]: a span between 2 positions in text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Location {
    /// A single position in text.
    Position(Position),
    /// A span between 2 position in text.
    Span(Span),
}

impl Location {
    /// Constructs a new [Location::Position] variant from a given [Position].
    pub fn position<T: Into<Position>>(position: T) -> Self {
        Location::Position(position.into())
    }

    /// Constructs a new [Location::Span] variant from a given [Span].
    pub fn span<T: Into<Span>>(span: T) -> Self {
        Location::Span(span.into())
    }

    /// Constucts a new [Location::Span] by extending the current location to the provided position.
    ///
    /// If the current location is a [Location::Position], then the result is a new [Location::Span]
    /// starting at the current position and extending to the provided one.
    ///
    /// If the current location is a [Location::Span], then the result is a new [Location::Span] where
    /// the inner [Span] has been created with [Span::extended_to].
    pub fn extended_to<T: Into<Position>>(&self, position: T) -> Self {
        match self {
            Self::Position(start) => Self::span(Span::new(*start, position)),
            Self::Span(span) => Self::span(span.extended_to(position)),
        }
    }

    /// Unwraps the inner [Position] if the variant is [Location::Position].
    ///
    /// # Panics
    /// This function will panic if the variant is not [Location::Position].
    pub fn unwrap_position(self) -> Position {
        match self {
            Location::Position(position) => position,
            _ => panic!("cannot unwrap position for {:?}", self),
        }
    }

    /// Borrowed variation of [Location::unwrap_position].
    ///
    /// # Panics
    /// This function will panic if the variant is not [Location::Position].
    pub fn unwrap_position_ref(&self) -> &Position {
        match self {
            Location::Position(position) => position,
            _ => panic!("cannot unwrap position for {:?}", self),
        }
    }

    /// Unwraps the inner [Span] if the variant is [Location::Span].
    ///
    /// # Panics
    /// This function will panic if the variant is not [Location::Span].
    pub fn unwrap_span(self) -> Span {
        match self {
            Location::Span(span) => span,
            _ => panic!("cannot unwrap span for {:?}", self),
        }
    }

    /// Borrowed variation of [Location::unwrap_span].
    ///
    /// # Panics
    /// This function will panic if the variant is not [Location::Span].
    pub fn unwrap_span_ref(&self) -> &Span {
        match self {
            Location::Span(span) => span,
            _ => panic!("cannot unwrap span for {:?}", self),
        }
    }
}

impl From<Span> for Location {
    fn from(span: Span) -> Self {
        Location::Span(span)
    }
}

impl From<Position> for Location {
    fn from(position: Position) -> Self {
        Location::Position(position)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Position(position) => write!(f, "{}", position),
            Location::Span(span) => write!(f, "{}", span),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_work_for_position() {
        let position = Position::new(1, 1, 1);
        let location = Location::position(position);

        assert_eq!(location.unwrap_position(), position);
    }

    #[test]
    fn should_work_for_span() {
        let start = Position::new(1, 1, 1);
        let end = Position::new(2, 2, 3);
        let span = Span::new(start, end);
        let location = Location::span(span.clone());

        assert_eq!(location.unwrap_span(), span);
    }

    #[test]
    fn should_be_able_to_extend_position() {
        let start = Position::new(1, 1, 1);
        let end = Position::new(2, 2, 3);
        let location = Location::position(start);
        let extended = location.extended_to(end);

        assert_eq!(extended.unwrap_span(), Span::new(start, end));
    }

    #[test]
    fn should_be_able_to_extend_span() {
        let start = Position::new(1, 1, 1);
        let end = Position::new(2, 2, 3);
        let location = Location::span(Span::new(start, end));
        let new_end = Position::new(3, 3, 6);
        let extended = location.extended_to(new_end);

        assert_eq!(extended.unwrap_span(), Span::new(start, new_end));
    }
}
