use crate::changelog;
use crate::changelog::parse::text::children_to_string;
use crate::changelog::parse::Position;
use eyre::Error;

#[derive(Debug)]
pub struct Changes {
    added: Option<ChangeSet>,
    changed: Option<ChangeSet>,
    deprecated: Option<ChangeSet>,
    removed: Option<ChangeSet>,
    fixed: Option<ChangeSet>,
    security: Option<ChangeSet>,
}

impl Default for Changes {
    fn default() -> Self {
        Changes {
            added: None,
            changed: None,
            deprecated: None,
            removed: None,
            fixed: None,
            security: None,
        }
    }
}

fn some_changeset_if_is_none(
    change_set: Option<ChangeSet>,
    new_change_set: ChangeSet,
) -> Result<Option<ChangeSet>, Error> {
    match change_set {
        Some(added) => Err(eyre::eyre!(
            "a {:?} changeset is already defined, but found duplicate",
            added
        )),
        None => Ok(Some(new_change_set)),
    }
}

impl TryFrom<&Vec<changelog::markdown::Change>> for Changes {
    type Error = Error;

    fn try_from(markdown_changes: &Vec<changelog::markdown::Change>) -> Result<Self, Self::Error> {
        let mut changes = Changes::default();

        for markdown_change in markdown_changes {
            let change_set = ChangeSet::try_from(markdown_change)?;
            match change_set.kind {
                ChangeKind::Added => {
                    changes.added = some_changeset_if_is_none(changes.added, change_set)?
                }
                ChangeKind::Changed => {
                    changes.changed = some_changeset_if_is_none(changes.changed, change_set)?
                }
                ChangeKind::Deprecated => {
                    changes.deprecated = some_changeset_if_is_none(changes.deprecated, change_set)?
                }
                ChangeKind::Removed => {
                    changes.removed = some_changeset_if_is_none(changes.removed, change_set)?
                }
                ChangeKind::Fixed => {
                    changes.fixed = some_changeset_if_is_none(changes.fixed, change_set)?
                }
                ChangeKind::Security => {
                    changes.security = some_changeset_if_is_none(changes.security, change_set)?
                }
            }
        }

        Ok(changes)
    }
}

#[derive(Debug)]
pub enum ChangeKind {
    Added,
    Changed,
    Deprecated,
    Removed,
    Fixed,
    Security,
}

impl TryFrom<&str> for ChangeKind {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "Added" => Ok(ChangeKind::Added),
            "Changed" => Ok(ChangeKind::Changed),
            "Deprecated" => Ok(ChangeKind::Deprecated),
            "Removed" => Ok(ChangeKind::Removed),
            "Fixed" => Ok(ChangeKind::Fixed),
            "Security" => Ok(ChangeKind::Security),
            _ => Err(eyre::eyre!("unknown change kind: {}", value)),
        }
    }
}

#[derive(Debug)]
pub struct ChangeSet {
    kind: ChangeKind,
    position: Option<Position>,
    items: Vec<Change>,
}

impl TryFrom<&changelog::markdown::Change> for ChangeSet {
    type Error = Error;

    fn try_from(value: &changelog::markdown::Change) -> Result<Self, Self::Error> {
        let markdown_node = &value.heading;
        let position = markdown_node
            .position
            .clone()
            .map(|position| position.into());

        let text = children_to_string(&markdown_node.children);
        let kind = ChangeKind::try_from(text.as_str())?;
        let items = value
            .items
            .iter()
            .map(Change::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ChangeSet {
            kind,
            position,
            items,
        })
    }
}

#[derive(Debug)]
pub struct Change {
    position: Option<Position>,
    text: String,
}

impl TryFrom<&changelog::markdown::Item> for Change {
    type Error = Error;

    fn try_from(value: &changelog::markdown::Item) -> Result<Self, Self::Error> {
        let markdown_node = &value.list_item;
        let position = markdown_node
            .position
            .clone()
            .map(|position| position.into());

        let text = children_to_string(&markdown_node.children);

        Ok(Change { position, text })
    }
}
