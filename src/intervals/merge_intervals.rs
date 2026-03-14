pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut merged = Vec::new();

        if intervals.is_empty() {
            return merged;
        }

        intervals.sort();
        merged.push(intervals.first().unwrap().clone());

        for b in intervals.iter().skip(1) {
            let a = merged.last().unwrap().clone();

            if a[1] < b[0] {
                merged.push(b.clone());
            } else if let Some(last) = merged.last_mut() {
                *last = vec![a[0], a[1].max(b[1])]
            }
        }

        merged
    }
}
