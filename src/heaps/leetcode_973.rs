#![allow(dead_code)]
struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let dists = points
            .into_iter()
            .map(|v| {
                let x = v[0];
                let y = v[1];

                (Reverse(x.pow(2) + y.pow(2)), v)
            })
            .collect::<Vec<_>>();

        let mut heap = BinaryHeap::from(dists);

        let mut res = Vec::with_capacity(k as usize);

        for _ in 0..k {
            res.push(heap.pop().unwrap().1)
        }

        res
    }
}
