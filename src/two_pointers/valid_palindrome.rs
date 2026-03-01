pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<u8> = s
            .as_bytes()
            .iter()
            .filter(|b| b.is_ascii_alphanumeric())
            .map(|b| b.to_ascii_lowercase())
            .collect();

        let mut left = 0;
        let mut right = s.len().saturating_sub(1);

        while left < right {
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}
