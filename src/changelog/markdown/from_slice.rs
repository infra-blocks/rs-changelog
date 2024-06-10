use crate::changelog::error::ChangelogParseError;
use crate::changelog::markdown::MarkdownError;
use markdown::mdast::Node;

pub trait TryFromSlice
where
    Self: Sized,
{
    /// Returns the slice minus the nodes that were consumed to produce the result.
    fn try_from_slice(
        slice: &[Node],
    ) -> Result<(&[Node], Self), ChangelogParseError<MarkdownError>>;
}

impl<T: TryFromSlice> TryFromSlice for Vec<T> {
    fn try_from_slice(
        slice: &[Node],
    ) -> Result<(&[Node], Self), ChangelogParseError<MarkdownError>> {
        let mut vec = vec![];
        let mut slice = slice;

        loop {
            match T::try_from_slice(slice) {
                Ok((remaining, item)) => {
                    slice = remaining;
                    vec.push(item);
                }
                Err(_) => {
                    break;
                }
            }
        }

        Ok((slice, vec))
    }
}
