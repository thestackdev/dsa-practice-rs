pub struct Solution;

impl Solution {
    pub fn next_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut result = Vec::new();

        for i in (0..nums.len()).rev() {
            while stack.last().is_some_and(|&top| top < nums[i]) {
                stack.pop();
            }

            result.push(stack.last().copied().unwrap_or(-1));
            stack.push(nums[i]);
        }

        result.reverse();
        result
    }
}
