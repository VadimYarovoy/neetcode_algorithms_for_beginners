use std::{mem, ptr};

struct MyLinkedList {
    head: *mut MyNode,
    tail: *mut MyNode,
    len: i32,
}

impl Default for MyLinkedList {
    fn default() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }
}

impl Drop for MyLinkedList {
    fn drop(&mut self) {
        while self.len > 0 {
            self.delete_at_index(0);
        }
    }
}

struct MyNode {
    next: *mut MyNode,
    prev: *mut MyNode,
    val: i32,
}

impl MyNode {
    fn new(val: i32) -> Self {
        Self {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
            val,
        }
    }
}

impl MyLinkedList {
    fn new() -> Self {
        Self::default()
    }

    fn get(&self, index: i32) -> i32 {
        if index >= self.len {
            return -1;
        }

        let mut target = self.head;

        unsafe {
            for _ in 0..index {
                target = (*target).next;
            }

            (*target).val
        }
    }

    fn add_at_head(&mut self, val: i32) {
        let new_head = Box::into_raw(Box::new(MyNode::new(val)));
        let old_head = self.head;

        unsafe {
            if self.head.is_null() {
                self.head = new_head;
                self.tail = new_head;
            } else {
                (*old_head).prev = new_head;
                (*new_head).next = old_head;
                self.head = new_head;
            }
        }

        self.len += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_tail = Box::into_raw(Box::new(MyNode::new(val)));
        let old_tail = self.tail;

        unsafe {
            if self.tail.is_null() {
                self.head = new_tail;
                self.tail = new_tail;
            } else {
                (*old_tail).next = new_tail;
                (*new_tail).prev = old_tail;
                self.tail = new_tail;
            }
        }

        self.len += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        match index {
            0 => {
                self.add_at_head(val);
            }
            index if index > self.len => {}
            index if index == self.len => {
                self.add_at_tail(val);
            }
            _ => {
                let new_mid = Box::into_raw(Box::new(MyNode::new(val)));
                let mut after = self.head;

                unsafe {
                    for _ in 0..index {
                        after = (*after).next;

                        let prev = (*after).prev;
                        (*prev).next = new_mid;
                        (*new_mid).prev = prev;
                        (*new_mid).next = after;
                        (*after).prev = new_mid;
                    }
                }

                self.len += 1;
            }
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        match (index, self.len) {
            (index, len) if index >= len => {}
            (0, 1) => {
                let target = self.head;
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();

                unsafe { mem::drop(Box::from_raw(target)) }

                self.len -= 1;
            }
            (index, len) if index == len - 1 => unsafe {
                let target = self.tail;
                let prev = (*target).prev;
                self.tail = prev;
                (*prev).next = ptr::null_mut();

                mem::drop(Box::from_raw(target));

                self.len -= 1;
            },
            (0, _) => unsafe {
                let target = self.head;
                let after = (*target).next;
                self.head = after;
                (*after).prev = ptr::null_mut();

                mem::drop(Box::from_raw(target));

                self.len -= 1;
            },
            (index, _) => {
                let mut target = self.head;

                unsafe {
                    for _ in 0..index {
                        target = (*target).next;
                    }

                    let prev = (*target).prev;
                    let after = (*target).next;

                    (*prev).next = after;
                    (*after).prev = prev;

                    mem::drop(Box::from_raw(target));

                    self.len -= 1;
                }
            }
        }
    }
}
