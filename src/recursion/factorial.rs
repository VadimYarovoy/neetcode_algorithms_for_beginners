#![allow(dead_code)]
fn factorial_recursion(num: i32) -> i32 {
    match num {
        1 => {
            return 1;
        }
        n => {
            return n * factorial_recursion(n - 1);
        }
    }
}

// Better
fn factorial_cycle(mut num: i32) -> i32 {
    let mut fact = 1;
    while num > 0 {
        fact *= num;
        num -= 1;
    }

    fact
}

#[cfg(test)]
mod factorial_tests {
    use super::{factorial_cycle, factorial_recursion};

    #[test]
    fn calculate_factorial() {
        assert_eq!(factorial_recursion(3), 6);
        assert_eq!(factorial_recursion(4), 24);

        assert_eq!(factorial_cycle(3), 6);
        assert_eq!(factorial_cycle(4), 24);
    }
}
