#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combinations = vec![];
        let mut combination = vec![];

        Self::helper(
            &candidates,
            &mut combinations,
            &mut combination,
            0,
            target,
            0,
        );

        combinations
    }

    fn helper(
        candidates: &Vec<i32>,
        combinations: &mut Vec<Vec<i32>>,
        combination: &mut Vec<i32>,
        idx: usize,
        target: i32,
        sum: i32,
    ) {
        if sum == target {
            combinations.push(combination.to_vec());
            return;
        }

        if idx >= candidates.len() || sum > target {
            return;
        }

        combination.push(candidates[idx]);

        Self::helper(
            candidates,
            combinations,
            combination,
            idx,
            target,
            sum + candidates[idx],
        );

        combination.pop();

        Self::helper(candidates, combinations, combination, idx + 1, target, sum);
    }
}
