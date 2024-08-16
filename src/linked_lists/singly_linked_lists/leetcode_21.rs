// Merge Two Sorted Lists

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
pub fn _merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (Some(l1), Some(l2)) => {
            if l1.val < l2.val {
                return Some(Box::new(ListNode {
                    val: l1.val,
                    next: _merge_two_lists(l1.next, Some(l2)),
                }));
            }
            return Some(Box::new(ListNode {
                val: l2.val,
                next: _merge_two_lists(Some(l1), l2.next),
            }));
        }
    }
}

#[cfg(test)]
mod reverse_list_tests {
    use super::{ListNode, _merge_two_lists};

    #[test]
    fn merge_liss() {
        let list_1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        }));
        let list_2 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        }));

        let res = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));

        assert_eq!(_merge_two_lists(list_1, list_2), res);
    }
}
