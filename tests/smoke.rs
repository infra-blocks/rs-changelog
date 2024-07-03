mod utils;

use changelog::Changelog;

#[test]
fn minimal() {
    let changelog =
        Changelog::try_from_file(utils::changelog_file("MINIMAL_CHANGELOG.md")).unwrap();
    assert_eq!(changelog.title(), "Changelog");
    assert_eq!(
        changelog.description(),
        r#"This is the minimum supported changelog."#
    );
    assert_eq!(changelog.releases().len(), 1);
    assert_eq!(changelog.links().len(), 1);
}

// TODO: support "unreleased"
/// This test should work. It basically showcases that the library can parse the changelog
/// presented at keepachangelog.com at the time of this writing.
#[test]
fn demo() {
    let changelog = Changelog::try_from_file(utils::changelog_file("DEMO_CHANGELOG.md")).unwrap();
    assert_eq!(changelog.title(), "Changelog");
    assert_eq!(
        changelog.description(),
        r#"All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

"#
    );
    assert_eq!(changelog.releases().len(), 15);
    assert_eq!(changelog.links().len(), 15);
}
