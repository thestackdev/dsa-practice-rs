pub struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut start = 0;

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                nums[start] = nums[i];
                start += 1;
            }
        }

        nums[..start].into()
    }
}
