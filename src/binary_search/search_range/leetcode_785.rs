#![allow(dead_code)]
use std::cmp::Ordering::{Equal, Greater, Less};

fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
    piles.sort_unstable();

    let (mut l, mut r) = (0, piles[piles.len() - 1]);
    let mut min_speed = i32::MAX;

    while l <= r {
        let k = l + (r - l) / 2;
        let mid_h = check(&piles, k);
        dbg!(mid_h, k);

        match mid_h.cmp(&h) {
            Greater => {
                l = k + 1;
            }
            Less | Equal => {
                min_speed = min_speed.min(k);
                r = k - 1;
            }
        }
    }

    min_speed
}

fn check(piles: &Vec<i32>, k: i32) -> i32 {
    let mut total = 0;

    for &p in piles {
        let val = (p as f64 / k as f64).ceil() as i32;
        total += val;
    }

    total
}

#[cfg(test)]
mod min_eating_speed_tests {
    use super::{check, min_eating_speed};

    #[test]
    fn simple_case_1() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;

        assert_eq!(min_eating_speed(piles, h), 4);
    }

    #[test]
    fn simple_case_2() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;

        assert_eq!(min_eating_speed(piles, h), 30);
    }

    #[test]
    fn simple_case_3() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;

        assert_eq!(min_eating_speed(piles, h), 23);
    }

    #[test]
    fn simple_case_4() {
        let piles = vec![805306368, 805306368, 805306368];
        let h = 1000000000;

        assert_eq!(min_eating_speed(piles, h), 3);
    }

    #[test]
    fn test_check() {
        let piles = vec![3, 6, 7, 11];
        let k = 5;

        assert_eq!(check(&piles, k), 8);
    }
}
