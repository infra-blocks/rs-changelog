use std::ops::Range;

use pulldown_cmark::Event;

pub type MarkdownItem<'source> = (Event<'source>, Range<usize>);
