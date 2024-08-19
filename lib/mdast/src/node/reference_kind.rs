use markdown::mdast as native;

/// Explicitness of a reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceKind {
    /// Only one set of brackets: `[foo]`.
    Shortcut,
    /// Two sets of brackets with one empty: `[foo][]`.
    Collapsed,
    /// Both sets of brackets set: `[text][label]`.
    Full,
}

impl From<native::ReferenceKind> for ReferenceKind {
    fn from(kind: native::ReferenceKind) -> Self {
        match kind {
            native::ReferenceKind::Shortcut => Self::Shortcut,
            native::ReferenceKind::Collapsed => Self::Collapsed,
            native::ReferenceKind::Full => Self::Full,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_native_shortcut() {
        assert_eq!(
            ReferenceKind::Shortcut,
            ReferenceKind::from(native::ReferenceKind::Shortcut)
        );
    }

    #[test]
    fn test_from_native_collapsed() {
        assert_eq!(
            ReferenceKind::Collapsed,
            ReferenceKind::from(native::ReferenceKind::Collapsed)
        );
    }

    #[test]
    fn test_from_native_full() {
        assert_eq!(
            ReferenceKind::Full,
            ReferenceKind::from(native::ReferenceKind::Full)
        );
    }
}
