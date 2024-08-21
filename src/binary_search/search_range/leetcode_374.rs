/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

#[allow(dead_code)]
#[allow(non_snake_case)]
unsafe fn guessNumber(n: i32) -> i32 {
    let (mut l, mut r) = (1, n);

    while l <= r {
        let mid = l + (r - l) / 2;
        match guess(mid) {
            0 => {
                return mid;
            }
            -1 => {
                r = mid - 1;
            }
            1 => {
                l = mid + 1;
            }
            _ => unreachable!(),
        }
    }

    unreachable!()
}

fn guess(_num: i32) -> i32 {
    unimplemented!()
}
