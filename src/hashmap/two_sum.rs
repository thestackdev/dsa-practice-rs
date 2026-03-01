use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        let mut result = vec![];

        for (i, num) in nums.iter().enumerate() {
            let diff = target - nums[i];

            if seen.contains_key(&diff) {
                let key = seen.get(&diff).unwrap();
                result.push(*key as i32);
                result.push(i as i32);
            } else {
                seen.insert(num, i);
            }
        }

        result
    }
}
