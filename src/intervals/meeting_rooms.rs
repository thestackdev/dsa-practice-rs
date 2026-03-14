#[derive(Debug, Clone)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

pub struct Solution;

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Interval>) -> bool {
        intervals.sort_by_key(|a| a.start);

        for a in 1..intervals.len() {
            if intervals[a - 1].end > intervals[a].start {
                return false;
            }
        }

        true
    }
}
