use segment::{LineSegment, Segment};

pub trait IntoSegments<'a> {
    fn into_segments(self) -> impl Iterator<Item = Segment<'a>>;
}

pub trait IntoLineSegments<'a> {
    fn into_line_segments(self) -> impl Iterator<Item = LineSegment<'a>>;
}

impl<'a, T> IntoSegments<'a> for T
where
    T: IntoLineSegments<'a>,
{
    fn into_segments(self) -> impl Iterator<Item = Segment<'a>> {
        self.into_line_segments().map(Segment::from)
    }
}
