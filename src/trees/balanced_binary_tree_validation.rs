use std::cell::{Ref, RefCell};
use std::rc::{self, Rc};

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
        if let Some(node) = node {
            let n = node.borrow_mut();
            let left = Self::check_balanced(&n.left);
            if left == -1 {
                return -1;
            }

            let right = Self::check_balanced(&n.right);
            if right == -1 {
                return -1;
            }

            if (left - right).abs() > 1 {
                return -1;
            } else {
                return 1 + left.max(right);
            }
        }

        0
    }
}
