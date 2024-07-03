use clast;
pub use error::*;
use semver::Version;

mod error;
mod rule;

use rule::Rule;

// TODO: a documented number for each rule.
#[derive(Debug)]
pub struct ValidationOptions {
    ordered_releases: bool,
    ordered_links: bool,
    /// Validate that there is a link for each release and a release for each link.
    versions_set_equals_links_set: bool,
    // TODO: consistent link urls
    // TODO: first URL is tag and others are compare for github?
    /// This rule verifies that all the releases, when sorted, have a single increment between them,
    /// whether it be a patch, minor or major version bump. "0.1.0" to "0.1.1" is valid, but
    /// "0.1.0" to "0.1.2" is not.
    single_increment_releases: bool,
    releases_start_at: Option<Version>,
}

impl Default for ValidationOptions {
    fn default() -> Self {
        ValidationOptions {
            ordered_releases: true,
            ordered_links: true,
            versions_set_equals_links_set: true,
            single_increment_releases: true,
            releases_start_at: Some(Version::new(0, 1, 0)),
        }
    }
}

impl ValidationOptions {
    pub fn pedantic() -> Self {
        Self::default()
    }
}

// TODO: accumulate errors.
// TODO: run in parallel?
pub fn validate(
    changelog: &clast::Changelog,
    options: ValidationOptions,
) -> Result<(), ValidationError> {
    if options.ordered_releases {
        rule::ordered_releases().validate(changelog)?;
    }
    if options.ordered_links {
        rule::ordered_links().validate(changelog)?;
    }
    if options.versions_set_equals_links_set {
        rule::releases_match_links().validate(changelog)?;
    }
    if options.single_increment_releases {
        rule::single_increment_releases().validate(changelog)?;
    }
    if let Some(start_at) = options.releases_start_at {
        rule::releases_start_at(start_at).validate(changelog)?;
    }
    Ok(())
}
