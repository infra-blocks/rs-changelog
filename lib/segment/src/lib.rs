//! The segment crate provides types and traits to work with slices of texts.
//!
//! Unlike regular slices, these [Segment]s are made to remember their offset in the original text.
//! This is useful when parsing documents and constructing ASTs, as it allows to nodes to
//! remember their original position in the text.
//!
//! It also offers a [LineSegment] type, which represents whole lines of text.
//!
//! In addition, it provides some [str] utilities onto the type directly through the [SegmentStrExt] trait.
mod line;
mod segment;
mod segment_index;
mod segment_like;
mod segment_str_ext;

pub use line::*;
pub use segment::*;
pub use segment_index::*;
pub use segment_like::*;
pub use segment_str_ext::*;
