#![allow(dead_code)]
struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);

        while heap.len() > 2 {
            let first = heap.pop().unwrap();
            let second = heap.pop().unwrap();

            match (first - second).abs() {
                0 => {}
                val => heap.push(val),
            }
        }

        let res = if heap.len() == 2 {
            let first = heap.pop().unwrap();
            let second = heap.pop().unwrap();

            (first - second).abs()
        } else {
            heap.pop().unwrap()
        };

        res
    }
}
