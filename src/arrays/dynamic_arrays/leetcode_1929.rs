// Concatenation of Array
fn _get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    nums.repeat(2)
}

#[cfg(test)]
mod concatenation_tests {
    use super::_get_concatenation;

    #[test]
    fn concat_array() {
        let nums = vec![1, 2, 1];
        assert_eq!(vec![1, 2, 1, 1, 2, 1], _get_concatenation(nums));
    }
}
