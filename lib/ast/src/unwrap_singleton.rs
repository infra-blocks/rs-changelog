use std::collections::VecDeque;

pub trait UnwrapSingleton<T> {
    fn unwrap_singleton(self) -> T;
}

impl<T> UnwrapSingleton<T> for VecDeque<T> {
    fn unwrap_singleton(self) -> T {
        if self.len() == 1 {
            self.into_iter().next().unwrap()
        } else {
            panic!(
                "trying to unwrap a single element on vec, but found {:?} elements",
                self.len()
            );
        }
    }
}
