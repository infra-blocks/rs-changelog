use std::{collections::HashSet, error::Error, fmt::Display, ops::Range};

use chrono::NaiveDate;
use itertools::Itertools;
use semver::Version;

use crate::{
    Changelog,
    lint::{
        ordered_change_set::OrderedChangeSet,
        ref_def_linters::{RefDefLintError, RefDefLinter},
        version_gap::versions_differ_by_one,
    },
};

mod ordered_change_set;
mod ref_def_linters;
mod version_gap;

impl<'source> Changelog<'source> {
    pub fn lint(&self) -> Result<(), ChangelogLintError> {
        dbg!("self.release_versions_in_descending_order");
        self.release_versions_in_descending_order()?;
        dbg!("self.no_gap_between_versions");
        self.no_gap_between_versions()?;
        dbg!("self.release_dates_in_descending_order");
        self.release_dates_in_descending_order()?;
        // TODO: optional
        // self.release_change_sets_in_lexicographical_order()?;
        dbg!("self.reference_definitions_in_descending_order");
        self.reference_definitions_in_descending_order()?;
        dbg!("self.no_dangling_reference_definitions");
        self.no_dangling_reference_definitions()?;
        dbg!("self.valid_reference_definition_destinations");
        self.valid_reference_definition_destinations()?;
        Ok(())
    }

    fn release_versions_in_descending_order(&self) -> Result<(), ChangelogLintError> {
        let releases = self.releases();
        for (previous, current) in releases.iter().map(|r| r.version()).tuple_windows() {
            // Releases are unique so they can't be the same neither. TODO: different error type?
            if previous <= current {
                return Err(ChangelogLintError::UnorderedReleaseVersions(
                    previous.clone(),
                    current.clone(),
                ));
            }
        }
        Ok(())
    }

    fn no_gap_between_versions(&self) -> Result<(), ChangelogLintError> {
        let releases = self.releases();
        for (previous, current) in releases.iter().map(|r| r.version()).tuple_windows() {
            if !versions_differ_by_one(current, previous) {
                return Err(ChangelogLintError::GapBetweenVersions(
                    previous.clone(),
                    current.clone(),
                ));
            }
        }
        Ok(())
    }

    fn release_dates_in_descending_order(&self) -> Result<(), ChangelogLintError> {
        let releases = self.releases();
        for (previous, current) in releases.iter().map(|r| r.date()).tuple_windows() {
            // The date could be the same, since it's a granularity of one day.
            if previous < current {
                return Err(ChangelogLintError::UnorderedReleaseDates(
                    *previous, *current,
                ));
            }
        }
        Ok(())
    }

    fn release_change_sets_in_lexicographical_order(&self) -> Result<(), ChangelogLintError> {
        if let Some(unreleased) = self.unreleased() {
            let change_sets = unreleased.change_sets();
            for (previous, current) in change_sets.map(OrderedChangeSet).tuple_windows() {
                if previous >= current {
                    return Err(ChangelogLintError::UnorderedChangeSets(
                        previous.0.range(),
                        current.0.range(),
                    ));
                }
            }
        }

        let releases = self.releases();
        for release in releases {
            for (previous, current) in release.change_sets().map(OrderedChangeSet).tuple_windows() {
                if previous >= current {
                    return Err(ChangelogLintError::UnorderedChangeSets(
                        previous.0.range(),
                        current.0.range(),
                    ));
                }
            }
        }
        Ok(())
    }

    // TODO: unit test the special unreleased case.
    fn reference_definitions_in_descending_order(&self) -> Result<(), ChangelogLintError> {
        let mut iter = self.reference_definitions().iter();
        if let Some(_) = self.unreleased() {
            // TODO: enforce the label is unreleased OR ELSE.
            iter.next().unwrap();
        }

        for (previous, current) in iter.tuple_windows() {
            let previous_version = Version::parse(previous.label()).unwrap();
            let current_version = Version::parse(current.label()).unwrap();
            if previous_version <= current_version {
                return Err(ChangelogLintError::UnorderedReferenceDefinitions(
                    previous.range().clone(),
                    current.range().clone(),
                ));
            }
        }
        Ok(())
    }

