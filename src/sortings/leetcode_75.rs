#[allow(dead_code)]
pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut counter = [0; 3];
    for &n in nums.iter() {
        counter[n as usize] += 1;
    }

    let mut curr_idx = 0;
    for i in 0..3 {
        for _ in 0..counter[i] {
            nums[curr_idx] = i as i32;
            curr_idx += 1;
        }
    }
}

#[cfg(test)]
mod sort_colors_tests {
    use super::sort_colors;

    #[test]
    fn sort_some_colors() {
        let mut colors = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut colors);
        assert_eq!(colors, vec![0, 0, 1, 1, 2, 2]);
    }
}
