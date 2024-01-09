use std::collections::VecDeque;

/// A FIFO-like context accumulator: when at capacity every new line that is fed
/// will pop the oldest line stored in the accumulator.
///
#[derive(Debug, PartialEq)]
pub(crate) struct SlidingAccumulator {
    capacity: usize,
    data: VecDeque<String>,
}

/// A context accumulator that accumulates line up to a certain number.
/// After the capacity is reached, feeding more lines has no effect.
#[derive(Debug, PartialEq)]
pub(crate) struct SaturatingAccumulator {
    capacity: usize,
    data: Vec<String>,
}

impl SlidingAccumulator {
    /// Creates a new [`SlidingAccumulator`] with capacity `capacity`.
    /// `capacity` can be 0, in which case [`feed`] does nothing.
    ///
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: VecDeque::with_capacity(capacity),
        }
    }

    /// Pushes a line into the accumulator.
    /// If the accumulator is at capacity, the oldest stored line is popped.
    /// If the capacity is zero does nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// let acc = SlidingAccumulator::new(2);
    /// assert_eq!(acc.snapshot(), Vec::<String>::new());
    /// acc.feed(String::from("one"));
    /// assert_eq!(acc.snapshot(), vec!["one"]);
    /// acc.feed(String::from("two"));
    /// assert_eq!(acc.snapshot(), vec!["one", "two"]);
    /// acc.feed(String::from("three"));
    /// assert_eq!(acc.snapshot(), vec!["two", "three"]);
    /// ```
    ///
    /// ```
    /// let acc = SlidingAccumulator::new(0);
    /// assert_eq!(acc.snapshot(), Vec::<String>::new());
    /// acc.feed(String::from("something"));
    /// assert_eq!(acc.snapshot(), Vec::<String>::new());
    /// ```
    ///
    pub(crate) fn feed(&mut self, line: String) {
        if self.capacity == 0 {
            return;
        }

        if self.data.len() == self.capacity {
            self.data.pop_front();
        }

        self.data.push_back(line);
    }

    /// Returns the accumulated lines as a [`Vec<String>`].
    ///
    /// # Examples
    ///
    /// ```
    /// let acc = SlidingAccumulator::new(2);
    /// assert_eq!(acc.snapshot(), Vec::<String>::new());
    /// acc.feed(String::from("one"));
    /// assert_eq!(acc.snapshot(), vec!["one"]);
    /// acc.feed(String::from("two"));
    /// assert_eq!(acc.snapshot(), vec!["one", "two"]);
    /// acc.feed(String::from("three"));
    /// assert_eq!(acc.snapshot(), vec!["two", "three"]);
    /// ```
    ///
    pub(crate) fn snapshot(&self) -> Vec<String> {
        self.data.iter().cloned().collect()
    }
}

