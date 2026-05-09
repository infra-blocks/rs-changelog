use std::ops::Range;

use crate::parse::{
    ast::Ast,
    releases::change_set::{ChangeSet, ChangeSetKind, ChangeSetParseError},
};

// TODO: wrap the *OPTION* in new type instead of the inner for clarity.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Changes {
    added: Option<Added>,
    changed: Option<Changed>,
    deprecated: Option<Deprecated>,
    removed: Option<Removed>,
    fixed: Option<Fixed>,
    security: Option<Security>,
}

impl Changes {
    pub fn try_new(
        added: Option<Added>,
        changed: Option<Changed>,
        deprecated: Option<Deprecated>,
        removed: Option<Removed>,
        fixed: Option<Fixed>,
        security: Option<Security>,
    ) -> Result<Self, ()> {
        if added.is_some()
            || changed.is_some()
            || deprecated.is_some()
            || removed.is_some()
            || fixed.is_some()
            || security.is_some()
        {
            Ok(Self::new(
                added, changed, deprecated, removed, fixed, security,
            ))
        } else {
            Err(())
        }
    }

    /// Returns the range covering the whole set of changes.
    #[allow(dead_code)]
    pub fn range(&self) -> Range<usize> {
        let mut start = usize::MAX;
        let mut end = usize::MIN;
        let mut update_range_bounds = |cs: &ChangeSet| {
            let cs_range = cs.range();
            if cs_range.start < start {
                start = cs_range.start;
            }
            if cs_range.end > end {
                end = cs_range.end;
            }
        };
        if let Some(cs) = &self.added {
            update_range_bounds(&cs.0)
        }
        if let Some(cs) = &self.changed {
            update_range_bounds(&cs.0)
        }
        if let Some(cs) = &self.deprecated {
            update_range_bounds(&cs.0)
        }
        if let Some(cs) = &self.removed {
            update_range_bounds(&cs.0)
        }
        if let Some(cs) = &self.fixed {
            update_range_bounds(&cs.0)
        }
        if let Some(cs) = &self.security {
            update_range_bounds(&cs.0)
        }

        Range { start, end }
    }

    pub(crate) fn parse(ast: &mut Ast) -> Result<Self, ChangesParseError> {
        let mut added: Option<Added> = None;
        let mut changed: Option<Changed> = None;
        let mut deprecated: Option<Deprecated> = None;
        let mut removed: Option<Removed> = None;
        let mut fixed: Option<Fixed> = None;
        let mut security: Option<Security> = None;

        let duplicate_change_set_err = |kind, first: ChangeSet, second: ChangeSet| {
            Err(ChangesParseError::DuplicateChangeSet {
                kind,
                first: first.range(),
                second: second.range(),
            })
        };

        loop {
            match ChangeSet::parse(ast) {
                Ok((kind, change_set)) => match kind {
                    ChangeSetKind::Added => match added {
                        Some(first) => {
                            return duplicate_change_set_err(kind, first.0, change_set);
                        }
                        _ => added = Some(Added(change_set)),
                    },
                    ChangeSetKind::Changed => match changed {
                        Some(first) => {
                            return duplicate_change_set_err(kind, first.0, change_set);
                        }
                        _ => changed = Some(Changed(change_set)),
                    },
                    ChangeSetKind::Deprecated => match deprecated {
                        Some(first) => {
                            return duplicate_change_set_err(kind, first.0, change_set);
                        }
                        _ => deprecated = Some(Deprecated(change_set)),
                    },
                    ChangeSetKind::Removed => match removed {
                        Some(first) => {
                            return duplicate_change_set_err(kind, first.0, change_set);
                        }
                        _ => removed = Some(Removed(change_set)),
                    },
                    ChangeSetKind::Fixed => match fixed {
                        Some(first) => {
                            return duplicate_change_set_err(kind, first.0, change_set);
                        }
                        _ => fixed = Some(Fixed(change_set)),
                    },
                    ChangeSetKind::Security => match security {
                        Some(first) => {
                            return duplicate_change_set_err(kind, first.0, change_set);
                        }
                        _ => security = Some(Security(change_set)),
                    },
                },
                // An error signals termination, not necessarily that there is an actual error.
                // There is only an error if no change set could be parsed before the first error.
                Err(err) => {
                    return Self::try_new(added, changed, deprecated, removed, fixed, security)
                        .map_err(|_| err.into());
                }
            }
        }
    }

