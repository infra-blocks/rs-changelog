//! Date utilities for the changelog crate.
//!
//! This crate merely provides a thin wrapper over the chrono library for the features that
//! the changelog crate needs. It acts as a layer of indirection should allow for an easier
//! time keeping backwards compatibility of the changelog crate API.

mod date;

pub use date::*;
