use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    fn two_sum(numbers: &Vec<i32>, mut left: usize, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut right = numbers.len() - 1;

        while left < right {
            let sum = numbers[left] + numbers[right];

            match sum.cmp(&target) {
                Ordering::Equal => {
                    result.push(vec![numbers[left], numbers[right]]);

                    while left < right && numbers[left] == numbers[left + 1] {
                        left += 1;
                    }
                    while right > left && numbers[right] == numbers[right - 1] {
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

        result
    }

    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        nums.sort();

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let two_sum_result = Solution::two_sum(&nums, i + 1, -nums[i]);

            for res in two_sum_result {
                let tmp = vec![nums[i], res[0], res[1]];
                result.push(tmp);
            }
        }

        result
    }
}
