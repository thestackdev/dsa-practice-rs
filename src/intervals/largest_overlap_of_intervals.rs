pub struct Solution;

impl Solution {
    pub fn largest_overlap(nums: Vec<Vec<i32>>) -> i32 {
        let mut max_overlap = 0;
        let mut active_overlap = 0;

        let mut points = Vec::new();

        for num in &nums {
            points.push((num[0], 'S'));
            points.push((num[1], 'E'));
        }

        points.sort();

        for (num, point_type) in points {
            if point_type == 'S' {
                active_overlap += 1;
            } else {
                active_overlap -= 1;
            }

            max_overlap = max_overlap.max(active_overlap);
        }

        max_overlap
    }
}
