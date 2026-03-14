use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {
    left_heap: BinaryHeap<i32>,
    right_heap: BinaryHeap<Reverse<i32>>,
}

impl Solution {
    fn new() -> Self {
        Self {
            left_heap: BinaryHeap::new(),
            right_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.left_heap.is_empty() || &num <= self.left_heap.peek().unwrap() {
            self.left_heap.push(num);

            if self.left_heap.len() > self.right_heap.len() {
                let val = self.left_heap.pop().unwrap();
                self.right_heap.push(Reverse(val));
            }
        } else {
            self.right_heap.push(Reverse(num));

            if self.right_heap.len() > self.left_heap.len() {
                let Reverse(val) = self.right_heap.pop().unwrap();
                self.left_heap.push(val);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.left_heap.len() == self.right_heap.len() {
            return (*self.left_heap.peek().unwrap_or(&0) as f64
                + self.right_heap.peek().unwrap_or(&Reverse(0)).0 as f64)
                / 2.0;
        }

        *self.left_heap.peek().unwrap_or(&0) as f64
    }
}
