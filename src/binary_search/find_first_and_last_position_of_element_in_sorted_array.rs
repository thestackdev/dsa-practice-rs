pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let left_insertion_index = Solution::find_insertion_index(&nums, target);

        if !(left_insertion_index >= 0
            && left_insertion_index < nums.len() as i32
            && nums[left_insertion_index as usize] == target)
        {
            return vec![-1, -1];
        }

        let right_insertion_index = Solution::find_insertion_index(&nums, target + 1);

        vec![left_insertion_index, right_insertion_index]
    }

    fn find_insertion_index(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] < target {
                left += 1;
            } else {
                right -= 1;
            }
        }

        left as i32
    }
}
