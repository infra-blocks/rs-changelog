// TODO: decide how to implement FromIterator.
/// Stack data structure.
///
/// It exposes the typical stack operations: [Stack::pop], [Stack::push] and basic
/// collection methods.
///
/// A [Stack] can be constructed from an iterator through the [FromIterator] trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Constructs a new, empty [Stack].
    pub fn empty() -> Self {
        Self { items: Vec::new() }
    }

    /// Removes and returns the first item from the [Stack], if any.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Pushes a new item onto the [Stack], becoming the first in line to get popped.
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Returns whether the [Stack] is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the number of items in the [Stack].
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T> FromIterator<T> for Stack<T> {
    /// Preserves ordering.
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut items: Vec<_> = iter.into_iter().collect();
        items.reverse();
        Self { items }
    }
}
