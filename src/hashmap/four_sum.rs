use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        let n = nums.len();

        if n < 4 {
            return result;
        }

        nums.sort();
        let target = target as i64;

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            for j in (i + 1)..nums.len() {
                if j > (i + 1) && nums[j] == nums[j - 1] {
                    continue;
                }

                let mut left = j + 1;
                let mut right = n - 1;

                while left < right {
                    let sum =
                        nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;

                    match sum.cmp(&target) {
                        Ordering::Equal => {
                            result.push(vec![nums[i], nums[j], nums[left], nums[right]]);

                            while left < right && nums[left] == nums[left + 1] {
                                left += 1;
                            }

                            while right > left && nums[right] == nums[right - 1] {
                                right -= 1;
                            }

                            left += 1;
                            right -= 1;
                        }
                        Ordering::Less => {
                            left += 1;
                        }
                        Ordering::Greater => {
                            right -= 1;
                        }
                    }
                }
            }
        }

        result
    }
}
