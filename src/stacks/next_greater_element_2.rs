pub struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![-1; n];
        let mut stack = Vec::new();

        for i in (0..2 * n).rev() {
            let idx = i % n;

            while stack.last().is_some_and(|&n| n <= nums[idx]) {
                stack.pop();
            }

            result[idx] = stack.last().copied().unwrap_or(-1);
            stack.push(nums[idx]);
        }

        result
    }
}
