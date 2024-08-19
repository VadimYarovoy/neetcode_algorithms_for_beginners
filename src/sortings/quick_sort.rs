#![allow(dead_code)]
fn quick_sort(arr: &mut [i32], s: isize, e: isize) {
    if s < e && e - s >= 1 {
        let pivot = partition(arr, s as isize, e as isize);
        quick_sort(arr, s, pivot - 1);
        quick_sort(arr, pivot + 1, e);
    }
}

fn partition(arr: &mut [i32], l: isize, h: isize) -> isize {
    let pivot = arr[h as usize];
    let mut i = l - 1;

    for j in l..h {
        if arr[j as usize] <= pivot {
            i = i + 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, h as usize);

    i + 1
}

#[cfg(test)]
mod quick_sort_tests {
    use super::quick_sort;

    #[test]
    fn sort_array() {
        let mut arr = [1, 4, 2, 5, 3];
        let mut arr_sorted = arr.clone();
        arr_sorted.sort_unstable();
        let e = arr.len() as isize - 1;
        quick_sort(&mut arr, 0, e);
        assert_eq!(arr_sorted, arr);
    }
}
