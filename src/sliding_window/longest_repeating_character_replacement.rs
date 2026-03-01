pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut left = 0;
        let mut count: [usize; 26] = [0; 26];
        let s: Vec<char> = s.chars().collect();

        let mut max_freq = 0;
        let mut result = 0;

        for right in 0..s.len() {
            let rc = (s[right] as u8 - b'A') as usize;
            count[rc] += 1;

            max_freq = max_freq.max(count[rc]);

            while (right - left + 1) - max_freq > k as usize {
                let lc = (s[left] as u8 - b'A') as usize;
                count[lc] -= 1;
                left += 1;
            }

            result = result.max(right - left + 1);
        }

        result as i32
    }
}
