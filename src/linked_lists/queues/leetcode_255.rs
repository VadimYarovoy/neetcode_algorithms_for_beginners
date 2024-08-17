#![allow(dead_code)]
use std::collections::VecDeque;

struct MyStack {
    first: VecDeque<i32>,
    second: VecDeque<i32>,
    is_first: bool,
}

impl MyStack {
    fn new() -> Self {
        Self {
            first: Default::default(),
            second: Default::default(),
            is_first: true,
        }
    }

    fn push(&mut self, x: i32) {
        if self.is_first {
            self.first.push_back(x);
        } else {
            self.second.push_back(x);
        }
    }

    fn pop(&mut self) -> i32 {
        if self.is_first {
            let mut elem = None;

            for (idx, &el) in self.first.iter().enumerate() {
                if idx == self.first.len() - 1 {
                    elem = Some(el);
                    break;
                }
                self.second.push_back(el);
            }

            self.first.clear();

            self.is_first = false;
            return elem.unwrap();
        } else {
            let mut elem = None;

            for (idx, &el) in self.second.iter().enumerate() {
                if idx == self.second.len() - 1 {
                    elem = Some(el);
                    break;
                }
                self.first.push_back(el);
            }

            self.second.clear();

            self.is_first = true;
            return elem.unwrap();
        }
    }

    fn top(&mut self) -> i32 {
        if self.is_first {
            let mut elem = None;

            for (idx, &el) in self.first.iter().enumerate() {
                if idx == self.first.len() - 1 {
                    elem = Some(el);
                }
                self.second.push_back(el);
            }

            self.first.clear();

            self.is_first = false;
            return elem.unwrap();
        } else {
            let mut elem = None;

            for (idx, &el) in self.second.iter().enumerate() {
                if idx == self.second.len() - 1 {
                    elem = Some(el);
                }
                self.first.push_back(el);
            }

            self.second.clear();

            self.is_first = true;
            return elem.unwrap();
        }
    }

    fn empty(&self) -> bool {
        self.first.is_empty() && self.second.is_empty()
    }
}
