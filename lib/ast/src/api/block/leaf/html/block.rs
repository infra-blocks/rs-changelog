use segment::Segment;

pub struct Html<'a> {
    pub segments: Vec<Segment<'a>>,
}

// HTML blocks have 7 opening and closing conditions.
