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

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            let mut n = node.borrow_mut();

            let left = Self::invert_tree(n.left.take());
            let right = Self::invert_tree(n.right.take());

            n.left = right;
            n.right = left;
        }

        root
    }

    pub fn invert_tree_stack(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![root.clone()];

        while let Some(node) = stack.pop() {
            if let Some(n) = node {
                let mut n = n.borrow_mut();

                let left = n.left.take();
                let right = n.right.take();

                n.left = right.clone();
                n.right = left.clone();

                Self::invert_tree_stack(left);
                Self::invert_tree_stack(right);
            }
        }

        root
    }
}
