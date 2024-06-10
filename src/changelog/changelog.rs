use eyre::WrapErr;
use std::fmt::Display;
use std::path::Path;

#[derive(Debug)]
pub struct Changelog {}

#[derive(Debug)]
pub struct ParseOptions {}

impl Default for ParseOptions {
    fn default() -> Self {
        ParseOptions {}
    }
}

// TODO: local error type.
impl Changelog {
    pub fn from_markdown(markdown: &str, options: &ParseOptions) -> Result<Changelog, eyre::Error> {
        let markdown_tree = markdown::to_mdast(
            markdown,
            &markdown::ParseOptions {
                constructs: markdown::Constructs::gfm(),
                ..markdown::ParseOptions::default()
            },
        )
        .map_err(|err| eyre::eyre!("error parsing markdown: {err}"))?;
        crate::changelog::markdown::Changelog::try_from(&markdown_tree)
            .wrap_err("error parsing changelog")?;
        Ok(Changelog {})
    }

    pub fn from_markdown_file<T: AsRef<Path> + Display>(
        file: &T,
        options: &ParseOptions,
    ) -> eyre::Result<Changelog, eyre::Error> {
        let content = std::fs::read_to_string(file)
            .wrap_err(format!("error reading changelog file '{}'", file))?;

        Changelog::from_markdown(&content, options)
    }
}
