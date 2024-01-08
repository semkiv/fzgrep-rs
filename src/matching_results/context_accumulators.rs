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
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: VecDeque::with_capacity(capacity),
        }
    }

    pub(crate) fn feed(&mut self, line: String) {
        if self.capacity == 0 {
            return;
        }

        if self.data.len() == self.capacity {
            self.data.pop_front();
        }

        self.data.push_back(line);
    }

    pub(crate) fn snapshot(&self) -> Vec<String> {
        self.data.iter().cloned().collect()
    }
}

impl SaturatingAccumulator {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: Vec::with_capacity(capacity),
        }
    }

    pub(crate) fn feed(&mut self, line: String) {
        if self.is_full() {
            return;
        }

        self.data.push(line);
    }

    pub(crate) fn is_full(&self) -> bool {
        self.data.len() == self.capacity
    }

    pub(crate) fn consume(self) -> Vec<String> {
        self.data
    }
}

todo!("documentation!");

#[cfg(test)]
mod test {
    todo!("tests!");
}
