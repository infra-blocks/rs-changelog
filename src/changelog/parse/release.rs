use chrono::NaiveDate;
use eyre::Error;
use semver::Version;

use crate::changelog;
use crate::changelog::error::RuntimeError;
use crate::changelog::parse::error::{invalid_format_error, ParsedNodeKind};
use crate::changelog::parse::text::children_to_string;
use crate::changelog::parse::{Changes, Position};

#[derive(Debug)]
pub struct Release {
    version: Version,
    date: NaiveDate,
    position: Option<Position>,
    changes: Changes,
}

type ParsedText = (Version, NaiveDate);

// TODO: positions.
fn parse_version(text: &str) -> Result<Version, impl std::error::Error> {
    Version::parse(&text.replace(&['[', ']'], "")).map_err(|err| {
        RuntimeError::from_source(format!("unable to parse version \"{text}\""), err)
    })
}

fn parse_date(text: &str) -> Result<NaiveDate, impl std::error::Error> {
    NaiveDate::parse_from_str(text, "%Y-%m-%d")
        .map_err(|err| RuntimeError::from_source(format!("unable to parse date \"{text}\""), err))
}

fn parse_text(position: Option<Position>, text: &str) -> Result<ParsedText, Error> {
    let tokens = text.split_once("-");

    match tokens {
        Some((left, right)) => {
            let left = left.trim();
            let right = right.trim();
            let version = parse_version(left).map_err(|err| {
                invalid_format_error(position, ParsedNodeKind::Release, text.into(), err)
            })?;
            let date = parse_date(right).map_err(|err| {
                invalid_format_error(position, ParsedNodeKind::Release, text.into(), err)
            })?;
            Ok((version, date))
        }
        None => Err(eyre::eyre!("malformed release text: {}", text)),
    }
}

impl TryFrom<&changelog::markdown::Release> for Release {
    type Error = Error;

    fn try_from(markdown_release: &changelog::markdown::Release) -> Result<Self, Self::Error> {
        let markdown_node = &markdown_release.heading;
        let position = markdown_node
            .position
            .clone()
            .map(|position| position.into());

        let text = children_to_string(&markdown_node.children);
        let (version, date) = parse_text(position, &text)?;
        let changes = Changes::try_from(&markdown_release.changes)?;
        Ok(Release {
            position,
            version,
            date,
            changes,
        })
    }
}
