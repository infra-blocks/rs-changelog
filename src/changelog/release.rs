use crate::changelog::Changes;
use chrono::NaiveDate;
use markdown::mdast::Heading;
use semver::Version;

pub struct Release {
    heading: Heading,
    version: Version,
    date: NaiveDate,
    changes: Changes,
    links: Vec<Link>,
}
