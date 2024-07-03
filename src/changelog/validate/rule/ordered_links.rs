use crate::changelog::validate::rule::Rule;
use crate::ValidationError;
use clast;

struct OrderedLinks;

impl Rule for OrderedLinks {
    fn validate(&self, changelog: &clast::Changelog) -> Result<(), ValidationError> {
        let links = &changelog.links;
        if links.is_empty() || links.len() == 1 {
            return Ok(());
        }

        for i in 1..links.len() {
            let previous_link = &links[i - 1];
            let previous_version = &previous_link.version;
            let current_link = &links[i];
            let current_version = &current_link.version;

            if previous_version <= current_version {
                return Err(ValidationError::UnorderedLinks(
                    previous_link.clone(),
                    current_link.clone(),
                ));
            }
        }
        Ok(())
    }
}

pub fn ordered_links() -> impl Rule {
    OrderedLinks
}

#[cfg(test)]
mod test {
    use super::*;

    mod validate {
        use super::*;

        #[test]
        fn works_without_links() {
            let changelog = r#"
# Changelog

Some bullsheetz.
"#
            .parse()
            .unwrap();
            assert!(ordered_links().validate(&changelog).is_ok());
        }

        #[test]
        fn works_a_single_link() {
            let changelog = r#"
# Changelog

Some bullsheetz.

[1.0.0]: https://stfu.com
"#
            .parse()
            .unwrap();
            assert!(ordered_links().validate(&changelog).is_ok());
        }

        #[test]
        fn works_with_ordered_links() {
            let changelog = r#"
# Changelog

Some bullsheetz.

[1.0.0]: https://stfu.com
[0.2.0]: https://stfu.com
[0.1.0]: https://stfu.com
[0.0.1]: https://stfu.com
"#
            .parse()
            .unwrap();
            assert!(ordered_links().validate(&changelog).is_ok());
        }

        #[test]
        fn fails_with_unordered_links() {
            let changelog = r#"
# Changelog

Some bullsheetz.

[1.0.0]: https://stfu.com
[0.1.0]: https://stfu.com
[0.2.0]: https://stfu.com
[0.0.1]: https://stfu.com
"#
            .parse()
            .unwrap();
            assert!(matches!(
                ordered_links().validate(&changelog).unwrap_err(),
                ValidationError::UnorderedLinks(_, _)
            ));
        }
    }
}
