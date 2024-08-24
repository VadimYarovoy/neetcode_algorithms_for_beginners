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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let mut queue = VecDeque::new();
        queue.push_back(root);

        let mut right_nodes = vec![];

        while !queue.is_empty() {
            let mut value = 0;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();

                value = node.clone().unwrap().borrow().val;

                let left = node.clone().unwrap().borrow().left.clone();

                if left.is_some() {
                    queue.push_back(left);
                }

                let right = node.unwrap().borrow().right.clone();

                if right.is_some() {
                    queue.push_back(right);
                }
            }

            right_nodes.push(value);
        }

        right_nodes
    }
}
