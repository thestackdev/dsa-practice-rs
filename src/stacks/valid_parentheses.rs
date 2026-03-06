use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let mut parentheses = HashMap::new();

        parentheses.insert('(', ')');
        parentheses.insert('{', '}');
        parentheses.insert('[', ']');

        for c in s.chars() {
            if parentheses.contains_key(&c) {
                stack.push(c);
            } else {
                match stack.last() {
                    Some(ele) if parentheses.get(ele).unwrap() == &c => {
                        stack.pop();
                    }
                    Some(_) => return false,
                    None => return false,
                }
            }
        }

        stack.is_empty()
    }
}
