use semver::Version;

use crate::ParseError;

pub fn parse_version(text: &str) -> Result<Version, ParseError> {
    Version::parse(&text.replace(&['[', ']'], ""))
        .map_err(|err| ParseError::invalid_text(text, "unable to parse version").with_source(err))
}
