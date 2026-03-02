use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut max_window = 0;
        const MAX_BASKETS: usize = 2;

        for right in 0..fruits.len() {
            *map.entry(fruits[right]).or_insert(0) += 1;

            while map.len() > MAX_BASKETS {
                let freq = map.entry(fruits[left]).or_insert(0);

                *freq -= 1;

                if *freq == 0 {
                    map.remove(&fruits[left]);
                }

                left += 1;
            }

            max_window = max_window.max(right - left + 1);
        }

        max_window as i32
    }
}
