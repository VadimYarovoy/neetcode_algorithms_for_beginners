#![allow(dead_code)]
struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::with_capacity(nums.len());

        for n in nums {
            if heap.len() < 5 {
                heap.push(n);
            } else {
                heap.push(n);
            }
        }

        let mut ans = None;

        for _ in 0..k {
            ans = heap.pop();
        }

        ans.unwrap()
    }
}
