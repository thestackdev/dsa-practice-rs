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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_within_bounds(root.clone(), None, None)
    }

    fn is_within_bounds(
        node: Option<Rc<RefCell<TreeNode>>>,
        min: Option<i32>,
        max: Option<i32>,
    ) -> bool {
        if let Some(node) = node {
            let n = node.borrow();

            if let Some(min) = min {
                if n.val <= min {
                    return false;
                }
            }

            if let Some(max) = max {
                if n.val >= max {
                    return false;
                }
            }

            Self::is_within_bounds(n.left.clone(), min, Some(n.val))
                && Self::is_within_bounds(n.right.clone(), Some(n.val), max)
        } else {
            true
        }
    }
}
