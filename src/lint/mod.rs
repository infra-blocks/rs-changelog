use std::{error::Error, fmt::Display};

use semver::Version;

use crate::Changelog;

impl<'source> Changelog<'source> {
    pub fn lint(&self) -> Result<(), ChangelogLintError> {
        // Check ordering of releases. Could be done during parsing?
        self.lint_release_version_in_descending_order()?;

        // Check ordering of change sets. Could be done during parsing?
        // Check ordering of ref defs.
        // Check links to be of the same form.
        Ok(())
    }

    fn lint_release_version_in_descending_order(&self) -> Result<(), ChangelogLintError> {
        let releases = self.releases();
        let mut previous_version: Option<&Version> = None;
        for release in releases {
            let current = release.version();
            if let Some(previous) = previous_version
                && previous <= current
            {
                return Err(ChangelogLintError::UnorderedReleases(
                    previous.clone(),
                    current.clone(),
                ));
            }
            previous_version = Some(current)
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangelogLintError {
    UnorderedReleases(Version, Version),
}

impl Display for ChangelogLintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangelogLintError::UnorderedReleases(first, second) => write!(
                f,
                "expected release version {} to come after {} to respect descending order",
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
        fn should_error_for_unordered_releases() {
            let changelog = Changelog::parse(
                r"# Changelog

This is a mfking changelog y'all.

## [0.2.0] - 2026-01-01

### Added

- Some bull.

## [0.3.0] - 2026-02-04

### Removed

- The bull. 

[0.3.0]: https://github.com/owner/repo/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/owner/repo/releases/tag/v0.2.0",
            )
            .unwrap();
            let result = changelog.lint();
            assert_eq!(
                result,
                Err(ChangelogLintError::UnorderedReleases(
                    Version::parse("0.2.0").unwrap(),
                    Version::parse("0.3.0").unwrap()
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
