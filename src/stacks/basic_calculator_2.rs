pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut sign = '+';
        let mut curr_num = 0;

        for c in s.chars().filter(|&c| c != ' ').chain(std::iter::once('+')) {
            if c.is_ascii_digit() {
                curr_num = curr_num * 10 + c.to_digit(10).unwrap() as i32;
            } else {
                match sign {
                    '+' => stack.push(curr_num),
                    '-' => stack.push(-curr_num),
                    '*' => {
                        let top = stack.pop().unwrap();
                        stack.push(top * curr_num);
                    }
                    '/' => {
                        let top = stack.pop().unwrap();
                        stack.push(top / curr_num);
                    }
                    _ => {}
                }

                curr_num = 0;
                sign = c;
            }
        }

        stack.iter().sum()
    }
}
