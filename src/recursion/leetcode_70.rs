#[allow(dead_code)]
pub fn climb_stairs(n: i32) -> i32 {
    if n < 3 {
        return n;
    }

    let mut res = 0;
    let (mut prev, mut curr) = (1, 1);

    for _ in 2..n + 1 {
        res = prev + curr;
        prev = curr;
        curr = res;
    }

    res
}

#[cfg(test)]
mod climb_stairs_tests {
    use super::climb_stairs;

    #[test]
    fn basic_cases() {
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(2), 2);
    }
}
