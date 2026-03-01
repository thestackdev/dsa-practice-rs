pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut start = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[start] = nums[i];

                start += 1;
            }
        }

        start as i32
    }
}
