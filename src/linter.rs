use changelog::{Changelog, ParseOptions};

pub struct Linter {
    file: String,
}

impl Linter {
    pub fn new<T: Into<String>>(file: T) -> Self {
        return Linter { file: file.into() };
    }

    // TODO: config file yo.
    pub fn lint(&self) -> eyre::Result<()> {
        // TODO: Log.
        println!("Linting file: {}", self.file);
        let changelog = Changelog::try_from_file_with_options(&self.file, ParseOptions::default())?;
        println!("Parsed changelog fields: {:?}", changelog);
        Ok(())
    }
}
