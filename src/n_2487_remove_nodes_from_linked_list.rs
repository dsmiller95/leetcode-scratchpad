use core::panic;

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

    fn from_vec(vals: Vec<i32>) -> Option<Box<Self>> {
        if vals.is_empty() {
            return None;
        }

        let mut nodes = vals.iter().map(|&x| Self::new(x)).collect::<Vec<_>>();
        for i in 0..(nodes.len() - 1) {
            let next = nodes[i + 1].clone();
            let mut current = nodes[i].clone();
            current.next = Some(Box::new(next));
            nodes[i] = current;
        }

        Some(Box::new(nodes[0].clone()))
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
}
struct Solution();

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
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
}
