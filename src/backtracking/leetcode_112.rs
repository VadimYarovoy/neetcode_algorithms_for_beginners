#![allow(dead_code)]
struct Solution;

// Definition for a binary tree node.
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
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let n = node.borrow();
            if n.left.is_none() && n.right.is_none() && n.val == target_sum {
                return true;
            }

            return Self::has_path_sum(n.left.clone(), target_sum - n.val)
                || Self::has_path_sum(n.right.clone(), target_sum - n.val);
        }

        false
    }
}

#[cfg(test)]
mod has_path_sum_tests {
    use super::*;

    #[test]
    fn simple_test() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));

        assert_eq!(Solution::has_path_sum(tree.clone(), 3), true);
        assert_eq!(Solution::has_path_sum(tree.clone(), 4), true);
        assert_eq!(Solution::has_path_sum(tree, 5), false);
    }
}
