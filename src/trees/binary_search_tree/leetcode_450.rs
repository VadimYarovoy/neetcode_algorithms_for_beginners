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
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&root, key)
    }

    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let val = node.borrow().val;
            match val.cmp(&key) {
                Greater => {
                    let l = Self::helper(&node.borrow().left, key);
                    node.borrow_mut().left = l;
                }
                Equal => {
                    if node.borrow().left.is_none() {
                        return node.borrow().right.clone();
                    }

                    if node.borrow().right.is_none() {
                        return node.borrow().left.clone();
                    }

                    let min_val = Self::min_val(&node.borrow().right);

                    if let Some(val) = min_val {
                        let r = Self::helper(&node.borrow().right, val);
                        node.borrow_mut().val = val;
                        node.borrow_mut().right = r;
                    }
                }
                Less => {
                    let r = Self::helper(&node.borrow().right, key);
                    node.borrow_mut().right = r;
                }
            }
        }

        root.clone()
    }

    fn min_val(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(n) = node {
            if n.borrow().left.is_some() {
                Self::min_val(&n.borrow().left)
            } else {
                Some(n.borrow().val)
            }
        } else {
            None
        }
    }
}
