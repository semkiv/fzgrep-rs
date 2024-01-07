use std::collections::VecDeque;

pub(crate) struct SlidingAccumulator {
    capacity: usize,
    data: VecDeque<String>,
}

pub(crate) struct SaturatingAccumulator {
    capacity: usize,
    data: Vec<String>,
}

impl SlidingAccumulator {
    pub(crate) const fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: VecDeque::with_capacity(capacity),
        }
    }

    pub(crate) const fn feed(&mut self, line: String) {
        if self.capacity == 0 {
            return;
        }

        if self.data.len() == self.capacity {
            self.data.pop_front();
        }

        self.data.push_back(line);
    }

    pub(crate) const fn is_full(&self) -> bool {
        false
    }

    pub(crate) const fn snapshot(&self) -> Vec<String> {
        self.data.iter().cloned().collect()
    }

    pub(crate) const fn consume(self) -> Vec<String> {
        self.data.into_iter().collect()
    }
}

impl SaturatingAccumulator {
    pub(crate) const fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: Vec::with_capacity(capacity),
        }
    }

    pub(crate) const fn feed(&mut self, line: String) {
        if self.is_full() {
            return;
        }

        self.data.push(line);
    }

    pub(crate) const fn is_full(&self) -> bool {
        self.data.len() == self.capacity
    }

    pub(crate) const fn snapshot(&self) -> Vec<String> {
        self.data.clone()
    }

    pub(crate) const fn consume(self) -> Vec<String> {
        self.data
    }
}

todo!("documentation!");

#[cfg(test)]
mod test {
    todo!("tests!");
}
