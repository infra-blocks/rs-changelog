mod utils;

use changelog::{Changelog, ChangelogError, ValidationError};

#[test]
fn dangling_link() {
    let err =
        Changelog::try_from_file(utils::changelog_file("DANGLING_LINK_CHANGELOG.md")).unwrap_err();
    assert!(matches!(
        err,
        ChangelogError::ValidationError(ValidationError::DanglingLinks(_))
    ));
}

#[test]
fn missing_link() {
    let err =
        Changelog::try_from_file(utils::changelog_file("MISSING_LINK_CHANGELOG.md")).unwrap_err();
    assert!(matches!(
        err,
        ChangelogError::ValidationError(ValidationError::MissingLinks(_))
    ));
}

#[test]
fn unordered_links() {
    let err = Changelog::try_from_file(utils::changelog_file("UNORDERED_LINKS_CHANGELOG.md"))
        .unwrap_err();
    assert!(matches!(
        err,
        ChangelogError::ValidationError(ValidationError::UnorderedLinks(_, _))
    ));
}

#[test]
fn unordered_releases() {
    let err = Changelog::try_from_file(utils::changelog_file("UNORDERED_RELEASES_CHANGELOG.md"))
        .unwrap_err();
    assert!(matches!(
        err,
        ChangelogError::ValidationError(ValidationError::UnorderedReleases(_, _))
    ));
}
