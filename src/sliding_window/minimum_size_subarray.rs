use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut current_sum = 0;
        let mut min_length = usize::MAX;

        for right in 0..nums.len() {
            current_sum += nums[right];

            while current_sum >= target {
                min_length = min_length.min(right - left + 1);
                current_sum -= nums[left];
                left += 1;
            }
        }

        if min_length == usize::MAX {
            0
        } else {
            min_length as i32
        }
    }
}
