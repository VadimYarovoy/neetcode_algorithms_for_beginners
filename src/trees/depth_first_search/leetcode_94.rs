// Definition for a binary tree node.
#![allow(dead_code)]
struct Solution;
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut arr = vec![];
        Self::helper(root, &mut arr);
        arr
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        match root {
            Some(node) => {
                let left = node.borrow().left.clone();
                if left.is_some() {
                    Self::helper(left, arr);
                }

                let value = node.borrow().val.clone();

                arr.push(value);

                let right = node.borrow().right.clone();
                if right.is_some() {
                    Self::helper(right, arr);
                }
            }
            None => {}
        }
    }
}
