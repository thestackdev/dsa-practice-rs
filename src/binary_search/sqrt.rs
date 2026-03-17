pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 1 {
            return x;
        }

        let mut left = 1i64;
        let mut right = x as i64;

        while left < right {
            let mid = left + (right - left + 1) / 2;

            if mid * mid > x as i64 {
                right = mid - 1;
            } else {
                left = mid;
            }
        }

        left as i32
    }
}
