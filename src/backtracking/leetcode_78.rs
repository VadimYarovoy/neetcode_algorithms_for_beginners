#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subsets = vec![];
        let mut subset = vec![];

        Self::helper(&nums, &mut subsets, &mut subset, 0);

        subsets
    }

    fn helper(nums: &Vec<i32>, subsets: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>, idx: usize) {
        if idx == nums.len() {
            subsets.push(subset.to_vec());
            return;
        }

        subset.push(nums[idx]);
        Self::helper(nums, subsets, subset, idx + 1);

        subset.pop();
        Self::helper(nums, subsets, subset, idx + 1);
    }
}
