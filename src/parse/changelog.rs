use crate::parse::{
    releases::{Releases, Unreleased},
    title::Title,
};

// TODO: implement ToOwned
#[derive(Debug, Clone, PartialEq)]
pub struct Changelog<'source> {
    pub source: &'source str,
    pub title: Title,
    /// The unreleased section of a document is optional, as it would basically become empty
    /// after each release. So, whether the user decides to have one or not, is up to them.
    pub unreleased: Option<Unreleased>,
    pub releases: Releases,
}
