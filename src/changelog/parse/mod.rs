/// This module is a translation of the markdown changelog into a parsed version.
/// The parsing references text processing. We unpack and parse the text found in nodes
/// into a structured changelog. Some transformations could fail.
mod change;
mod changelog;
mod description;
mod error;
mod link;
mod release;
mod text;
mod title;

pub use crate::changelog::position::*;
pub use change::*;
pub use changelog::*;
pub use description::*;
pub use link::*;
pub use release::*;
pub use title::*;
