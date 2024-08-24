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
use std::cmp::Ordering::{Equal, Greater, Less};
use std::rc::Rc;
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => match (node.borrow().val).cmp(&val) {
                Equal => Some(node.clone()),
                Greater => Self::search_bst(node.borrow().left.clone(), val),
                Less => return Self::search_bst(node.borrow().right.clone(), val),
            },
        }
    }
}
