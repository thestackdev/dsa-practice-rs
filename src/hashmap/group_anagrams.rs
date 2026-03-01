use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut char_freq: [u8; 26] = [0; 26];
            s.chars()
                .map(|c| c.to_ascii_lowercase() as u8 - 97)
                .for_each(|u| char_freq[u as usize] += 1);

            map.entry(char_freq).or_default().push(s);
        }

        map.into_values().collect()
    }
}
