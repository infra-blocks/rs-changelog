use crate::changelog::validate::rule::Rule;
use crate::ValidationError;
use clast;
use semver::Version;
use std::collections::HashSet;

struct ReleasesMatchLinks;

impl Rule for ReleasesMatchLinks {
    fn validate(&self, changelog: &clast::Changelog) -> Result<(), ValidationError> {
        let releases: HashSet<&Version> = changelog
            .change_sets
            .iter()
            .map(|release| &release.version)
            .collect();
        let links: HashSet<&Version> = changelog.links.iter().map(|link| &link.version).collect();

        if releases != links {
            let missing_links: Vec<_> = releases.difference(&links).collect();
            if missing_links.len() > 0 {
                return Err(ValidationError::MissingLinks(
                    missing_links
                        .iter()
                        .map(|version| version.to_string())
                        .collect(),
                ));
            }

            let dangling_links: Vec<_> = links.difference(&releases).collect();
            if dangling_links.len() > 0 {
                return Err(ValidationError::DanglingLinks(
                    dangling_links
                        .iter()
                        .map(|version| version.to_string())
                        .collect(),
                ));
            }
        }

        Ok(())
    }
}

pub fn releases_match_links() -> impl Rule {
    ReleasesMatchLinks
}

#[cfg(test)]
mod test {
    use super::*;

    mod validate {
        use super::*;

        #[test]
        fn works_without_links_or_releases() {
            let changelog = r#"
# Changelog

For tests yo.
"#
            .parse()
            .unwrap();
            assert!(releases_match_links().validate(&changelog).is_ok());
        }

        #[test]
        fn fails_with_one_link() {
            let changelog = r#"
# Changelog

For tests yo.

[0.1.0]: https://where.the.release.at.com
"#
            .parse()
            .unwrap();
            assert!(matches!(
                releases_match_links().validate(&changelog).unwrap_err(),
                ValidationError::DanglingLinks(_)
            ));
        }

        #[test]
        fn fails_with_one_release() {
            let changelog = r#"
# Changelog

For tests yo.

## [0.1.0] - 2020-01-01

### Added

- Big stuff.
"#
            .parse()
            .unwrap();
            assert!(matches!(
                releases_match_links().validate(&changelog).unwrap_err(),
                ValidationError::MissingLinks(_)
            ));
        }

        #[test]
        fn works_with_link_matching_a_release() {
            let changelog = r#"
# Changelog

For tests yo.

## [0.1.0] - 2020-01-01

### Added

- Big stuff.

[0.1.0]: https://the.release.at.here.com
"#
            .parse()
            .unwrap();
            assert!(releases_match_links().validate(&changelog).is_ok(),);
        }
    }
}
