use mdast::Mdast;

#[test]
fn minimal() {
    Mdast::try_from_file(test_utils::changelog_file("MINIMAL_CHANGELOG.md")).unwrap();
}
