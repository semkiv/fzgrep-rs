use std::collections::VecDeque;

/// A FIFO accumulator: when at capacity every new item that is fed
/// will pop the oldest item stored in the accumulator.
///
#[derive(Debug)]
pub struct SlidingAccumulator<T> {
    data: Option<VecDeque<T>>,
}

impl<T> SlidingAccumulator<T> {
    /// Creates a new [`SlidingAccumulator`] with capacity `capacity`.
    /// `capacity` can be 0, in which case [`SlidingAccumulator::feed`] does nothing.
    ///
    pub fn new(capacity: usize) -> Self {
        Self {
            data: (capacity > 0).then(|| VecDeque::with_capacity(capacity)),
        }
    }

    /// Pushes an item into the accumulator.
    /// If the accumulator is at capacity, the oldest stored item is popped.
    /// If the capacity is zero does nothing.
    ///
    pub fn feed(&mut self, item: T) {
        if let Some(data) = self.data.as_mut() {
            if data.len() == data.capacity() {
                data.pop_front();
            }

            data.push_back(item);
        }
    }
}

impl<T: Clone> SlidingAccumulator<T> {
    /// Returns the accumulated items as a [`Vec<T>`].
    ///
    pub fn snapshot(&self) -> Option<Vec<T>> {
        self.data.as_ref().map(|data| Vec::from((*data).clone()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sliding_accumulator_constructor_zero_capacity() {
        let acc = SlidingAccumulator::<String>::new(0);
        assert_eq!(acc.data, None);
    }

    #[test]
    fn sliding_accumulator_constructor() {
        let acc = SlidingAccumulator::<String>::new(3);
        assert!(acc.data.as_ref().unwrap().is_empty());
        assert_eq!(acc.data.unwrap().capacity(), 3);
    }

    #[test]
    fn sliding_accumulator_feed() {
        let mut acc = SlidingAccumulator::new(3);
        assert!(acc.data.as_ref().unwrap().is_empty());
        acc.feed(String::from("one"));
        assert_eq!(acc.data, Some(VecDeque::from([String::from("one")])));
        acc.feed(String::from("two"));
        assert_eq!(
            acc.data,
            Some(VecDeque::from([String::from("one"), String::from("two")]))
        );
        acc.feed(String::from("three"));
        assert_eq!(
            acc.data,
            Some(VecDeque::from([
                String::from("one"),
                String::from("two"),
                String::from("three")
            ]))
        );
        acc.feed(String::from("four"));
        assert_eq!(
            acc.data,
            Some(VecDeque::from([
                String::from("two"),
                String::from("three"),
                String::from("four")
            ]))
        );
    }

    #[test]
    fn sliding_accumulator_feed_zero_capacity() {
        let mut acc = SlidingAccumulator::new(0);
        assert_eq!(acc.data, None);
        acc.feed(String::from("something"));
        assert_eq!(acc.data, None);
    }

    #[test]
    fn sliding_accumulator_snapshot() {
        let mut acc = SlidingAccumulator::new(3);
        assert_eq!(acc.snapshot(), Some(Vec::new()));
        acc.feed(String::from("one"));
        assert_eq!(acc.snapshot(), Some(vec![String::from("one")]));
        acc.feed(String::from("two"));
        assert_eq!(
            acc.snapshot(),
            Some(vec![String::from("one"), String::from("two")])
        );
        acc.feed(String::from("three"));
        assert_eq!(
            acc.snapshot(),
            Some(vec![
                String::from("one"),
                String::from("two"),
                String::from("three")
            ])
        );
        acc.feed(String::from("four"));
        assert_eq!(
            acc.snapshot(),
            Some(vec![
                String::from("two"),
                String::from("three"),
                String::from("four")
            ])
        );
    }
}
