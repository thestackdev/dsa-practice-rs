use std::cell::RefCell;
use std::cmp::max;
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
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut max_width = i32::MIN;

        queue.push_back((root.clone(), 0));

        while !queue.is_empty() {
            let len = queue.len();

            let mut first_index = 0;

            for i in 0..len {
                let (node, index) = queue.pop_front().unwrap();

                if let Some(node) = node {
                    let node = node.borrow();

                    if let Some(left) = &node.left {
                        queue.push_back((node.left.clone(), 2 * index + 1));
                    }
                    if let Some(right) = &node.right {
                        queue.push_back((node.right.clone(), 2 * index + 2));
                    }
                }

                if i == 0 {
                    first_index = index;
                }
                if i == len - 1 {
                    max_width = max_width.max(index - first_index + 1);
                }
            }
        }

        max_width
    }
}
