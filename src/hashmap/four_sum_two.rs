use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut count = 0;

        let mut map = HashMap::new();

        for i in &nums1 {
            for j in &nums2 {
                *map.entry(i + j).or_insert(0) += 1;
            }
        }

        for k in &nums3 {
            for l in &nums4 {
                if let Some(value) = map.get(&-(k + l)) {
                    count += value
                }
            }
        }

        count
    }
}
