use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn remove_duplicates_navie(nums: &mut Vec<i32>) -> i32 {
        let mut seen: HashSet<i32> = HashSet::new();

        for num in nums {
            if !seen.contains(num) {
                seen.insert(*num);
            }
        }

        seen.len() as i32
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut start = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[start] = nums[i];

                start += 1;
            }
        }

        start as i32
    }
}
