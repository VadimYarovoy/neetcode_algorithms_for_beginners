// Reverse Linked List

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn _new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn _reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut curr) = (None, head);

    while let Some(mut node) = curr {
        curr = node.next;
        node.next = prev;
        prev = Some(node);
    }

    prev
}
