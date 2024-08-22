use std::cmp::Ordering::{Equal, Greater, Less};

#[allow(dead_code)]
fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r) = (0, nums.len() as i32 - 1);

    while l <= r {
        let mid = (l + r) / 2;
        match nums[mid as usize].cmp(&target) {
            Equal => {
                return mid;
            }
            Greater => {
                r = mid - 1;
            }
            Less => {
                l = mid + 1;
            }
        }
    }
    -1
}

#[cfg(test)]
mod binary_search_tests {
    use super::search;

    #[test]
    fn contains_target() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(search(nums, 9), 4);
    }

    #[test]
    fn no_target_value() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(search(nums, 2), -1);
    }
}
