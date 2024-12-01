//! This module define utilities to represent extractions.
//!
//! An extraction is an operation where a type is constructed by consuming
//! from a sequence of input. The conversion starts at the first element, but
//! could very well stop before the last element. This is in contrast with
//! standard conversion traits, such as [From] and [TryFrom].
//!
//! A successful extraction is expected to return an [Extraction].

/// An extraction is a pair of the extracted value with the remaining content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extraction<E, R> {
    pub extracted: E,
    pub remaining: R,
}

impl<E, R> Extraction<E, R> {
    /// Constructs a new extraction.
    pub fn new(extracted: E, remaining: R) -> Self {
        Self {
            extracted,
            remaining,
        }
    }

    /// Transforms the extracted value using the given function.
    ///
    /// The remaining content is left unchanged.
    pub fn map_extracted<NE>(self, f: impl FnOnce(E) -> NE) -> Extraction<NE, R> {
        Extraction::new(f(self.extracted), self.remaining)
    }

    /// Transforms the remaining content using the given function.
    ///
    /// The extracted value is left unchanged.
    pub fn map_remaining<NR>(self, f: impl FnOnce(R) -> NR) -> Extraction<E, NR> {
        Extraction::new(self.extracted, f(self.remaining))
    }

    pub fn into_tuple(self) -> (E, R) {
        (self.extracted, self.remaining)
    }
}

// TODO: remove R from type parameter and make it an associated type. This will require some refactor...
/// A trait that types can implement to indicate their ability to conditionally be constructed by partially
/// consuming a sequence of input.
pub trait TryExtract<T>: Sized {
    type Remaining;
    type Error;

    /// Tries to extract a value from the given content.
    ///
    /// Upon success, an [Extraction] wrapped in an [Ok] variant is returned. Otherwise, an [Err] variant
    /// is returned.
    fn try_extract(content: T) -> Result<Extraction<Self, Self::Remaining>, Self::Error>;
}
