// Remove Duplicates from Sorted Array
// Time O(n), Space O(1)
fn _remove_duplicates(nums: &mut [i32]) -> i32 {
    let mut l = 1;

    for r in 1..nums.len() {
        if nums[r] != nums[r - 1] {
            nums[l] = nums[r];
            l += 1;
        }
    }

    l as _
}

#[cfg(test)]
mod remove_duplicates_tests {
    use super::_remove_duplicates;

    #[test]
    fn one_duplicate() {
        let mut nums = [1, 1, 2];
        let new_len = _remove_duplicates(&mut nums) as usize;
        assert_eq!(new_len, 2);
        assert_eq!(nums[..new_len], [1, 2]);
    }

    #[test]
    fn many_duplicates() {
        let mut nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let new_len = _remove_duplicates(&mut nums) as usize;
        assert_eq!(new_len, 5);
        assert_eq!(nums[..new_len], [0, 1, 2, 3, 4]);
    }
}
