pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] == target {
                return mid as i32;
            }

            if nums[left] <= nums[mid] {
                if nums[left] <= target && target < nums[mid] {
                    right = mid
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[right - 1] {
                    left = mid + 1
                } else {
                    right = mid;
                }
            }
        }

        if !nums.is_empty() && left >= 0 && left < nums.len() && nums[left] == target {
            return left as i32;
        }

        -1
    }
}
