use keep_a_changelog::Changelog;

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

        let changelog = Changelog::parse_from_file(&self.file, None)
            .map_err(|err| err.wrap_err(format!("Error linting changelog file '{}'", self.file)))?;

        self.check_versions_ordered(&changelog)?;
        Ok(())
    }

    fn check_versions_ordered(&self, changelog: &Changelog) -> eyre::Result<()> {
        // TODO: the releases are already returned ordered in descending order.
        let releases = changelog.releases();

        if releases.is_empty() {
            return Ok(());
        }

        let mut previous_release = &releases[0];
        for i in 1..releases.len() {
            let current_release = &releases[i];
            let previous_version = previous_release.version().as_ref().unwrap();
            let current_version = current_release.version().as_ref().unwrap();

            if current_version <= previous_version {
                return Err(eyre::eyre!(
                    "Changelog versions are not ordered! Found {} after {}",
                    current_version,
                    previous_version
                ));
            }
            previous_release = current_release;
        }
        Ok(())
    }

    fn check_version_bumps(&self, changelog: &Changelog) -> eyre::Result<()> {
        todo!()
        // // TODO: are they ordered? That should be the first check.
        // let releases = changelog.releases();
        //
        // if releases.is_empty() {
        //     return Ok(());
        // }
        //
        // let mut previous_release = &releases[0];
        // for i in 1..releases.len() {
        //     let current_release = &releases[i];
        //     let previous_version = previous_release.version();
        //     let current_version = current_release.version();
        //
        //     // TODO: semver diff: is it a difference of one major, one minor, etc...
        //     if previous_version >= current_version {
        //         return Err(eyre::eyre!(
        //             "Release {} has a version lower or equal to the previous release {}",
        //             current_version,
        //             previous_version
        //         ));
        //     }
        //     let previous_release = &releases[i - 1];
        // }
        // Ok(())
    }
}
