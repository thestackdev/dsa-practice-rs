#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut current = head.as_mut();
        let mut current_index = 0;

        while current.is_some() && current_index < n {
            current = current.unwrap().next.as_mut();
            current_index += 1;
        }

        if let Some(node) = current {
            if let Some(next) = node.next.as_mut() {
                node.next = next.next.take();
            }
        }

        head
    }
}
