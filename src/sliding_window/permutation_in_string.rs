use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let mut left = 0;
        let mut expected_freq: [usize; 26] = [0; 26];
        let mut running_freq: [usize; 26] = [0; 26];

        for c in s1.chars() {
            let ci = c as u8 - b'a';
            expected_freq[ci as usize] += 1;
        }

        let s2: Vec<char> = s2.chars().collect();

        for right in 0..s2.len() {
            let ri = s2[right] as u8 - b'a';
            running_freq[ri as usize] += 1;

            if right - left + 1 == s1.len() {
                if expected_freq == running_freq {
                    return true;
                }

                let li = s2[left] as u8 - b'a';
                running_freq[li as usize] -= 1;

                left += 1;
            }
        }

        false
    }
}
