// Min Stack
struct _MinStack {
    storage: Vec<i32>,
}

impl _MinStack {
    fn _new() -> Self {
        Self {
            storage: Default::default(),
        }
    }

    fn _push(&mut self, val: i32) {
        self.storage.push(val);
    }

    fn _pop(&mut self) {
        self.storage.pop();
    }

    fn _top(&self) -> i32 {
        *self.storage.last().unwrap()
    }

    fn _get_min(&self) -> i32 {
        *self.storage.iter().min().unwrap()
    }
}

#[cfg(test)]
mod min_stack_test {
    use super::_MinStack;

    #[test]
    fn simple_case() {
        let mut min_stack = _MinStack::_new();
        min_stack._push(1);
        min_stack._push(2);

        assert_eq!(min_stack._get_min(), 1);

        min_stack._pop();

        assert_eq!(min_stack._top(), 1);
    }
}