    fn new(
        added: Option<Added>,
        changed: Option<Changed>,
        deprecated: Option<Deprecated>,
        removed: Option<Removed>,
        fixed: Option<Fixed>,
        security: Option<Security>,
    ) -> Self {
        Self {
            added,
            changed,
            deprecated,
            removed,
            fixed,
            security,
        }
    }
}

impl From<Added> for Changes {
    fn from(value: Added) -> Self {
        Changes::new(Some(value), None, None, None, None, None)
    }
}

impl From<Changed> for Changes {
    fn from(value: Changed) -> Self {
        Changes::new(None, Some(value), None, None, None, None)
    }
}

impl From<Deprecated> for Changes {
    fn from(value: Deprecated) -> Self {
        Changes::new(None, None, Some(value), None, None, None)
    }
}

impl From<Removed> for Changes {
    fn from(value: Removed) -> Self {
        Changes::new(None, None, None, Some(value), None, None)
    }
}

impl From<Fixed> for Changes {
    fn from(value: Fixed) -> Self {
        Changes::new(None, None, None, None, Some(value), None)
    }
}

impl From<Security> for Changes {
    fn from(value: Security) -> Self {
        Changes::new(None, None, None, None, None, Some(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangesParseError {
    InvalidChangeSet(ChangeSetParseError),
    DuplicateChangeSet {
        kind: ChangeSetKind,
        first: Range<usize>,
        second: Range<usize>,
    },
}

impl From<ChangeSetParseError> for ChangesParseError {
    fn from(value: ChangeSetParseError) -> Self {
        Self::InvalidChangeSet(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Added(ChangeSet);

impl From<ChangeSet> for Added {
    fn from(value: ChangeSet) -> Self {
        Added(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Changed(ChangeSet);

impl From<ChangeSet> for Changed {
    fn from(value: ChangeSet) -> Self {
        Changed(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deprecated(ChangeSet);

impl From<ChangeSet> for Deprecated {
    fn from(value: ChangeSet) -> Self {
        Deprecated(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Removed(ChangeSet);

impl From<ChangeSet> for Removed {
    fn from(value: ChangeSet) -> Self {
        Removed(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fixed(ChangeSet);

impl From<ChangeSet> for Fixed {
    fn from(value: ChangeSet) -> Self {
        Fixed(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Security(ChangeSet);

impl From<ChangeSet> for Security {
    fn from(value: ChangeSet) -> Self {
        Security(value)
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use changelog_ast::AstIterator;

    use crate::parse::releases::change_set::Change;

    use super::*;

    #[test]
    fn should_error_with_duplicate_change_set() {
        // TODO: AST struct bro
        let mut ast: VecDeque<_> = AstIterator::new(
            r"### Added
- stuff
### Added
- same stuff again. So good.",
        )
        .collect();
        let result = Changes::parse(&mut ast);
        assert_eq!(
            result,
            Err(ChangesParseError::DuplicateChangeSet {
                kind: ChangeSetKind::Added,
                first: 0..18,
                second: 18..56
            })
        );
    }

    #[test]
    fn should_work_with_all_6() {
        let mut ast: VecDeque<_> = AstIterator::new(
            r"### Added
- the stuff you don't want
### Changed
- the stuff you liked
### Deprecated
- the brand new stuff
### Removed
- the stuff you needed
### Fixed
- the stuff you didn't care about
### Security
- ooopsies
",
        )
        .collect();
        let result = Changes::parse(&mut ast);
        assert_eq!(
            result,
            Ok(Changes::new(
                Some(Added(ChangeSet::new(0..10, vec![Change::new(10..37)]))),
                Some(Changed(ChangeSet::new(37..49, vec![Change::new(49..71)]))),
                Some(Deprecated(ChangeSet::new(
                    71..86,
                    vec![Change::new(86..108)]
                ))),
                Some(Removed(ChangeSet::new(
                    108..120,
                    vec![Change::new(120..143)]
                ))),
                Some(Fixed(ChangeSet::new(143..153, vec![Change::new(153..187)]))),
                Some(Security(ChangeSet::new(
                    187..200,
                    vec![Change::new(200..211)]
                ))),
            ))
        );
    }
}
