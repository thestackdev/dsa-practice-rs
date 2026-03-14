pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;

        let mut i = 1;
        let mut current_index = 0;
        intervals.sort();

        while i < intervals.len() {
            let (a, b) = (&intervals[current_index], &intervals[i]);

            let start = a[0].max(b[0]);
            let end = a[1].min(b[1]);

            if start < end {
                count += 1;
                if intervals[i][1] < intervals[current_index][1] {
                    current_index = i;
                }
            } else {
                current_index = i;
            }
            i += 1;
        }

        count
    }
}
