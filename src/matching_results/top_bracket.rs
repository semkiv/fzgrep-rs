/// A container that store up to a max count (capacity) of items sorted in descending order.
///
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TopBracket<T> {
    capacity: usize,
    data: Vec<T>,
}

impl<T> TopBracket<T> {
    /// Creates a empty `TopBracket` with a capacity of `capacity`.
    ///
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: Vec::with_capacity(capacity),
        }
    }

    /// Converts this `TopBracket` into a [`Vec`].
    ///
    pub(crate) fn into_vec(self) -> Vec<T> {
        self.data
    }
}

impl<T: Ord> TopBracket<T> {
    /// Tries to add an `item` to the container. If the container is not at capacity yet, the item is added right away.
    /// If the container is at capacity already one of the two happens:
    /// if the item is larger than the the smallest item present in the container,
    /// the smallest item is removed from the container and the new item is added,
    /// otherwise the new item is simply discarded.
    ///
    /// After each addition the container is sorted to ensure order.
    ///
    /// Returns `true` if the item was actually inserted, `false` otherwise.
    ///
    pub(crate) fn push(&mut self, item: T) -> bool {
        if self.capacity == 0 {
            return false;
        }

        if self.data.len() == self.capacity {
            #[expect(
                clippy::unwrap_used,
                reason = "The capacity is non-zero, so the deque is non-empty and there must at least one element"
            )]
            if item <= *self.data.last().unwrap() {
                return false;
            }

            self.data.pop();
        }

        self.data.push(item);
        self.data.sort_by(|a, b| b.cmp(a));
        true
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::default_numeric_fallback, reason = "It's tests, who cares?")]
    #![expect(clippy::shadow_unrelated, reason = "It's tests, who cares?")]

    use super::*;

    #[test]
    fn constructor() {
        let capacity = 4;
        let container = TopBracket::<i32>::new(capacity);
        assert_eq!(container.capacity, capacity);
        assert_eq!(container.data.len(), 0);
        assert_eq!(container.data.capacity(), capacity);
    }

    #[test]
    fn push() {
        let capacity = 4;
        let mut container = TopBracket::new(capacity);
        assert_eq!(container.capacity, capacity);
        assert_eq!(container.data.len(), 0);
        assert_eq!(container.data.capacity(), capacity);

        assert!(container.push(1));
        assert_eq!(container.data.len(), 1);
        assert_eq!(container.data, [1]);

        assert!(container.push(2));
        assert_eq!(container.data.len(), 2);
        assert_eq!(container.data, [2, 1]);

        assert!(container.push(0));
        assert_eq!(container.data.len(), 3);
        assert_eq!(container.data, [2, 1, 0]);

        assert!(container.push(1));
        assert_eq!(container.data.len(), 4);
        assert_eq!(container.data, [2, 1, 1, 0]);

        assert!(!container.push(-1));
        assert_eq!(container.data.len(), 4);
        assert_eq!(container.data, [2, 1, 1, 0]);

        assert!(!container.push(0));
        assert_eq!(container.data.len(), 4);
        assert_eq!(container.data, [2, 1, 1, 0]);

        assert!(container.push(2));
        assert_eq!(container.data.len(), 4);
        assert_eq!(container.data, [2, 2, 1, 1]);
    }

    #[test]
    fn into_vec() {
        let capacity = 4;
        let container = TopBracket::<i32>::new(capacity);
        assert_eq!(container.into_vec(), []);

        let mut container = TopBracket::new(capacity);
        container.push(1);
        container.push(2);
        assert_eq!(container.into_vec(), [2, 1]);

        let mut container = TopBracket::new(capacity);
        container.push(1);
        container.push(2);
        container.push(0);
        container.push(1);
        container.push(2);
        assert_eq!(container.into_vec(), [2, 2, 1, 1]);
    }
}
