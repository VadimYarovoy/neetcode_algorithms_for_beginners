// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut l, mut r) = (1, n);

        while l <= r {
            let mid = l + (r - l) / 2;

            match (self.isBadVersion(mid - 1), self.isBadVersion(mid)) {
                (false, true) => {
                    return mid;
                }
                (true, true) => {
                    r = mid - 1;
                }
                (true, false) => {
                    unreachable!()
                }
                (false, false) => {
                    l = mid + 1;
                }
            }
        }

        unreachable!()
    }

    #[allow(non_snake_case)]
    fn isBadVersion(&self, _version: i32) -> bool {
        unimplemented!()
    }
}
