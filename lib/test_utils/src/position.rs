use markdown::unist;

pub struct PositionFields {
    pub start: unist::Point,
    pub end: unist::Point,
}

// TODO: random instead.
impl Default for PositionFields {
    fn default() -> Self {
        Self {
            start: unist::Point::new(1, 1, 0),
            end: unist::Point::new(1, 10, 10),
        }
    }
}

impl From<()> for PositionFields {
    fn from(_: ()) -> Self {
        Default::default()
    }
}

// TODO: randomize positions a little.
pub fn position<T: Into<PositionFields>>(fields: T) -> unist::Position {
    let fields = fields.into();
    unist::Position::new(
        fields.start.line,
        fields.start.column,
        fields.start.offset,
        fields.end.line,
        fields.end.column,
        fields.end.offset,
    )
}
