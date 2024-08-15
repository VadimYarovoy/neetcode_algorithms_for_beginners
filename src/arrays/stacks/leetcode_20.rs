// Valid Parentheses
fn _is_valid(s: String) -> bool {
    let mut stack = vec![];

    for ch in s.chars() {
        if stack.is_empty() {
            stack.push(ch);
        } else {
            match ch {
                ']' if *stack.last().unwrap() == '[' => {
                    stack.pop();
                }
                '}' if *stack.last().unwrap() == '{' => {
                    stack.pop();
                }
                ')' if *stack.last().unwrap() == '(' => {
                    stack.pop();
                }
                _ => {
                    stack.push(ch);
                }
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod brackets_tests {
    use super::_is_valid;

    #[test]
    fn valid_string() {
        let s = "()[]{}".to_string();
        assert_eq!(true, _is_valid(s));
    }

    #[test]
    fn invalid_string() {
        let s = "()]{}".to_string();
        assert_ne!(true, _is_valid(s));
    }
}
