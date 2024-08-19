//! # mdast
//!
//! This library provides a thin layer on top of the [markdown] crate. This is done in order to use a standard [location::Location] type,
//! that is reused across all changelog CLI dependencies. It is also done to tweak the behavior of the underlying library by exposing normalization
//! functions and other utilities.
//!
//! At this moment, the library does not support the Mdx and frontmatter markdown extensions, since they are not prioritary to support on changelogs.
//!
//! ## Usage
//!
//! The main structure exported by this library is the [Mdast]. It represents a Markdown Abstract Syntax tree. You can get one from a file:
//! ```no_run
//! use mdast::Mdast;
//!
//! let mdast = Mdast::try_from_file("CHANGELOG.md").unwrap();
//! ```
//! Or from a string:
//! ```rust
//! use mdast::Mdast;
//! use std::str::FromStr;
//!
//! let mdast = Mdast::from_str("# Title\n\nSome content").unwrap();
//! ```
//!
//! You can then traverse the nodes of the tree and explore the AST generated.
mod convert;
mod error;
mod mdast;
mod node;
mod utils;

pub use error::*;
pub use mdast::*;
pub use node::*;
