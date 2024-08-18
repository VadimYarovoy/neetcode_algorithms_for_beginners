#[allow(dead_code)]
fn fibonacci_recursion(n: i32) -> i32 {
    match n {
        1 => {
            return 1;
        }
        0 => {
            return 0;
        }
        _ => {
            return fibonacci_recursion(n - 1) + fibonacci_recursion(n - 2);
        }
    }
}

#[allow(dead_code)]
fn fibonacci_cycle(n: i32) -> i32 {
    match n {
        0 => {
            return 0;
        }
        1 => {
            return 1;
        }
        _ => {}
    }

    let (mut num1, mut num2) = (0, 1);

    for _ in 0..n - 1 {
        let temp = num1 + num2;
        num1 = num2;
        num2 = temp;
    }

    num2
}

#[cfg(test)]
mod fibonacci_tests {
    use super::{fibonacci_cycle, fibonacci_recursion};

    #[test]
    fn small_fibonacci_numbers_with_recursion() {
        assert_eq!(fibonacci_recursion(2), 1);
        assert_eq!(fibonacci_recursion(4), 3);
        assert_eq!(fibonacci_recursion(8), 21);
    }

    #[test]
    fn small_fibonacci_numbers_with_cycle() {
        assert_eq!(fibonacci_cycle(2), 1);
        assert_eq!(fibonacci_cycle(4), 3);
        assert_eq!(fibonacci_cycle(8), 21);
    }
}
