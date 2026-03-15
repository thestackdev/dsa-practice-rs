use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, 1);

        let mut count = 0;
        let mut running_sum = 0;

        for num in nums {
            running_sum += num;

            let diff = running_sum - k;
            if let Some(value) = map.get(&diff) {
                count += value;
            }

            *map.entry(running_sum).or_insert(0) += 1;
        }

        count
    }
}

