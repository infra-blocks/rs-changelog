use crate::changelog::markdown;
use crate::changelog::parse::{Description, Link, Release, Title};
use eyre::Error;

#[derive(Debug)]
pub struct Changelog {
    title: Title,
    description: Description,
    releases: Vec<Release>,
    links: Vec<Link>,
}

impl TryFrom<&markdown::Changelog> for Changelog {
    type Error = Error;

    fn try_from(markdown_changelog: &markdown::Changelog) -> Result<Self, Self::Error> {
        let title = Title::try_from(&markdown_changelog.title)?;
        let description = Description::try_from(&markdown_changelog.description)?;
        let releases = markdown_changelog
            .releases
            .iter()
            .map(Release::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let links = markdown_changelog
            .links
            .iter()
            .map(Link::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Changelog {
            title,
            description,
            releases,
            links,
        })
    }
}
