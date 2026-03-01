use std::mem;

pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut pivot_index = nums.len() - 2;

        while pivot_index as i32 >= 0 && nums[pivot_index] >= nums[pivot_index + 1] {
            pivot_index -= 1;
        }

        if pivot_index as i32 == -1 {
            nums.reverse();
            return;
        }

        let mut right_most_index = nums.len() - 1;

        while right_most_index > 0 && nums[right_most_index] <= nums[pivot_index] {
            right_most_index -= 1;
        }

        nums.swap(pivot_index, right_most_index);

        nums[pivot_index + 1..].reverse();
    }
}
