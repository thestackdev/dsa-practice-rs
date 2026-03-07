use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        for num1 in nums1 {
            for i in 0..nums2.len() {
                if num1 == nums2[i] {
                    let next_greater_element = nums2[i + 1..]
                        .iter()
                        .find(|&&n| n > num1)
                        .copied()
                        .unwrap_or(-1);

                    result.push(next_greater_element);
                }
            }
        }

        result
    }

    pub fn next_greater_element_optimised(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut hashmap = HashMap::new();

        for i in 0..nums1.len() {
            hashmap.insert(nums1[i], i);
        }

        for i in (0..nums2.len()).rev() {
            while stack.last().is_some_and(|&ele| ele < nums2[i]) {
                stack.pop();
            }

            if let Some(&value) = hashmap.get(&nums2[i]) {
                nums1[value] = stack.last().copied().unwrap_or(-1);
            }

            stack.push(nums2[i]);
        }

        nums1
    }
}
