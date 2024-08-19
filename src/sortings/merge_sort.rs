#![allow(dead_code)]
pub fn mergesort(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    mergesort(&mut arr[..mid]);
    mergesort(&mut arr[mid..]);

    let mut ret = arr.to_vec();

    merge(&arr[..mid], &arr[mid..], &mut ret[..]);

    arr.copy_from_slice(&ret);
}

fn merge(arr1: &[i32], arr2: &[i32], ret: &mut [i32]) {
    let mut l = 0;
    let mut r = 0;
    let mut idx = 0;

    while l < arr1.len() && r < arr2.len() {
        if arr1[l] <= arr2[r] {
            ret[idx] = arr1[l];
            idx += 1;
            l += 1;
        } else {
            ret[idx] = arr2[r];
            idx += 1;
            r += 1;
        }
    }

    if l < arr1.len() {
        ret[idx..].copy_from_slice(&arr1[l..]);
    }
    if r < arr2.len() {
        ret[idx..].copy_from_slice(&arr2[r..]);
    }
}

#[cfg(test)]
mod mergesort_sort_tests {
    use super::mergesort;

    #[test]
    fn sort_array() {
        let mut arr = [1, 4, 2, 5, 3];
        let mut arr_sorted = arr.clone();
        arr_sorted.sort_unstable();
        mergesort(&mut arr);
        assert_eq!(arr_sorted, arr);
    }
}
