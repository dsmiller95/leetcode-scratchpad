// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution();

impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut rev_head = None;

        while let Some(mut node) = head {
            let next = node.next.take();
            node.next = rev_head;

            rev_head = Some(node);
            head = next;
        }

        let mut carry = 0;

        (head, rev_head) = (rev_head, head);

        while let Some(mut node) = head {
            let next = node.next.take();
            node.next = rev_head;

            let full_val = node.val * 2 + carry;
            carry = full_val / 10;
            node.val = full_val % 10;

            rev_head = Some(node);
            head = next;
        }
        (head, rev_head) = (rev_head, head);

        if carry != 0 {
            let new_node = ListNode {
                next: head,
                val: carry,
            };
            head = Some(Box::new(new_node));
        }

        head
    }
}
