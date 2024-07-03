use changelog::{Changelog, ChangelogError};

mod utils;

#[test]
fn empty() {
    let err = Changelog::try_from_file(utils::changelog_file("EMPTY_CHANGELOG.md")).unwrap_err();
    assert!(matches!(err, ChangelogError::ParseError(_)));
}
