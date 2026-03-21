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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut global_max = i32::MIN;
        Self::dfs(&root, &mut global_max);
        global_max
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, global_max: &mut i32) -> i32 {
        let node = match node {
            None => return 0,
            Some(val) => val.borrow(),
        };

        let left = Self::dfs(&node.left, global_max).max(0);
        let right = Self::dfs(&node.right, global_max).max(0);

        *global_max = (*global_max).max(node.val + left + right);

        node.val + left.max(right)
    }
}

