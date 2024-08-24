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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut layers = vec![vec![]];
        let mut nodes = VecDeque::new();
        nodes.push_back(root);

        while !nodes.is_empty() {
            let mut layer = vec![];
            for _ in 0..nodes.len() {
                let node = nodes.pop_front().unwrap();
                layer.push(node.clone().unwrap().borrow().val);

                let left = node.clone().unwrap().borrow().left.clone();

                if left.is_some() {
                    nodes.push_back(left);
                }

                let right = node.clone().unwrap().borrow().right.clone();

                if right.is_some() {
                    nodes.push_back(right);
                }
            }

            layers.push(layer);
        }

        layers
    }
}
