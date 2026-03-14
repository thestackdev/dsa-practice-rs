pub struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        let mut i = 0;
        let mut j = 0;

        while i < first_list.len() && j < second_list.len() {
            let (a, b) = (&first_list[i], &second_list[j]);

            let start = a[0].max(b[0]);
            let end = a[1].min(b[1]);

            if start <= end {
                result.push(vec![start, end]);
            }

            if a[1] < b[1] {
                i += 1;
            } else {
                j += 1;
            }
        }

        result
    }
}
