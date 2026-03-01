pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result = Vec::new();
        if s.len() < p.len() {
            return result;
        }

        let mut left = 0;
        let mut right = 0;

        let mut expected_freq: [u8; 26] = [0; 26];
        let mut window_freq: [u8; 26] = [0; 26];

        for c in p.chars() {
            expected_freq[c as usize - 97] += 1;
        }

        let s: Vec<char> = s.chars().collect();

        while right < s.len() {
            window_freq[s[right] as usize - 97] += 1;
            if right - left + 1 == p.len() {
                if expected_freq == window_freq {
                    result.push(left as i32);
                }

                window_freq[s[left] as usize - 97] -= 1;
                left += 1;
            }

            right += 1;
        }

        result
    }
}
