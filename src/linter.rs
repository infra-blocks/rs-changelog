use crate::changelog::{Changelog, ParseOptions};

pub struct Linter {
    file: String,
}

impl Linter {
    pub fn new<T: Into<String>>(file: T) -> Self {
        return Linter { file: file.into() };
    }

    pub fn lint(&self) -> eyre::Result<()> {
        // TODO: Log.
        println!("Linting file: {}", self.file);
        let changelog = Changelog::from_markdown_file(&self.file, &ParseOptions::default())?;
        println!("Parsed changelog fields: {:?}", changelog);
        // self.check_versions_ordered(&markdown_changelog)?;
        Ok(())
    }

    // fn check_versions_ordered(&self, changelog: &Changelog) -> eyre::Result<()> {
    //     todo!()
    // }

    // fn check_version_bumps(&self, changelog: &Changelog) -> eyre::Result<()> {
    //     todo!()
    //     // // TODO: are they ordered? That should be the first check.
    //     // let releases = changelog.releases();
    //     //
    //     // if releases.is_empty() {
    //     //     return Ok(());
    //     // }
    //     //
    //     // let mut previous_release = &releases[0];
    //     // for i in 1..releases.len() {
    //     //     let current_release = &releases[i];
    //     //     let previous_version = previous_release.version();
    //     //     let current_version = current_release.version();
    //     //
    //     //     // TODO: semver diff: is it a difference of one major, one minor, etc...
    //     //     if previous_version >= current_version {
    //     //         return Err(eyre::eyre!(
    //     //             "Release {} has a version lower or equal to the previous release {}",
    //     //             current_version,
    //     //             previous_version
    //     //         ));
    //     //     }
    //     //     let previous_release = &releases[i - 1];
    //     // }
    //     // Ok(())
    // }
}
