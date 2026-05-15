use std::path::Path;

use changelog::{ChangeSet, Release, check};
use chrono::NaiveDate;
use semver::Version;

macro_rules! assert_changes {
    ($content:expr, $change_set:expr, $expected:expr) => {
        let effective: Vec<_> = $change_set
            .changes()
            .map(|c| &$content[c.range().clone()])
            .collect();
        assert_eq!(effective, $expected);
    };
}

fn assert_added(content: &str, change_set: &ChangeSet, expected: Vec<&str>) {
    let ChangeSet::Added(added) = change_set else {
        panic!("expected an Added change set, got {:?}", change_set);
    };
    assert_changes!(content, added, expected);
}

fn assert_changed(content: &str, change_set: &ChangeSet, expected: Vec<&str>) {
    let ChangeSet::Changed(changed) = change_set else {
        panic!("expected a Changed change set, got {:?}", change_set);
    };
    assert_changes!(content, changed, expected);
}

fn assert_fixed(content: &str, change_set: &ChangeSet, expected: Vec<&str>) {
    let ChangeSet::Fixed(fixed) = change_set else {
        panic!("expected a Fixed change set, got {:?}", change_set);
    };
    assert_changes!(content, fixed, expected);
}

fn assert_removed(content: &str, change_set: &ChangeSet, expected: Vec<&str>) {
    let ChangeSet::Removed(removed) = change_set else {
        panic!("expected a Removed change set, got {:?}", change_set);
    };
    assert_changes!(content, removed, expected);
}

fn assert_release(content: &str, release: &Release, expectations: &ReleaseExpectations) {}

struct ReleaseExpectations {
    version: Version,
    date: NaiveDate,
    change_sets: ChangeSetsExpectations,
}

struct ChangeSetsExpectations {
    expectations: Vec<ChangeSetExpectations>,
}

enum ChangeSetExpectations {
    Added(Vec<&'static str>),
    Changed(Vec<&'static str>),
    Deprecated(Vec<&'static str>),
    Fixed(Vec<&'static str>),
    Removed(Vec<&'static str>),
    Security(Vec<&'static str>),
}

impl ChangeSetExpectations {
    fn assert(&self, content: &str, effective: &ChangeSet) {
        match self {
            ChangeSetExpectations::Added(items) => todo!(),
            ChangeSetExpectations::Changed(items) => todo!(),
            ChangeSetExpectations::Deprecated(items) => todo!(),
            ChangeSetExpectations::Fixed(items) => todo!(),
            ChangeSetExpectations::Removed(items) => todo!(),
            ChangeSetExpectations::Security(items) => todo!(),
        }
    }
}

#[test]
fn should_parse_keep_a_changelog_demo() {
    let path = Path::new("./tests/assets/keep-a-changelog.md");
    let content = std::fs::read_to_string(&path).unwrap();
    let changelog = check(&content).unwrap();
    let unreleased = changelog.unreleased().as_ref().unwrap();
    let mut change_sets = unreleased.change_sets();
    assert_added(
        &content,
        change_sets.next().unwrap(),
        vec![
            "- v1.1 Brazilian Portuguese translation.\n",
            "- v1.1 German Translation\n",
            "- v1.1 Spanish translation.\n",
            "- v1.1 Italian translation.\n",
            "- v1.1 Polish translation.\n",
            // TODO: get rid of the double new line
            "- v1.1 Ukrainian translation.\n\n",
        ],
    );
    assert_changed(
        &content,
        change_sets.next().unwrap(),
        vec![
            "- Use frontmatter title & description in each language version template\n",
            "- Replace broken OpenGraph image with an appropriately-sized Keep a Changelog 
  image that will render properly (although in English for all languages)\n",
            "- Fix OpenGraph title & description for all languages so the title and 
description when links are shared are language-appropriate\n\n",
        ],
    );
    assert_removed(
        &content,
        change_sets.next().unwrap(),
        vec![
            "- Trademark sign previously shown after the project description in version 
0.3.0\n\n",
        ],
    );
    assert_eq!(change_sets.next(), None);

    let mut releases = changelog.releases().iter();
    let release = releases.next().unwrap();
    assert_eq!(release.version(), &Version::new(1, 1, 1));
    assert_eq!(
        release.date(),
        &NaiveDate::from_ymd_opt(2023, 3, 5).unwrap()
    );
    let mut change_sets = release.change_sets();
    assert_added(
        &content,
        change_sets.next().unwrap(),
        vec![
            "- Arabic translation (#444).\n",
            "- v1.1 French translation.\n",
            "- v1.1 Dutch translation (#371).\n",
            "- v1.1 Russian translation (#410).\n",
            "- v1.1 Japanese translation (#363).\n",
            "- v1.1 Norwegian Bokmål translation (#383).\n",
            "- v1.1 \"Inconsistent Changes\" Turkish translation (#347).\n",
            "- Default to most recent versions available for each languages.\n",
            "- Display count of available translations (26 to date!).\n",
            "- Centralize all links into `/data/links.json` so they can be updated easily.\n\n",
        ],
    );
    assert_fixed(
        &content,
        change_sets.next().unwrap(),
        vec![
            "- Improve French translation (#377).\n",
            "- Improve id-ID translation (#416).\n",
            "- Improve Persian translation (#457).\n",
            "- Improve Russian translation (#408).\n",
            "- Improve Swedish title (#419).\n",
            "- Improve zh-CN translation (#359).\n",
            "- Improve French translation (#357).\n",
            "- Improve zh-TW translation (#360, #355).\n",
            "- Improve Spanish (es-ES) transltion (#362).\n",
            "- Foldout menu in Dutch translation (#371).\n",
            "- Missing periods at the end of each change (#451).\n",
            "- Fix missing logo in 1.1 pages.\n",
            "- Display notice when translation isn't for most recent version.\n",
            "- Various broken links, page versions, and indentations.\n\n",
        ],
    );
    assert_changed(
        &content,
        change_sets.next().unwrap(),
        vec!["- Upgrade dependencies: Ruby 3.2.1, Middleman, etc.\n\n"],
    );
    assert_removed(
        &content,
        change_sets.next().unwrap(),
        vec![
            "- Unused normalize.css file.\n",
            "- Identical links assigned in each translation file.\n",
            "- Duplicate index file for the english version.\n\n",
        ],
    );
}
