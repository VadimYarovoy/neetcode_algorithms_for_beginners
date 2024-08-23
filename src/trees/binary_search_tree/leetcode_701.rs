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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root.clone() {
            None => {
                return Some(Rc::new(RefCell::new(TreeNode::new(val))));
            }
            Some(node) => {
                let mut node = node.borrow_mut();

                if val < node.val {
                    node.left = Self::insert_into_bst(node.left.clone(), val)
                } else {
                    node.right = Self::insert_into_bst(node.right.clone(), val)
                }
            }
        }

        root
    }
}
