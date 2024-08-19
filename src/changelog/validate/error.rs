use clast;
use semver::Version;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum ValidationError {
    UnorderedReleases(clast::Release, clast::Release),
    UnorderedLinks(clast::ReleaseLink, clast::ReleaseLink),
    MissingLinks(Vec<String>),
    DanglingLinks(Vec<String>),
    InvalidIncrementBetweenReleases(clast::Release, clast::Release),
    WrongFirstRelease(Version, clast::Release),
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::UnorderedReleases(previous, current) => {
                write!(
                    f,
                    "found unordered releases: {}{} <= {}{}",
                    previous.position.to_string(),
                    previous.version,
                    current.position.to_string(),
                    current.version,
                )
            }
            ValidationError::UnorderedLinks(previous, current) => {
                write!(
                    f,
                    "found unordered links: {}{} <= {}{}",
                    previous.position.to_string(),
                    previous.version,
                    current.position.to_string(),
                    current.version,
                )
            }
            ValidationError::MissingLinks(missing_links) => {
                write!(
                    f,
                    "found releases without links: {}",
                    missing_links.join(", ")
                )
            }
            ValidationError::DanglingLinks(dangling_links) => {
                write!(
                    f,
                    "found links without releases: {}",
                    dangling_links.join(", ")
                )
            }
            ValidationError::InvalidIncrementBetweenReleases(previous, current) => {
                write!(
                    f,
                    "found invalid increment between releases: {}{} -> {}{}, expected at most a single major, minor or patch increment",
                    previous.position.to_string(),
                    previous.version,
                    current.position.to_string(),
                    current.version,
                )
            }
            ValidationError::WrongFirstRelease(expected, effective) => {
                write!(
                    f,
                    "found wrong first release version: {}{}, expected to be {}",
                    effective.position.to_string(),
                    effective.version,
                    expected,
                )
            }
        }
    }
}

impl Error for ValidationError {}
