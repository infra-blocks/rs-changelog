use crate::changelog::validate::rule::Rule;
use crate::ValidationError;
use clast;
use semver::Version;

struct ReleasesStartAt {
    start: Version,
}

impl ReleasesStartAt {
    fn new(start: Version) -> Self {
        Self { start }
    }
}

impl Rule for ReleasesStartAt {
    fn validate(&self, changelog: &clast::Changelog) -> Result<(), ValidationError> {
        let releases = &changelog.releases;
        let mut sorted = releases.to_vec();
        sorted.sort_by(|left, right| left.version.cmp(&right.version));

        let Some(first_release) = sorted.first() else {
            return Ok(());
        };

        if first_release.version != self.start {
            return Err(ValidationError::WrongFirstRelease(
                self.start.clone(),
                first_release.clone(),
            ));
        }
        Ok(())
    }
}

pub fn releases_start_at<T: Into<Version>>(start: T) -> impl Rule {
    ReleasesStartAt::new(start.into())
}

#[cfg(test)]
mod test {
    use super::*;

    mod validate {
        use super::*;

        #[test]
        fn works_without_releases() {
            let changelog = r#"
# Changelog

Dis here some tests.
"#
            .parse()
            .unwrap();
            assert!(releases_start_at(Version::new(0, 1, 4))
                .validate(&changelog)
                .is_ok());
        }

        #[test]
        fn works_with_one_release_that_matches() {
            let changelog = r#"
# Changelog

Dis here some tests.

## [1.5.0] - 2020-01-01

### Added

- Some bull.
"#
            .parse()
            .unwrap();
            assert!(releases_start_at(Version::new(1, 5, 0))
                .validate(&changelog)
                .is_ok());
        }

        #[test]
        fn works_with_multiple_unordered_releases() {
            let changelog = r#"
# Changelog

Dis here some tests.

## [0.2.0] - 2020-01-01

### Added

- Some bull.

## [0.1.0] - 2020-01-01

### Added

- More bull.

## [1.3.0] - 2020-01-01

### Added

- All the bull.
"#
            .parse()
            .unwrap();
            let result = releases_start_at(Version::new(0, 1, 0)).validate(&changelog);
            assert!(result.is_ok());
        }

        #[test]
        fn fails_when_first_release_does_not_match() {
            let changelog = r#"
# Changelog

Dis here some tests.

## [0.2.0] - 2020-01-01

### Added

- Some bull.

## [0.1.0] - 2020-01-01

### Added

- More bull.

## [1.3.0] - 2020-01-01

### Added

- All the bull.
"#
            .parse()
            .unwrap();
            let result = releases_start_at(Version::new(0, 2, 0)).validate(&changelog);
            assert!(matches!(
                result.unwrap_err(),
                ValidationError::WrongFirstRelease(_, _)
            ));
        }
    }
}
