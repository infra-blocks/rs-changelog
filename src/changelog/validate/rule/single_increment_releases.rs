use crate::changelog::validate::rule::Rule;
use crate::ValidationError;
use clast::Changelog;

struct SingleIncrementReleases;

impl Rule for SingleIncrementReleases {
    fn validate(&self, changelog: &Changelog) -> Result<(), ValidationError> {
        let releases = &changelog.change_sets;
        let mut sorted = releases.to_vec();
        sorted.sort_by(|left, right| left.version.cmp(&right.version));

        for i in 1..sorted.len() {
            let previous = &sorted[i - 1];
            let previous_version = &previous.version;
            let current = &sorted[i];
            let current_version = &current.version;

            if self.is_valid_patch_increment(previous_version, current_version) {
                continue;
            }
            if self.is_valid_minor_increment(previous_version, current_version) {
                continue;
            }
            if self.is_valid_major_increment(previous_version, current_version) {
                continue;
            }
            return Err(ValidationError::InvalidIncrementBetweenReleases(
                previous.clone(),
                current.clone(),
            ));
        }

        Ok(())
    }
}

impl SingleIncrementReleases {
    fn is_valid_patch_increment(
        &self,
        previous: &semver::Version,
        current: &semver::Version,
    ) -> bool {
        previous.patch + 1 == current.patch
            && previous.minor == current.minor
            && previous.major == current.major
    }

    fn is_valid_minor_increment(
        &self,
        previous: &semver::Version,
        current: &semver::Version,
    ) -> bool {
        previous.minor + 1 == current.minor && previous.major == current.major && current.patch == 0
    }

    fn is_valid_major_increment(
        &self,
        previous: &semver::Version,
        current: &semver::Version,
    ) -> bool {
        previous.major + 1 == current.major && current.minor == 0 && current.patch == 0
    }
}

pub fn single_increment_releases() -> impl Rule {
    SingleIncrementReleases
}

#[cfg(test)]
mod test {
    use super::*;

    mod validate {
        use super::*;

        // TODO: minimal changelog as a test_utils function.
        #[test]
        fn works_without_releases() {
            let changelog = r#"
# Changelog

The big daddy with the big booty.
"#
            .parse()
            .unwrap();
            assert!(single_increment_releases().validate(&changelog).is_ok());
        }

        #[test]
        fn works_with_single_release() {
            let changelog = r#"
# Changelog

The big daddy with the big booty.

## [1.0.0] - 2024-01-01

### Security

- Word.
"#
            .parse()
            .unwrap();
            assert!(single_increment_releases().validate(&changelog).is_ok());
        }

        #[test]
        fn works_with_a_patch_increment() {
            let changelog = r#"
# Changelog

The big daddy with the big booty.

## [0.0.2] - 2024-01-02

### Security

- Some secure.

## [0.0.1] - 2024-01-01

### Security

- No security at all.
"#
            .parse()
            .unwrap();
            assert!(single_increment_releases().validate(&changelog).is_ok());
        }

        #[test]
        fn works_with_a_minor_increment() {
            let changelog = r#"
# Changelog

The big daddy with the big booty.

## [0.1.0] - 2024-01-02

### Security

- Some secure.

## [0.0.1] - 2024-01-01

### Security

- No security at all.
"#
            .parse()
            .unwrap();
            assert!(single_increment_releases().validate(&changelog).is_ok());
        }

        #[test]
        fn works_with_a_major_increment() {
            let changelog = r#"
# Changelog

The big daddy with the big booty.

## [1.0.0] - 2024-01-02

### Security

- Some secure.

## [0.0.1] - 2024-01-01

### Security

- No security at all.
"#
            .parse()
            .unwrap();
            assert!(single_increment_releases().validate(&changelog).is_ok());
        }

        // This is just to showcase that this rule does not check for ordering.
        #[test]
        fn works_with_unordered_releases() {
            let changelog = r#"
# Changelog

The big daddy with the big booty.

## [0.0.1] - 2024-01-01

### Security

- No security at all.

## [1.0.0] - 2024-01-02

### Security

- Some secure.
"#
            .parse()
            .unwrap();
            assert!(single_increment_releases().validate(&changelog).is_ok());
        }

        #[test]
        fn fails_with_two_releases_with_same_version() {
            let changelog = r#"
# Changelog

The big daddy with the big booty.

## [0.0.1] - 2024-01-02

### Security

- Some secure.

## [0.0.1] - 2024-01-01

### Security

- No security at all.
"#
            .parse()
            .unwrap();
            assert!(matches!(
                single_increment_releases()
                    .validate(&changelog)
                    .unwrap_err(),
                ValidationError::InvalidIncrementBetweenReleases(_, _)
            ));
        }

        #[test]
        fn fails_with_two_releases_with_invalid_patch_increment() {
            let changelog = r#"
# Changelog

The big daddy with the big booty.

## [0.0.3] - 2024-01-02

### Security

- Some secure.

## [0.0.1] - 2024-01-01

### Security

- No security at all.
"#
            .parse()
            .unwrap();
            assert!(matches!(
                single_increment_releases()
                    .validate(&changelog)
                    .unwrap_err(),
                ValidationError::InvalidIncrementBetweenReleases(_, _)
            ));
        }

        #[test]
        fn fails_with_two_releases_with_invalid_minor_increment() {
            let changelog = r#"
# Changelog

The big daddy with the big booty.

## [0.1.1] - 2024-01-02

### Security

- Some secure.

## [0.0.1] - 2024-01-01

### Security

- No security at all.
"#
            .parse()
            .unwrap();
            assert!(matches!(
                single_increment_releases()
                    .validate(&changelog)
                    .unwrap_err(),
                ValidationError::InvalidIncrementBetweenReleases(_, _)
            ));
        }

        #[test]
        fn fails_with_two_releases_with_invalid_major_increment() {
            let changelog = r#"
# Changelog

The big daddy with the big booty.

## [2.0.0] - 2024-01-02

### Security

- Some secure.

## [0.0.1] - 2024-01-01

### Security

- No security at all.
"#
            .parse()
            .unwrap();
            assert!(matches!(
                single_increment_releases()
                    .validate(&changelog)
                    .unwrap_err(),
                ValidationError::InvalidIncrementBetweenReleases(_, _)
            ));
        }
    }
}
