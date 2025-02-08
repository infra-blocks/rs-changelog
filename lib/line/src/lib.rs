//! This crate exposes utilities to enhance the typings around text lines.
//!
//! The main type is [LineSlice], which is a thin wrapper around string slices that enforces
//! that the inner text does not container any non-terminating newline characters. In other
//! words, if there is a newline character, then it must be the last character of the slice.
//!
//! [LineSlice]s can be constructed directly, but the intended way to obtain them is through
//! the [str] extension trait [ByLines], which adds the [by_lines] method to string slices.
//!
//! [by_lines] returns a [Lines] iterator that wraps every line of text in a [LineSlice].
//! [LineSlice]s can be further reduced through slicing, and the invariant remains valid.
//!
//! # Example
//!
//! ```
//! use line::{ByLines, LineSlice};
//!
//! let text = r"Some multiline
//! text
//! you wouldn't believe.";
//! let lines = text.by_lines().collect::<Vec<_>>();
//! assert_eq!(
//!     lines,
//!     vec![
//!         LineSlice::new("Some multiline\n"),
//!         LineSlice::new("text\n"),
//!         LineSlice::new("you wouldn't believe.")
//!     ]
//! );
//! assert_eq!(lines[0].line_text(), "Some multiline");
//! assert_eq!(lines[2].line_text(), "you wouldn't believe.");
//! ```
mod by_lines;
mod line_slice;
mod lines;

pub use by_lines::*;
pub use line_slice::*;
pub use lines::*;
