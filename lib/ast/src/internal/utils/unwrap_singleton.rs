pub trait UnwrapSingleton<T> {
    fn unwrap_singleton(self) -> T;
}

impl<T: IntoIterator<Item = U>, U> UnwrapSingleton<U> for T {
    fn unwrap_singleton(self) -> U {
        let mut iter = self.into_iter();
        let result = iter
            .next()
            .expect("cannot unwrap singleton on empty collection");
        assert!(
            iter.next().is_none(),
            "unwrap singleton on collection with at least 2 elements"
        );
        return result;
    }
}
