#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TopBracket<T> {
    capacity: usize,
    data: Vec<T>,
}

impl<T> TopBracket<T> {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: Vec::with_capacity(capacity),
        }
    }

    pub(crate) fn into_vec(self) -> Vec<T> {
        self.data
    }
}

impl<T: Ord> TopBracket<T> {
    pub(crate) fn push(&mut self, item: T) -> bool {
        if self.data.len() == self.capacity {
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