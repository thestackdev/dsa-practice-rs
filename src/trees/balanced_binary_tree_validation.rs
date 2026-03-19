use std::cell::{Ref, RefCell};
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check_balanced(&root) != -1
    }

    fn check_balanced(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let n = n.borrow_mut();

                let left_height = Self::check_balanced(&n.left);
                if left_height == -1 {
                    return -1;
                }

                let right_height = Self::check_balanced(&n.right);
                if right_height == -1 {
                    return -1;
                }

                if (left_height - right_height).abs() > 1 {
                    -1
                } else {
                    1 + left_height.max(right_height)
                }
            }
        }
    }
}
