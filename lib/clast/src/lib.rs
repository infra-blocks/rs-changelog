//! This crate provides a simplified, flattened AST for changelogs that follow the
//! [keep-a-changelog](https://keepachangelog.com/) specification.
//!
//! It reduces a markdown AST produced by [mdast] into only the nodes relevant to a changelog.
//! Every node that isn't relevant to a changelog is treated as invalid.
//! For example, headings of depth over 3 are invalid, since they don't correspond to
//! anything in the specification. Ordered lists are invalid, since they aren't mentioned
//! in the specification, etc...
//!
//! Furthermore, the AST is flattened such that the text is aggregated from children nodes
//! and structured into a flat list of nodes.
//!
//! Besides those assumption, this crate doesn't enforce any rules. For example, it doesn't
//! check ordering or presence of items (it doesn't enforce the presence of a title, a description,
//! etc...).

mod changelog;
mod convert;
mod error;
mod node;

pub use changelog::*;
pub use error::*;
pub use node::*;
