use std::ops::Range;

use crate::parse::{
    ast::Ast,
    releases::change_set::{ChangeSet, ChangeSetParseError},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Changes {
    change_sets: Vec<ChangeSet>,
}

impl Changes {
    pub(crate) fn iter(&self) -> std::slice::Iter<'_, ChangeSet> {
        self.change_sets.iter()
    }

    pub(crate) fn parse(ast: &mut Ast) -> Result<Self, ChangesParseError> {
        let mut change_sets: Vec<ChangeSet> = vec![];
        loop {
            match ChangeSet::parse(ast) {
                Ok(change_set) => {
                    if let Some(first) = change_sets.iter().find(|cs| cs.is_same_kind(&change_set))
                    {
                        return Err(ChangesParseError::DuplicateChangeSet {
                            first: first.range(),
                            second: change_set.range(),
                        });
                    }
                    change_sets.push(change_set);
                }
                // An error signals termination, not necessarily that there is an actual error.
                // There is only an error if no change set could be parsed before the first error.
                Err(err) => {
                    if change_sets.is_empty() {
                        return Err(err.into());
                    } else {
                        return Ok(Self::new(change_sets));
                    }
                }
            }
        }
    }

    fn new(change_sets: Vec<ChangeSet>) -> Self {
        Self { change_sets }
    }
}

impl<T: Into<ChangeSet>> From<T> for Changes {
    fn from(value: T) -> Self {
        Changes::new(vec![value.into()])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangesParseError {
    InvalidChangeSet(ChangeSetParseError),
    DuplicateChangeSet {
        first: Range<usize>,
        second: Range<usize>,
    },
}

impl From<ChangeSetParseError> for ChangesParseError {
    fn from(value: ChangeSetParseError) -> Self {
        Self::InvalidChangeSet(value)
    }
}

#[cfg(test)]
mod test {

    use crate::parse::releases::{Added, Change, Changed, Deprecated, Fixed, Removed, Security};

    use super::*;

    #[test]
    fn should_error_with_duplicate_change_set() {
        let mut ast = Ast::from(
            r"### Added
- stuff
### Added
- same stuff again. So good.",
        );
        let result = Changes::parse(&mut ast);
        assert_eq!(
            result,
            Err(ChangesParseError::DuplicateChangeSet {
                first: 0..18,
                second: 18..56
            })
        );
    }

    #[test]
    fn should_work_with_all_6() {
        let mut ast = Ast::from(
            r"### Added
- the stuff you don't want
### Changed
- the stuff you liked
### Deprecated
- the brand new stuff
### Fixed
- the stuff you didn't care about
### Removed
- the stuff you needed
### Security
- ooopsies
",
        );
        let result = Changes::parse(&mut ast);
        assert_eq!(
            result,
            Ok(Changes::new(vec![
                Added::new(0..10, vec![Change::new(10..37)]).into(),
                Changed::new(37..49, vec![Change::new(49..71)]).into(),
                Deprecated::new(71..86, vec![Change::new(86..108)]).into(),
                Fixed::new(108..118, vec![Change::new(118..152)]).into(),
                Removed::new(152..164, vec![Change::new(164..187)]).into(),
                Security::new(187..200, vec![Change::new(200..211)]).into(),
            ]))
        );
    }
}
