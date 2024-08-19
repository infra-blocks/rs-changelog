//! This crate abstracts away the concept of a location in a text.
//!
//! A [Location] can refer to a single [Position] in text, or a [Span] between two positions.
//! It can be embedded in abstract tree nodes as well as errors, or anywhere else a text location can be meaningful.
//!
//! It provides a default implementation for displaying locations in a human-readable format. It is
//! available as an implementation of the [std::fmt::Display].
mod error;
mod location;
mod position;
mod span;

pub use location::*;
pub use position::*;
pub use span::*;
