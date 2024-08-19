//! This is a small crate providing a utility trait with a blanket implementation.
//!
//! The trait in question is [MaybeDisplay]. It is used for types that can be displayed conditionally.
//! The blanket implementation is provided for [`Option<T>`] when `T` implements [fmt::Display].
//!
//! This means that it can be used like this:
//! ```
//! use std::fmt;
//! use maybe_display::MaybeDisplay;
//!
//! enum Size {
//!   Small,
//!   Big,
//! }
//!
//! impl fmt::Display for Size {
//!   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!     match self {
//!       Size::Small => write!(f, "Small "),
//!       Size::Big => write!(f, "Big "),
//!     }
//!   }
//! }
//!
//! struct Pizza {
//!   size: Option<Size>
//! }
//!
//! impl fmt::Display for Pizza {
//!   // Will print either "Small Pizzer", "Big Pizzer" or "Pizzer" when size is None.
//!   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!     self.size.maybe_fmt(f)?;
//!     write!(f, "Pizzer")
//!   }
//! }
//!
//! ```
//!
//! The blanket implementation simply does nothing if the value is [Option::None], or calls [fmt::Display::fmt] on the wrapped value
//! otherwise.
use std::fmt;

/// A trait for types that can be displayed conditionally.
pub trait MaybeDisplay {
    /// Conditionally formats the value.
    fn maybe_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<T: fmt::Display> MaybeDisplay for Option<T> {
    fn maybe_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }
}