    pub fn no_dangling_reference_definitions(&self) -> Result<(), ChangelogLintError> {
        let mut iter = self.reference_definitions().iter();
        if let Some(_) = self.unreleased() {
            iter.next().unwrap();
        }

        // This lint assumes the parsing eliminates all releases with broken links. So all the releases
        // have working reference definitions, but the opposite is not necessarily true.
        let release_versions: HashSet<_> = self
            .releases()
            .iter()
            .map(|r| r.version().clone())
            .collect();

        for def in iter {
            if !release_versions.contains(&Version::parse(def.label()).unwrap()) {
                return Err(ChangelogLintError::DanglingReferenceDefinition(
                    def.range().clone(),
                ));
            }
        }
        Ok(())
    }

    /// We're going over all the reference definitions, enforcing their destination URLs are both consistent
    /// and valid for their given version control provider.
    pub fn valid_reference_definition_destinations(&self) -> Result<(), ChangelogLintError> {
        // The changelog parsing requires at least one of unreleased or one released version,
        // guaranteeing, that there is going to be at least one ref def.
        let first = self.reference_definitions().iter().next_back().unwrap();
        let linter = RefDefLinter::try_new(first).ok_or(
            ChangelogLintError::UnknownReferenceDefinitionFormat(first.range().clone()),
        )?;
        // We only expect the first release, the one at the bottom, to be categorized as a "release"
        // definition. All other entries should be diffs definition with the previous version.
        linter.lint_release_definition(first)?;

        // Now we restart the iteration and we go in pairs.
        for (previous, current) in self.reference_definitions().iter().rev().tuple_windows() {
            linter.lint_diff_definition(previous, current)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangelogLintError {
    // TODO: store ranges instead?!?!?
    UnorderedReleaseVersions(Version, Version),
    GapBetweenVersions(Version, Version),
    UnorderedReleaseDates(NaiveDate, NaiveDate),
    UnorderedChangeSets(Range<usize>, Range<usize>),
    UnorderedReferenceDefinitions(Range<usize>, Range<usize>),
    DanglingReferenceDefinition(Range<usize>),
    UnknownReferenceDefinitionFormat(Range<usize>),
    InvalidRerenceDefinition(RefDefLintError),
}

impl From<RefDefLintError> for ChangelogLintError {
    fn from(value: RefDefLintError) -> Self {
        Self::InvalidRerenceDefinition(value)
    }
}

impl Display for ChangelogLintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangelogLintError::UnorderedReleaseVersions(first, second) => write!(
                f,
                "expected release version {} to come after {} to respect descending order",
                first, second
            ),
            ChangelogLintError::GapBetweenVersions(first, second) => write!(
                f,
                "expected release version {} to differ with previous version {} by only one",
                first, second
            ),
            ChangelogLintError::UnorderedReleaseDates(first, second) => write!(
                f,
                "expected release date {} to come after {} to respect descending order",
                first, second
            ),
            ChangelogLintError::UnorderedChangeSets(first, second) => {
                write!(
                    f,
                    "expected change set {:?} to come after {:?}",
                    first, second
                )
            }
            ChangelogLintError::UnorderedReferenceDefinitions(first, second) => write!(
                f,
                "expected reference definition {:?} to come before {:?}",
                first, second
            ),
            ChangelogLintError::DanglingReferenceDefinition(range) => {
                write!(f, "found dangling reference definition {:?}", range)
            }
            ChangelogLintError::UnknownReferenceDefinitionFormat(range) => {
                write!(f, "unknown reference definition format found {:?}", range)
            }
            ChangelogLintError::InvalidRerenceDefinition(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ChangelogLintError {}

#[cfg(test)]
mod test {
    use super::*;

    mod lint {
        use super::*;

        #[test]
        fn should_error_for_unordered_release_versions() {
            let changelog = Changelog::parse(
                r"# Changelog

This is a mfking changelog y'all.

## [0.1.0] - 2026-02-01

### Added

- Some bull.

## [0.2.0] - 2026-01-01

### Removed

- The bull. 

[0.2.0]: https://github.com/owner/repo/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/owner/repo/releases/tag/v0.1.0",
            )
            .unwrap();
            let result = changelog.lint();
            assert_eq!(
                result,
                Err(ChangelogLintError::UnorderedReleaseVersions(
                    Version::parse("0.1.0").unwrap(),
                    Version::parse("0.2.0").unwrap()
                ))
            );
        }

        #[test]
        fn should_error_with_version_gap() {
            let changelog = Changelog::parse(
                r"# Changelog

This is a mfking changelog y'all.

## [0.2.1] - 2026-02-04

### Removed

- The bull.

## [0.1.0] - 2026-01-01

### Added

- Some bull.

[0.2.1]: https://github.com/owner/repo/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/owner/repo/releases/tag/v0.1.0",
            )
            .unwrap();
            let result = changelog.lint();
            assert_eq!(
                result,
                Err(ChangelogLintError::GapBetweenVersions(
                    Version::new(0, 2, 1),
                    Version::new(0, 1, 0)
                ))
            );
        }

        #[test]
        fn should_error_for_unordered_release_dates() {
            let changelog = Changelog::parse(
                r"# Changelog

This is a mfking changelog y'all.

## [0.2.0] - 2026-01-01

### Removed

- The bull. 

## [0.1.0] - 2026-02-01

### Added

- Some bull.

[0.2.0]: https://github.com/owner/repo/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/owner/repo/releases/tag/v0.1.0",
            )
            .unwrap();
            let result = changelog.lint();
            assert_eq!(
                result,
                Err(ChangelogLintError::UnorderedReleaseDates(
                    NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2026, 2, 1).unwrap()
                ))
            );
        }

        #[ignore]
        #[test]
        fn should_error_for_unordered_change_sets() {
            let changelog = Changelog::parse(
                r"# Changelog

This is a mfking changelog y'all.

## [0.1.0] - 2026-02-01

### Removed

- The same bull just added.

### Added

- Some bull.

[0.1.0]: https://github.com/owner/repo/releases/tag/v0.1.0",
            )
            .unwrap();
            let result = changelog.lint();
            assert_eq!(
                result,
                Err(ChangelogLintError::UnorderedChangeSets(73..115, 115..140))
            );
        }

        #[ignore]
        #[test]
        fn should_error_for_unordered_unreleased_change_sets() {
            let changelog = Changelog::parse(
                r"# Changelog

This is a mfking changelog y'all.

## [Unreleased]

### Removed

- The same bull just added.

### Added

- Some bull.

[Unreleased]: https://github.com/owner/repo/releases/tag/v0.1.0",
            )
            .unwrap();
            let result = changelog.lint();
            assert_eq!(
                result,
                Err(ChangelogLintError::UnorderedChangeSets(65..107, 107..132))
            );
        }

        #[test]
        fn should_error_for_unordered_reference_definitions() {
            let changelog = Changelog::parse(
                r"# Changelog

This is a mfking changelog y'all.

## [0.2.0] - 2026-02-04

### Removed

- The bull.

## [0.1.0] - 2026-01-01

### Added

- Some bull.

[0.1.0]: https://github.com/owner/repo/releases/tag/v0.1.0
[0.2.0]: https://github.com/owner/repo/compare/v0.1.0...v0.2.0",
            )
            .unwrap();
            let result = changelog.lint();
            assert_eq!(
                result,
                Err(ChangelogLintError::UnorderedReferenceDefinitions(
                    149..207,
                    208..270
                ))
            );
        }

        #[test]
        fn should_error_with_dangling_reference_definition() {
            let changelog = Changelog::parse(
                r"# Changelog

This is a mfking changelog y'all.

## [0.2.0] - 2026-02-04

### Removed

- The bull.

## [0.1.0] - 2026-01-01

### Added

- Some bull.

[0.2.0]: https://github.com/owner/repo/compare/v0.1.0...v0.2.0
[0.1.1]: https://github.com/owner/repo/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/owner/repo/releases/tag/v0.1.0",
            )
            .unwrap();
            let result = changelog.lint();
            assert_eq!(
                result,
                Err(ChangelogLintError::DanglingReferenceDefinition(212..274))
            );
        }

        #[test]
        fn should_error_with_invalid_reference_definition_destination() {
            let changelog = Changelog::parse(
                r"# Changelog

This is a mfking changelog y'all.

## [0.2.0] - 2026-02-04

### Removed

- The bull.

## [0.1.0] - 2026-01-01

### Added

- Some bull.

[0.2.0]: https://gitlab.com/owner/repo/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/owner/repo/releases/tag/v0.1.0",
            )
            .unwrap();
            let result = changelog.lint();
            assert!(
                matches!(result, Err(ChangelogLintError::InvalidRerenceDefinition(_))),
                "{:?}",
                result
            );
        }

        #[test]
        fn should_work_with_valid_changelog() {
            let changelog = Changelog::parse(
                r"# Changelog

This is a mfking changelog y'all.

## [0.2.0] - 2026-02-04

### Removed

- The bull.

## [0.1.0] - 2026-01-01

### Added

- Some bull.

[0.2.0]: https://github.com/owner/repo/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/owner/repo/releases/tag/v0.1.0",
            )
            .unwrap();
            let result = changelog.lint();
            assert_eq!(result, Ok(()));
        }
    }
}
