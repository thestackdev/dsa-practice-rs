pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut new = new_interval.clone();
        let mut inserted = false;

        for interval in intervals {
            if interval[1] < new[0] {
                result.push(interval);
            } else if interval[0] > new[1] {
                if !inserted {
                    result.push(new.clone());
                    inserted = true;
                }
                result.push(interval)
            } else {
                new[0] = new[0].min(interval[0]);
                new[1] = new[1].max(interval[1]);
            }
        }

        if !inserted {
            result.push(new.clone());
        }

        result
    }
}
