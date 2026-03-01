use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut left = 0;
        let mut running_sum: i64 = 0;
        let mut result: i64 = 0;
        let mut map: HashMap<i32, usize> = HashMap::new();

        for right in 0..nums.len() {
            *map.entry(nums[right]).or_insert(0) += 1;
            running_sum += nums[right] as i64;

            if (right - left + 1) == k as usize {
                if map.len() == k as usize {
                    result = result.max(running_sum);
                }

                let freq = map.entry(nums[left]).or_insert(0);
                *freq -= 1;

                if *freq == 0 {
                    map.remove(&nums[left]);
                }

                running_sum -= nums[left] as i64;
                left += 1;
            }
        }

        result
    }
}
