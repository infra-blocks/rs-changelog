use super::Segment;

pub trait IntoSegments<'a> {
    type IntoIter: Iterator<Item = Segment<'a>>;

    fn into_segments(self) -> Self::IntoIter;
}
