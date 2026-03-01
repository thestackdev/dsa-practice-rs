use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn two_sum_two(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::new();

        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let sum = numbers[left] + numbers[right];

            match sum.cmp(&target) {
                Ordering::Equal => {
                    result.push((left + 1) as i32);
                    result.push((right + 1) as i32);

                    return result;
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
}
