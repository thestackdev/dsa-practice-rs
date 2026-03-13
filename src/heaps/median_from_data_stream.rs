use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianFinder {
    left_half: BinaryHeap<i32>,
    right_half: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            left_half: BinaryHeap::new(),
            right_half: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.left_half.is_empty() || &num <= self.left_half.peek().unwrap() {
            self.left_half.push(num);

            if self.left_half.len() > self.right_half.len() {
                let val = self.left_half.pop().unwrap();
                self.right_half.push(Reverse(val));
            }
        } else {
            self.right_half.push(Reverse(num));

            if self.right_half.len() > self.left_half.len() {
                let val = self.right_half.pop().unwrap();
                self.left_half.push(Reverse(val));
            }
        }
    }

    fn find_median(&self) -> f64 {}
}

