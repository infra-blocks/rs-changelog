//! This module exposes utilities to parse single segments.
//!
//! Those utilities come in the form of [TryFrom] implementations that take in a [segment::Segment]
//! and returns a more informative type. For example, a [BlankLineSegment] can be constructed
//! using [TryFrom] using a [segment::Segment].
//!
//! These implementations use move semantics and return the invalid segment as an error in the
//! case where parsing fails.
//!
//! Note that each type is concerned solely with determining what is and isn't a valid segment.
//! They do not try to be *exclusively* valid. Meaning that there can be two types for which
//! the [TryFrom] implementation succeeds! The order in which the parsing must is up to the
//! client code.
mod atx_heading;
mod blank_line;
mod fenced_code;
mod indented_code;
mod link;
mod paragraph;
mod setext_heading;
mod thematic_break;

pub use atx_heading::*;
pub use blank_line::*;
pub use fenced_code::*;
pub use indented_code::*;
pub use link::*;
pub use paragraph::*;
pub use setext_heading::*;
pub use thematic_break::*;
