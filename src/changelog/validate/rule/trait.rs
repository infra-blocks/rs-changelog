use crate::ValidationError;
use clast;

pub trait Rule {
    fn validate(&self, changelog: &clast::Changelog) -> Result<(), ValidationError>;
}
