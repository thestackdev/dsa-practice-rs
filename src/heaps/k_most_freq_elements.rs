use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut freq = HashMap::new();

        for word in words {
            *freq.entry(word).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();

        for (key, value) in freq {
            heap.push((value, Reverse(key)));
        }

        let x = heap.pop().unwrap();
        let x = x.1;

        (0..k).map(|_| heap.pop().unwrap().1.0).collect()
    }
}
