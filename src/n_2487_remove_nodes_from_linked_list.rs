use core::panic;
use std::borrow::{Borrow, BorrowMut};

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

impl ListNode {
    fn reverse(node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vec_vals = Self::to_vec(node);
        vec_vals.reverse();
        Self::from_vec(vec_vals)
    }

    fn from_vec(vals: Vec<i32>) -> Option<Box<Self>> {
        if vals.is_empty() {
            return None;
        }

        let mut nodes = vals
            .iter()
            .map(|&x| Some(Box::new(Self::new(x))))
            .collect::<Vec<_>>();

        for i in (1..(vals.len())).rev() {
            let next_node = nodes[i].take().unwrap();
            let prev_nod = nodes[i - 1].borrow_mut().as_mut().unwrap();
            prev_nod.next = Some(next_node);
        }

        nodes[0].clone()
    }

    fn to_vec(node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut next = node;
        while next.is_some() {
            let next_v = next.unwrap();
            result.push(next_v.val);
            next = next_v.next;
        }
        result
    }

    fn next_val(node: &Option<Box<ListNode>>) -> Option<i32> {
        Some(node.as_ref()?.next.as_ref()?.val)
    }
}

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed = ListNode::reverse(head);
        let mut max = reversed.as_ref().map(|x| x.val).unwrap_or(i32::MIN);
        let mut current_node = &mut reversed;

        while current_node.is_some() {
            let Some(next_val) = ListNode::next_val(current_node) else {
                break;
            };

            max = max.max(next_val);
            if next_val < max {
                let cur_val = current_node.as_mut().unwrap();
                let old_next = cur_val.next.take();
                let new_next = old_next.and_then(|x| x.next);
                cur_val.next = new_next;
            } else {
                let cur_val = current_node.as_mut().unwrap();
                current_node = &mut cur_val.next;
            }
        }

        ListNode::reverse(reversed)
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_1() {
        let list = [5, 2, 13, 3, 8];
        let expected = [13, 8];

        let list = ListNode::from_vec(list.to_vec());
        let expected = expected.to_vec();
        let result = Solution::remove_nodes(list);
        let result_vec = ListNode::to_vec(result);
        assert_eq!(expected, result_vec);
    }

    #[test]
    fn test_2() {
        let list = [1, 1, 1, 1];
        let expected = [1, 1, 1, 1];

        let list = ListNode::from_vec(list.to_vec());
        let expected = expected.to_vec();
        let result = Solution::remove_nodes(list);
        let result_vec = ListNode::to_vec(result);
        assert_eq!(expected, result_vec);
    }

    #[test]
    fn test_reverse() {
        let list = [2, 4, 56, 9, 9, 2];
        let expected = [2, 9, 9, 56, 4, 2];

        let list = ListNode::from_vec(list.to_vec());
        let expected = expected.to_vec();
        let result = ListNode::reverse(list);
        let result_vec = ListNode::to_vec(result);
        assert_eq!(expected, result_vec);
    }

    #[test]
    fn test_to_vec_round_trip() {
        let list = [2, 4, 56, 9, 9, 2];

        let list_head = ListNode::from_vec(list.to_vec());
        let expected = list.to_vec();
        let result = ListNode::to_vec(list_head);
        assert_eq!(expected, result);
    }
}
