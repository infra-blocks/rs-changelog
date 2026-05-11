use std::{error::Error, fmt::Display};

use chrono::NaiveDate;
use itertools::Itertools;
use semver::Version;

use crate::Changelog;

impl<'source> Changelog<'source> {
    pub fn lint(&self) -> Result<(), ChangelogLintError> {
        // Check ordering of releases. Could be done during parsing?
        self.lint_release_version_in_descending_order()?;
        self.lint_release_date_in_descending_order()?;

        // Check ordering of change sets. Could be don
        // e during parsing?
        // Check ordering of ref defs.
        // Check links to be of the same form.
        Ok(())
    }

    fn lint_release_version_in_descending_order(&self) -> Result<(), ChangelogLintError> {
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

    fn lint_release_date_in_descending_order(&self) -> Result<(), ChangelogLintError> {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangelogLintError {
    UnorderedReleaseVersions(Version, Version),
    UnorderedReleaseDates(NaiveDate, NaiveDate),
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
