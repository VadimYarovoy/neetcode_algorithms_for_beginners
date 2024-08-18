#[allow(dead_code)]
fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i - 1;
        while j > 0 && arr[j + 1] < arr[j] {
            let temp = arr[j + 1];
            arr[j + 1] = arr[j];
            arr[j] = temp;
            j -= 1;
        }
    }
}

#[cfg(test)]
mod insertion_sort_tests {
    use super::insertion_sort;

    #[test]
    fn sort_array() {
        let mut arr = [1, 4, 2, 5, 3];
        let mut arr_sorted = arr.clone();
        arr_sorted.sort_unstable();
        insertion_sort(&mut arr);
        assert_eq!(arr_sorted, arr);
    }
}
