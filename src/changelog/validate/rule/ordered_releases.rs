use crate::changelog::validate::rule::Rule;
use crate::ValidationError;
use clast;

struct OrderedReleases;

impl Rule for OrderedReleases {
    fn validate(&self, changelog: &clast::Changelog) -> Result<(), ValidationError> {
        let releases = &changelog.change_sets;
        if releases.is_empty() || releases.len() == 1 {
            return Ok(());
        }

        // Read in the order they are written. So "previous" release should be higher.
        for i in 1..releases.len() {
            let previous_release = &releases[i - 1];
            let previous_version = &previous_release.version;
            let current_release = &releases[i];
            let current_version = &current_release.version;

            if previous_version <= current_version {
                return Err(ValidationError::UnorderedReleases(
                    previous_release.clone(),
                    current_release.clone(),
                ));
            }
        }
        Ok(())
    }
}

pub fn ordered_releases() -> impl Rule {
    OrderedReleases
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

Some bullsheetz.
"#
            .parse()
            .unwrap();
            assert!(ordered_releases().validate(&changelog).is_ok());
        }

        #[test]
        fn works_a_single_release() {
            let changelog = r#"
# Changelog

Big releases.

## [1.0.0] - 2020-01-01

### Added

- You don't wanna know.
"#
            .parse()
            .unwrap();
            assert!(ordered_releases().validate(&changelog).is_ok());
        }

        #[test]
        fn works_with_ordered_releases() {
            let changelog = r#"
# Changelog

Big releases.

## [1.0.0] - 2020-01-01

### Added

- You don't wanna know.

## [0.2.0] - 2019-12-01

### Added

- The shit that's been said.

## [0.1.0] - 2019-11-01

### Added

- About Trump's shooting y'all.

## [0.0.1] - 2019-10-01

### Security

- Stopped checking for potential sniper mirador around presidential press releases.
"#
            .parse()
            .unwrap();
            assert!(ordered_releases().validate(&changelog).is_ok());
        }

        #[test]
        fn fails_with_unordered_releases() {
            let changelog = r#"
# Changelog

Big releases.

## [1.0.0] - 2020-01-01

### Added

- You don't wanna know.

## [0.1.0] - 2019-11-01

### Added

- About Trump's shooting y'all.

## [0.2.0] - 2019-12-01

### Added

- The shit that's been said.

## [0.0.1] - 2019-10-01

### Security

- Stopped checking for potential sniper mirador around presidential press releases.
"#
            .parse()
            .unwrap();
            assert!(matches!(
                ordered_releases().validate(&changelog).unwrap_err(),
                ValidationError::UnorderedReleases(_, _)
            ));
        }
    }
}
