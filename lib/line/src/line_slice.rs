/// A slice of text that will never overlap over 1 line.
///
/// This type has an invariant and guarantees that no
/// newline character is present within, unless it is the last character.
///
/// It can be further reduced through slicing itself, and the
/// invariant is kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSlice<'a>(&'a str);

impl<'a> LineSlice<'a> {
    /// Constructs a new [LineSlice] from a string slice.
    ///
    /// # Panics
    ///
    /// Panics the string slice contains a non-terminating newline character.
    pub fn new(line: &'a str) -> Self {
        line.try_into()
            .expect(format!("trying to construct invalid line with: {:?}", line).as_str())
    }

    /// Returns the wrapped text.
    pub fn text(&self) -> &'a str {
        self.0
    }

    /// Returns the wrapped text without the terminating newline character, if present.
    pub fn line_text(&self) -> &'a str {
        if self.0.ends_with('\n') {
            let len = self.0.len();
            &self.0[..len - 1]
        } else {
            self.0
        }
    }

    /// Returns the wrapped text, with the lifetime tied to this instance.
    ///
    /// See [LineSlice::text] for an alternative that keeps the lifetime of original
    /// string slice.
    pub fn as_str(&self) -> &str {
        self.0
    }

    /// Returns the length of the wrapped text, including the terminating character.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> TryFrom<&'a str> for LineSlice<'a> {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some(offset) = value.find('\n') {
            if offset != value.len() - 1 {
                return Err(value);
            }
        }
        return Ok(LineSlice(value));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod try_from {
        use super::*;

        macro_rules! failure_case {
            ($test:ident, $text:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(LineSlice::try_from($text), Err($text));
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $text:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(LineSlice::try_from($text), Ok(LineSlice::new($text)));
                }
            };
        }

        failure_case!(should_fail_with_non_terminating_newlineslice, "\n\n");

        success_case!(should_work_with_empty, "");
        success_case!(should_work_with_single_newlineslice, "\n");
        success_case!(should_work_with_single_whitespace, " ");
        success_case!(should_work_with_terminating_newlineslice, "hello\n");
    }

    mod new {
        use super::*;

        #[test]
        fn should_work_with_valid_lineslice() {
            let slice = LineSlice::new("stuff");
            assert_eq!(slice.0, "stuff");
        }

        #[test]
        #[should_panic]
        fn should_panic_with_invalid_lineslice() {
            LineSlice::new("\n ");
        }
    }
}
