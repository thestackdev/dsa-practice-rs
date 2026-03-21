use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back(root.clone());

        while !queue.is_empty() {
            let len = queue.len();

            for i in 0..len {
                if let Some(node) = queue.pop_front().unwrap() {
                    let node = node.borrow();

                    if let Some(left) = &node.left {
                        queue.push_back(node.left.clone());
                    }
                    if let Some(right) = &node.right {
                        queue.push_back(node.right.clone());
                    }

                    if i == len - 1 {
                        result.push(node.val);
                    }
                }
            }
        }

        result
    }
}
