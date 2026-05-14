use std::{error::Error, fmt::Display, ops::Range};

use chrono::NaiveDate;
use itertools::Itertools;
use semver::Version;

use crate::{Changelog, lint::ordered_change_set::OrderedChangeSet};

mod ordered_change_set;

impl<'source> Changelog<'source> {
    pub fn lint(&self) -> Result<(), ChangelogLintError> {
        // Check ordering of releases. Could be done during parsing?
        self.lint_release_versions_in_descending_order()?;
        // self.lint_no_gap_between_versions()?;
        self.lint_release_dates_in_descending_order()?;
        self.lint_release_change_sets_in_lexicographical_order()?;
        self.lint_reference_definitions_in_descending_order()?;
        // self.lint_no_gap_between_definitions()?;
        // self.lint_reference_definition_repository()?;
        // self.lint_reference_definition_links()?;

        Ok(())
    }

    fn lint_release_versions_in_descending_order(&self) -> Result<(), ChangelogLintError> {
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

    fn lint_release_dates_in_descending_order(&self) -> Result<(), ChangelogLintError> {
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

    fn lint_release_change_sets_in_lexicographical_order(&self) -> Result<(), ChangelogLintError> {
        if let Some(unreleased) = self.unreleased() {
            let changes = unreleased.changes();
            for (previous, current) in changes.iter().map(OrderedChangeSet).tuple_windows() {
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
            let changes = release.changes();
            for (previous, current) in changes.iter().map(OrderedChangeSet).tuple_windows() {
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

    fn lint_reference_definitions_in_descending_order(&self) -> Result<(), ChangelogLintError> {
        let reference_definitions = self.reference_definitions();
        for (previous, current) in reference_definitions.iter().tuple_windows() {
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
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangelogLintError {
    // TODO: store ranges instead?!?!?
    UnorderedReleaseVersions(Version, Version),
    UnorderedReleaseDates(NaiveDate, NaiveDate),
    UnorderedChangeSets(Range<usize>, Range<usize>),
    UnorderedReferenceDefinitions(Range<usize>, Range<usize>),
}

impl Display for ChangelogLintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangelogLintError::UnorderedReleaseVersions(first, second) => write!(
                f,
                "expected release version {} to come after {} to respect descending order",
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
