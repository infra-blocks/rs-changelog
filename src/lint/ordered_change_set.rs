use std::cmp::Ordering;

use crate::ChangeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderedChangeSet<'a>(pub &'a ChangeSet);

impl<'a> Ord for OrderedChangeSet<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0.is_same_kind(other.0) {
            return Ordering::Equal;
        }
        if self.0.is_added() {
            return Ordering::Less;
        }
        if other.0.is_added() {
            return Ordering::Greater;
        }
        if self.0.is_changed() {
            return Ordering::Less;
        }
        if other.0.is_changed() {
            return Ordering::Greater;
        }
        if self.0.is_deprecated() {
            return Ordering::Less;
        }
        if other.0.is_deprecated() {
            return Ordering::Greater;
        }
        if self.0.is_fixed() {
            return Ordering::Less;
        }
        if other.0.is_fixed() {
            return Ordering::Greater;
        }
        if self.0.is_removed() {
            return Ordering::Less;
        }

        Ordering::Greater
    }
}

impl<'a> PartialOrd for OrderedChangeSet<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> From<&'a ChangeSet> for OrderedChangeSet<'a> {
    fn from(value: &'a ChangeSet) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use crate::{Added, Changed, Deprecated, Fixed, Removed, Security};

    use super::*;

    macro_rules! equals {
        ($change_set:expr) => {
            let left_cs = $change_set;
            let left = OrderedChangeSet(&left_cs);
            let right_cs = $change_set;
            let right = OrderedChangeSet(&right_cs);
            assert!(left == right);
            assert!(right == left);
        };
    }

    macro_rules! unequals {
        ($lower:expr, $greater:expr) => {
            let left_cs = $lower;
            let left = OrderedChangeSet(&left_cs);
            let right_cs = $greater;
            let right = OrderedChangeSet(&right_cs);
            assert!(left < right);
            assert!(right > left);
        };
    }

    #[test]
    fn added_with_added() {
        equals!(Added::default().into());
    }

    #[test]
    fn added_with_changed() {
        unequals!(Added::default().into(), Changed::default().into());
    }

    #[test]
    fn changed_with_changed() {
        equals!(Changed::default().into());
    }

    #[test]
    fn changed_with_deprecated() {
        unequals!(Changed::default().into(), Deprecated::default().into());
    }

    #[test]
    fn deprecated_with_deprecated() {
        equals!(Deprecated::default().into());
    }

    #[test]
    fn deprecated_with_fixed() {
        unequals!(Deprecated::default().into(), Fixed::default().into());
    }

    #[test]
    fn fixed_with_fixed() {
        equals!(Fixed::default().into());
    }

    #[test]
    fn fixed_with_removed() {
        unequals!(Fixed::default().into(), Removed::default().into());
    }

    #[test]
    fn removed_with_removed() {
        equals!(Removed::default().into());
    }

    #[test]
    fn removed_with_security() {
        unequals!(Removed::default().into(), Security::default().into());
    }

    #[test]
    fn security_with_security() {
        equals!(Security::default().into());
    }
}
