//! This crate abstracts away the concept of locations in a text.
//!
//! A [Location] can refer to a single [Position] in text, or a [Span] between two positions.
//!
//! A [Position] is made up of a line number, a column (counted by characters), and an offset (in bytes).
//! A [Span] is simply a range covering two positions.
//!
//! This crate also exposes the concept of textual [Segment]s, which are used to represent a portion of
//! text that is not aware of its position. [Segment]s can be used to conveniently calculate a new [Position],
//! based on the addition of a [Position] and a [Segment].
//!
//! No structs in this crate keep references to the original text. They only store the positional information.
//! Therefore, their implementation of [Eq] and [PartialEq] is based on the positional information only.
//!
//! The crate provides default implementations for displaying locations in a human-readable format. It is
//! available as an implementation of the [std::fmt::Display].
mod error;
mod location;
mod position;
mod span;

pub use location::*;
pub use position::*;
pub use span::*;
