use markdown::mdast as native;

/// GFM: alignment of phrasing content.
///
/// Used to align the contents of table cells within a table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignKind {
    /// Left alignment.
    ///
    /// See the `left` value of the `text-align` CSS property.
    ///
    /// ```markdown
    ///   | | aaa |
    /// > | | :-- |
    ///       ^^^
    /// ```
    Left,
    /// Right alignment.
    ///
    /// See the `right` value of the `text-align` CSS property.
    ///
    /// ```markdown
    ///   | | aaa |
    /// > | | --: |
    ///       ^^^
    /// ```
    Right,
    /// Center alignment.
    ///
    /// See the `center` value of the `text-align` CSS property.
    ///
    /// ```markdown
    ///   | | aaa |
    /// > | | :-: |
    ///       ^^^
    /// ```
    Center,
    /// No alignment.
    ///
    /// Phrasing content is aligned as defined by the host environment.
    ///
    /// ```markdown
    ///   | | aaa |
    /// > | | --- |
    ///       ^^^
    /// ```
    None,
}

impl From<native::AlignKind> for AlignKind {
    fn from(align_kind: native::AlignKind) -> Self {
        match align_kind {
            native::AlignKind::Left => AlignKind::Left,
            native::AlignKind::Right => AlignKind::Right,
            native::AlignKind::Center => AlignKind::Center,
            native::AlignKind::None => AlignKind::None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod from {
        use super::*;

        #[test]
        fn should_work_with_left() {
            assert_eq!(AlignKind::Left, AlignKind::from(native::AlignKind::Left));
        }

        #[test]
        fn should_work_with_right() {
            assert_eq!(AlignKind::Right, AlignKind::from(native::AlignKind::Right));
        }

        #[test]
        fn should_work_with_center() {
            assert_eq!(
                AlignKind::Center,
                AlignKind::from(native::AlignKind::Center)
            );
        }

        #[test]
        fn should_work_with_none() {
            assert_eq!(AlignKind::None, AlignKind::from(native::AlignKind::None));
        }
    }
}
