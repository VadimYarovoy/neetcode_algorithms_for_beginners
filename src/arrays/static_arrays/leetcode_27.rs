// Remove Duplicates from Sorted Array
// Time O(n), Space O(1)
fn _remove_element(nums: &mut [i32], val: i32) -> i32 {
    let mut l = 0;

    for r in 0..nums.len() {
        if nums[r] != val {
            nums[l] = nums[r];
            l += 1;
        }
    }

    l as i32
}

#[cfg(test)]
mod static_array_tests {
    use super::_remove_element;

    #[test]
    fn simple_case() {
        let mut nums = [3,2,2,3];
        let new_len = _remove_element(&mut nums, 3) as usize;
        assert_eq!(new_len, 2);
        assert_eq!(nums[..new_len], [2, 2]);
    }
}