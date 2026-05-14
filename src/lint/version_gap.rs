use semver::Version;

/// Tests whether two versions different by exactly one bump.
///
/// The bump can either be a major, minor, or patch version.
/// The first version is treated as the *preceding* version of the second.
/// For example, if the versions are 0.1.1 and 2.0.0, then the function
/// returns true.
pub fn versions_differ_by_one(first: &Version, second: &Version) -> bool {
    // For a major version bump, the major part should be incremented and all other fields reset to 0.
    Version::new(first.major + 1, 0, 0) == *second
        // For a minor version bump, the major number should remain the same, the minor number incremented, and the patch number be reset.
        || Version::new(first.major, first.minor + 1, 0) == *second
        // For a patch version bump, then all other fields should remain the same and the patch incremented.
        || Version::new(first.major, first.minor, first.patch + 1) == *second
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_false_for_major_bump_without_reset() {
        assert!(!versions_differ_by_one(
            &Version::new(3, 4, 5),
            &Version::new(4, 4, 5)
        ));
    }

    #[test]
    fn should_return_false_for_minor_bump_without_reset() {
        assert!(!versions_differ_by_one(
            &Version::new(3, 4, 5),
            &Version::new(3, 5, 5)
        ));
    }

    #[test]
    fn should_return_false_if_all_fields_vary() {
        assert!(!versions_differ_by_one(
            &Version::new(3, 4, 5),
            &Version::new(4, 5, 6)
        ));
    }

    #[test]
    fn should_return_true_for_major_bump() {
        assert!(versions_differ_by_one(
            &Version::new(3, 4, 5),
            &Version::new(4, 0, 0)
        ));
    }

    #[test]
    fn should_return_true_for_minor_bump() {
        assert!(versions_differ_by_one(
            &Version::new(3, 4, 5),
            &Version::new(3, 5, 0)
        ));
    }

    #[test]
    fn should_return_true_for_patch_bump() {
        assert!(versions_differ_by_one(
            &Version::new(3, 4, 5),
            &Version::new(3, 4, 6)
        ));
    }
}
