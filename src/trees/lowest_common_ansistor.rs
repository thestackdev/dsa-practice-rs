use std::cell::RefCell;
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root.as_ref()?;

        let p_val = p.as_ref()?.borrow().val;
        let q_val = q.as_ref()?.borrow().val;

        let curr = node.as_ref().borrow().val;

        if curr == p_val || curr == q_val {
            return root.clone();
        }

        let left = Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());

        match (&left, &right) {
            (None, None) => None,
            (None, Some(_)) => right,
            (Some(_), None) => left,
            (Some(_), Some(_)) => root.clone(),
        }
    }
}
