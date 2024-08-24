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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut n = 0;
        let mut stack = vec![];
        let mut curr = root;

        while curr.is_some() || !stack.is_empty() {
            while curr.is_some() {
                stack.push(curr.clone());
                curr = curr.unwrap().borrow().left.clone();
            }

            curr = stack.pop().unwrap();

            n += 1;

            if n == k {
                return curr.unwrap().borrow().val;
            }

            curr = curr.unwrap().borrow().right.clone();
        }

        unreachable!()
    }
}
