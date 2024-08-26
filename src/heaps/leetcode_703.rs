#![allow(dead_code)]
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: i32,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::from(nums.iter().map(|&x| Reverse(x)).collect::<Vec<_>>());
        while heap.len() > k as usize {
            heap.pop();
        }

        Self { heap, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k as usize {
            self.heap.pop();
        }

        let Reverse(res) = *self.heap.peek().unwrap();

        res
    }
}
