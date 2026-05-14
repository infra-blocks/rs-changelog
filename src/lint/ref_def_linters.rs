use std::{error::Error, fmt::Display};

use url::{Host, Url};

use crate::ReferenceDefinition;

pub enum RefDefLinter {
    GitHub(GitHubRefDefLinter),
}

impl RefDefLinter {
    pub fn try_new(ref_def: &ReferenceDefinition) -> Option<Self> {
        // TODO: this should be a specific error.
        let Ok(url) = Url::parse(ref_def.dest()) else {
            return None;
        };

        if let Some(linter) = GitHubRefDefLinter::try_new(&url) {
            return Some(Self::GitHub(linter));
        }
        None
    }

    pub fn lint_release_definition(
        &self,
        ref_def: &ReferenceDefinition,
    ) -> Result<(), RefDefLintError> {
        match self {
            RefDefLinter::GitHub(linter) => linter.lint_release_definition(ref_def),
        }
    }

    // [0.2.0]: https://github.com/owner/repo/compare/v0.1.0...v0.2.0
    pub fn lint_diff_definition(
        &self,
        // This is the earlier version
        previous: &ReferenceDefinition,
        // This is the other one, bro.
        current: &ReferenceDefinition,
    ) -> Result<(), RefDefLintError> {
        match self {
            RefDefLinter::GitHub(linter) => linter.lint_diff_definition(previous, current),
        }
    }
}

// TODO: use ranges here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefDefLintError {
    InvalidUrl(String),
}

impl From<url::ParseError> for RefDefLintError {
    fn from(value: url::ParseError) -> Self {
        Self::InvalidUrl(value.to_string())
    }
}

impl Display for RefDefLintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefDefLintError::InvalidUrl(msg) => write!(f, "invalid url: {}", msg),
        }
    }
}

impl Error for RefDefLintError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubRefDefLinter {
    base_url: String,
}

impl GitHubRefDefLinter {
    fn try_new(url: &Url) -> Option<Self> {
        match url.host() {
            Some(Host::Domain("github.com")) => {
                let path = url.path();
                let path_tokens: Vec<_> = path.split("/").collect();
                if path_tokens.len() < 3 {
                    return None;
                }
                let owner = path_tokens[1];
                let repository = path_tokens[2];
                let mut base_url = to_base_url(url);
                let new_path = format!("/{}/{}", owner, repository);

                base_url.set_path(new_path.as_str());
                Some(Self {
                    base_url: base_url.into(),
                })
            }
            _ => None,
        }
    }

    fn lint_release_definition(
        &self,
        ref_def: &ReferenceDefinition<'_>,
    ) -> Result<(), RefDefLintError> {
        let label = ref_def.label();
        let url = Url::parse(ref_def.dest())?;
        let expected = if label.eq_ignore_ascii_case("unreleased") {
            format!("{}/commits/HEAD", self.base_url)
        } else {
            format!("{}/releases/tag/v{}", self.base_url, label)
        };
        if url.as_str() != expected {
            return Err(RefDefLintError::InvalidUrl(format!(
                "{} not matching expected {}",
                url.as_str(),
                expected
            )));
        }
        Ok(())
    }

    fn lint_diff_definition(
        &self,
        previous: &ReferenceDefinition<'_>,
        current: &ReferenceDefinition<'_>,
    ) -> Result<(), RefDefLintError> {
        let url = Url::parse(current.dest())?;
        let expected = if current.label().eq_ignore_ascii_case("unreleased") {
            format!("{}/compare/v{}...HEAD", self.base_url, previous.label(),)
        } else {
            format!(
                "{}/compare/v{}...v{}",
                self.base_url,
                previous.label(),
                current.label()
            )
        };
        if url.as_str() != expected {
            return Err(RefDefLintError::InvalidUrl(format!(
                "{} not matching expected {}",
                url.as_str(),
                expected
            )));
        }
        Ok(())
    }
}

fn to_base_url(url: &Url) -> Url {
    let mut url = url.clone();
    url.set_path("");
    url.set_query(None);
    url.set_fragment(None);
    url
}

