use clast::Changelog;
use location::{Location, Position, Span};

#[test]
fn minimal() {
    let changelog =
        Changelog::try_from_file(test_utils::changelog_file("MINIMAL_CHANGELOG.md")).unwrap();
    let nodes = changelog.nodes;
    assert_eq!(nodes.len(), 6);
    let title = nodes[0].unwrap_heading_ref();
    assert_eq!(title.depth, 1);
    assert_eq!(title.text, "Changelog");
    assert_eq!(
        title.location,
        Location::span(Span::new(Position::new(1, 1), Position::new(1, 12)))
    );
    let description = nodes[1].unwrap_paragraph_ref();
    assert_eq!(description.text, "This is the minimum supported changelog.");
    let changeset_heading = nodes[2].unwrap_heading_ref();
    assert_eq!(changeset_heading.depth, 2);
    // The brackets are stripped off by the markdown library. To keep'em, we would have to write a thin wrapper
    // around the markdown library. Could be next.
    assert_eq!(changeset_heading.text, "0.1.0 - 2020-01-01");
    let change_kind_heading = nodes[3].unwrap_heading_ref();
    assert_eq!(change_kind_heading.depth, 3);
    assert_eq!(change_kind_heading.text, "Added");
    let changes = nodes[4].unwrap_list_ref();
    assert_eq!(changes.items.len(), 1);
    assert_eq!(changes.items[0].text, "At least one release and one item");
    let definition = nodes[5].unwrap_definition_ref();
    assert_eq!(definition.label, "0.1.0");
    assert_eq!(
        definition.destination,
        "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0"
    );
}

#[test]
fn demo() {
    // Just testing it doesn't throw.
    Changelog::try_from_file(test_utils::changelog_file("DEMO_CHANGELOG.md")).unwrap();
}