impl SaturatingAccumulator {
    /// Creates a new [`SaturatingAccumulator`] with capacity `capacity`.
    /// `capacity` can be 0, in which case [`feed`] does nothing.
    ///
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: Vec::with_capacity(capacity),
        }
    }

    /// Pushes a line into the accumulator.
    /// If the accumulator is at capacity, new lines are ignored.
    /// If the capacity is zero does nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// let acc = SaturatingAccumulator::new(2);
    /// acc.feed(String::from("one"));
    /// acc.feed(String::from("two"));
    /// acc.feed(String::from("three"));
    /// assert_eq!(acc.consume(), vec!["one", "two"]);
    /// ```
    ///
    /// ```
    /// let acc = SaturatingAccumulator::new(0);
    /// acc.feed(String::from("something"));
    /// assert_eq!(acc.consume(), Vec::<String>::new());
    /// ```
    ///
    pub(crate) fn feed(&mut self, line: String) {
        if self.is_saturated() {
            return;
        }

        self.data.push(line);
    }

    /// Returns whether the accumulator is completely filled up.
    ///
    /// # Examples
    ///
    /// ```
    /// let acc = SaturatingAccumulator::new(2);
    /// assert!(!acc.is_saturated())
    /// acc.feed(String::from("one"));
    /// assert!(!acc.is_saturated())
    /// acc.feed(String::from("two"));
    /// assert!(acc.is_saturated())
    /// acc.feed(String::from("three"));
    /// assert!(acc.is_saturated())
    /// ```
    pub(crate) fn is_saturated(&self) -> bool {
        self.data.len() == self.capacity
    }

    /// Turns the accumulator into a [`Vec<String>`] of accumulated lines.
    ///
    /// # Examples
    ///
    /// ```
    /// let acc = SaturatingAccumulator::new(2);
    /// acc.feed(String::from("one"));
    /// acc.feed(String::from("two"));
    /// acc.feed(String::from("three"));
    /// assert_eq!(acc.consume(), vec!["two", "three"]);
    /// ```
    ///
    pub(crate) fn consume(self) -> Vec<String> {
        self.data
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sliding_accumulator_constructor() {
        let mut acc = SlidingAccumulator::new(3);
        assert_eq!(
            acc,
            SlidingAccumulator {
                capacity: 3,
                data: VecDeque::new(),
            }
        );
    }

    #[test]
    fn sliding_accumulator_feed() {
        let mut acc = SlidingAccumulator::new(3);
        assert_eq!(acc.data, VecDeque::from([]));
        acc.feed(String::from("one"));
        assert_eq!(acc.data, VecDeque::from([String::from("one")]));
        acc.feed(String::from("two"));
        assert_eq!(
            acc.data,
            VecDeque::from([String::from("one"), String::from("two")])
        );
        acc.feed(String::from("three"));
        assert_eq!(
            acc.data,
            VecDeque::from([
                String::from("one"),
                String::from("two"),
                String::from("three")
            ])
        );
        acc.feed(String::from("four"));
        assert_eq!(
            acc.data,
            VecDeque::from([
                String::from("two"),
                String::from("three"),
                String::from("four")
            ])
        );
    }

    #[test]
    fn sliding_accumulator_feed_zero_capacity() {
        let mut acc = SlidingAccumulator::new(0);
        assert_eq!(acc.data, VecDeque::from([]));
        acc.feed(String::from("something"));
        assert_eq!(acc.data, VecDeque::from([]));
    }

    #[test]
    fn sliding_accumulator_snapshot() {
        let mut acc = SlidingAccumulator::new(3);
        assert_eq!(acc.snapshot(), [""; 0]);
        acc.feed(String::from("one"));
        assert_eq!(acc.snapshot(), ["one"]);
        acc.feed(String::from("two"));
        assert_eq!(acc.snapshot(), ["one", "two"]);
        acc.feed(String::from("three"));
        assert_eq!(acc.snapshot(), ["one", "two", "three"]);
        acc.feed(String::from("four"));
        assert_eq!(acc.snapshot(), ["two", "three", "four"]);
    }

    #[test]
    fn saturating_accumulator_constructor() {
        let mut acc = SaturatingAccumulator::new(3);
        assert_eq!(
            acc,
            SaturatingAccumulator {
                capacity: 3,
                data: Vec::new(),
            }
        );
    }

    #[test]
    fn saturating_accumulator_feed() {
        let mut acc = SaturatingAccumulator::new(3);
        assert_eq!(acc.data, [""; 0]);
        acc.feed(String::from("one"));
        assert_eq!(acc.data, [String::from("one")]);
        acc.feed(String::from("two"));
        assert_eq!(acc.data, [String::from("one"), String::from("two")]);
        acc.feed(String::from("three"));
        assert_eq!(
            acc.data,
            [
                String::from("one"),
                String::from("two"),
                String::from("three")
            ]
        );
        acc.feed(String::from("four"));
        assert_eq!(
            acc.data,
            [
                String::from("one"),
                String::from("two"),
                String::from("three")
            ]
        );
    }

    #[test]
    fn saturating_accumulator_feed_zero_capacity() {
        let mut acc = SaturatingAccumulator::new(3);
        assert!(!acc.is_saturated());
        acc.feed(String::from("one"));
        assert!(!acc.is_saturated());
        acc.feed(String::from("two"));
        assert!(!acc.is_saturated());
        acc.feed(String::from("three"));
        assert!(acc.is_saturated());
        acc.feed(String::from("four"));
        assert!(acc.is_saturated());
    }

    #[test]
    fn saturating_accumulator_is_saturated() {
        let mut acc = SaturatingAccumulator::new(0);
        assert_eq!(acc.data, [""; 0]);
        acc.feed(String::from("something"));
        assert_eq!(acc.data, [""; 0]);
    }

    #[test]
    fn saturating_accumulator_consume() {
        let mut acc = SaturatingAccumulator::new(3);
        acc.feed(String::from("one"));
        acc.feed(String::from("two"));
        acc.feed(String::from("three"));
        acc.feed(String::from("four"));
        assert_eq!(acc.consume(), ["one", "two", "three"]);
    }
}