#[cfg(test)]
mod test {
    use super::*;

    mod github_ref_def_linter {
        use super::*;

        mod lint_relese_definition {
            use std::ops::Range;

            use changelog_ast::CowStr;

            use super::*;

            // A GitHub release reference definition has this form:
            // [<label>]: https://github.com/owner/repo/releases/tag/v<label>

            macro_rules! failure {
                ($init_url:expr, $ref_def:expr) => {
                    let dest_url = Url::parse($init_url).unwrap();
                    let linter = GitHubRefDefLinter::try_new(&dest_url).unwrap();
                    let ref_def = $ref_def;

                    assert!(matches!(
                        linter.lint_release_definition(&ref_def),
                        Err(RefDefLintError::InvalidUrl(_))
                    ));
                };
            }

            #[test]
            fn should_error_with_invalid_url() {
                failure!(
                    "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0",
                    ReferenceDefinition::new("0.1.0".into(), "toto".into(), Range::default())
                );
            }

            #[test]
            fn should_error_with_gitlab_url() {
                failure!(
                    "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0",
                    ReferenceDefinition::new(
                        "0.1.0".into(),
                        "https://gitlab.com/infra-blocks/rs-changelog/releases/tag/v0.1.0".into(),
                        Range::default()
                    )
                );
            }

            #[test]
            fn should_error_with_invalid_path() {
                failure!(
                    "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0",
                    ReferenceDefinition::new(
                        "0.1.0".into(),
                        "https://github.com/el-pendeloco/rs-changelog/releases/tag/v0.1.0".into(),
                        Range::default()
                    )
                );
            }

            #[test]
            fn should_error_with_trailing_fragment() {
                failure!(
                    "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0",
                    ReferenceDefinition::new(
                        "0.1.0".into(),
                        "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0#jesus"
                            .into(),
                        Range::default()
                    )
                );
            }

            #[test]
            fn should_error_with_label_version_mismatch() {
                failure!(
                    "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0",
                    ReferenceDefinition::new(
                        "0.2.0".into(),
                        "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0".into(),
                        Range::default()
                    )
                );
            }

            #[test]
            fn should_work_with_valid_versioned_ref_def() {
                let label = "0.1.0";
                let dest = "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0";
                let ref_def =
                    ReferenceDefinition::new(label.into(), CowStr::from(dest), Range::default());
                let dest_url = Url::parse(dest).unwrap();
                let linter = GitHubRefDefLinter::try_new(&dest_url).unwrap();
                assert_eq!(linter.lint_release_definition(&ref_def), Ok(()));
            }

            #[test]
            fn should_work_with_valid_unreleased_ref_def() {
                let label = "unreleased";
                let dest = "https://github.com/infra-blocks/rs-changelog/commits/HEAD";
                let ref_def =
                    ReferenceDefinition::new(label.into(), CowStr::from(dest), Range::default());
                let dest_url = Url::parse(dest).unwrap();
                let linter = GitHubRefDefLinter::try_new(&dest_url).unwrap();
                assert_eq!(linter.lint_release_definition(&ref_def), Ok(()));
            }
        }

        mod lint_diff_definition {
            use std::ops::Range;

            use super::*;

            // A GitHub diff reference definition has this form:
            // [<current_label>]: https://github.com/owner/repo/compare/v<previous_label>...v<current_label>
            // [<previous_label>]: ...

            macro_rules! failure {
                ($first:expr, $second:expr) => {
                    let init_ref_def = $first;
                    let dest_url = Url::parse(init_ref_def.dest()).unwrap();
                    let linter = GitHubRefDefLinter::try_new(&dest_url).unwrap();
                    let ref_def = $second;
                    assert!(matches!(
                        linter.lint_diff_definition(&init_ref_def, &ref_def),
                        Err(RefDefLintError::InvalidUrl(_))
                    ));
                };
            }

            #[test]
            fn should_error_with_invalid_url() {
                failure!(
                    ReferenceDefinition::new(
                        "0.1.0".into(),
                        "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0".into(),
                        Range::default()
                    ),
                    ReferenceDefinition::new("0.2.0".into(), "toto".into(), Range::default(),)
                );
            }

