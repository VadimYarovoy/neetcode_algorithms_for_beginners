#![allow(dead_code)]
use std::cmp::Ordering::{Equal, Greater, Less};

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (mut l, mut r) = (0, matrix.len() as i32 - 1);
    let inner_len = matrix[0].len() - 1;

    while l <= r {
        let mid = (l + r) / 2;

        let arr_first = matrix[mid as usize][0];
        let arr_last = matrix[mid as usize][inner_len];

        if target >= arr_first && target <= arr_last {
            return search(&matrix[mid as usize], target);
        } else if target < arr_first {
            r = mid - 1;
        } else if target > arr_last {
            l = mid + 1;
        }
    }

    false
}

fn search(nums: &Vec<i32>, target: i32) -> bool {
    let (mut l, mut r) = (0, nums.len() as i32 - 1);

    while l <= r {
        let mid = (l + r) / 2;
        match nums[mid as usize].cmp(&target) {
            Equal => {
                return true;
            }
            Greater => {
                r = mid - 1;
            }
            Less => {
                l = mid + 1;
            }
        }
    }
    false
}

#[cfg(test)]
mod binary_matrix_search_tests {
    use super::search_matrix;

    #[test]
    fn contains_target() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(search_matrix(matrix, 3), true);
    }

    #[test]
    fn no_target_value() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(search_matrix(matrix, 70), false);
    }
}
