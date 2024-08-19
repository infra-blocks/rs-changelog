use stack::Stack;

use crate::{Error, Title};

/// The changelog is the main struct of this crate.
///
/// It is constructed from a [clast::Changelog] and holds and organize
/// several fields:
/// - a [Title]
///
/// This changelog is well-formed: it will
/// be constructed from the subset of markdown that is applicable to changelogs
/// and will have the ordering of its fields enforced.
#[derive(Debug, Clone)]
pub struct Changelog {
    /// The changelog [Title].
    pub title: Title,
}

impl TryFrom<clast::Changelog> for Changelog {
    type Error = Error;

    fn try_from(changelog: clast::Changelog) -> Result<Self, Self::Error> {
        let mut nodes = Stack::from_iter(changelog.nodes);
        Self::try_from(&mut nodes)
    }
}

impl TryFrom<&mut Stack<clast::NodeKind>> for Changelog {
    type Error = Error;

    fn try_from(nodes: &mut Stack<clast::NodeKind>) -> Result<Self, Self::Error> {
        let title = nodes.try_into()?;

        Ok(Self { title })
    }
}