            #[test]
            fn should_error_with_gitlab_url() {
                failure!(
                    ReferenceDefinition::new(
                        "0.1.0".into(),
                        "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0".into(),
                        Range::default()
                    ),
                    ReferenceDefinition::new(
                        "0.2.0".into(),
                        "https://gitlab.com/infra-blocks/rs-changelog/compare/v0.1.0...v0.2.0"
                            .into(),
                        Range::default(),
                    )
                );
            }

            #[test]
            fn should_error_with_invalid_path() {
                failure!(
                    ReferenceDefinition::new(
                        "0.1.0".into(),
                        "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0".into(),
                        Range::default()
                    ),
                    ReferenceDefinition::new(
                        "0.2.0".into(),
                        "https://gitlab.com/infra-blocks/rs-changelog/diff/v0.1.0...v0.2.0".into(),
                        Range::default(),
                    )
                );
            }

            #[test]
            fn should_error_with_trailing_fragment() {
                failure!(
                    ReferenceDefinition::new(
                        "0.1.0".into(),
                        "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0".into(),
                        Range::default()
                    ),
                    ReferenceDefinition::new(
                        "0.2.0".into(),
                        "https://gitlab.com/infra-blocks/rs-changelog/diff/v0.1.0...v0.2.0".into(),
                        Range::default(),
                    )
                );
            }

            #[test]
            fn should_work_with_valid_ref_def() {
                let init_ref_def = ReferenceDefinition::new(
                    "0.1.0".into(),
                    "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0".into(),
                    Range::default(),
                );
                let dest_url = Url::parse(init_ref_def.dest()).unwrap();
                let linter = GitHubRefDefLinter::try_new(&dest_url).unwrap();
                let ref_def = ReferenceDefinition::new(
                    "0.2.0".into(),
                    "https://github.com/infra-blocks/rs-changelog/compare/v0.1.0...v0.2.0".into(),
                    Range::default(),
                );
                assert_eq!(linter.lint_diff_definition(&init_ref_def, &ref_def), Ok(()));
            }

            #[test]
            fn should_work_with_valid_unreleased_ref_def() {
                let init_ref_def = ReferenceDefinition::new(
                    "0.1.0".into(),
                    "https://github.com/infra-blocks/rs-changelog/releases/tag/v0.1.0".into(),
                    Range::default(),
                );
                let dest_url = Url::parse(init_ref_def.dest()).unwrap();
                let linter = GitHubRefDefLinter::try_new(&dest_url).unwrap();
                let ref_def = ReferenceDefinition::new(
                    "Unreleased".into(),
                    "https://github.com/infra-blocks/rs-changelog/compare/v0.1.0...HEAD".into(),
                    Range::default(),
                );
                assert_eq!(linter.lint_diff_definition(&init_ref_def, &ref_def), Ok(()));
            }
        }
    }

    mod to_base_url {
        use super::*;

        #[test]
        fn should_keep_a_base_url_the_same_with_trailing_slash() {
            let url = Url::parse("https://www.stfu.com/").unwrap();
            assert_eq!(to_base_url(&url), url);
        }

        #[test]
        fn should_strip_out_path() {
            let url = Url::parse("https://www.stfu.com/please").unwrap();
            assert_eq!(
                to_base_url(&url),
                Url::parse("https://www.stfu.com/").unwrap()
            );
        }

        #[test]
        fn should_strip_out_fragment() {
            let url = Url::parse("https://www.stfu.com/#when-im-talking").unwrap();
            assert_eq!(
                to_base_url(&url),
                Url::parse("https://www.stfu.com/").unwrap()
            );
        }

        #[test]
        fn should_strip_out_query_params() {
            let url = Url::parse("https://www.stfu.com/?target=ai-boosters").unwrap();
            assert_eq!(
                to_base_url(&url),
                Url::parse("https://www.stfu.com/").unwrap()
            );
        }
    }
}
