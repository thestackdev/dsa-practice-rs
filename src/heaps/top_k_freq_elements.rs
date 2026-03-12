use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = HashMap::new();

        for num in nums {
            *freq.entry(num).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();

        for (key, value) in freq {
            heap.push((value, key));
        }

        (0..k).map(|_| heap.pop().unwrap().1).collect()
    }
}
