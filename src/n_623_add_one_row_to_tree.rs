// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

type TreeRef = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn add_one_row(root: TreeRef, val: i32, depth: i32) -> TreeRef {
        if depth == 1 {
            return Self::new_node(val, root, None);
        }
        Self::add_one_row_in_place(root.clone(), val, depth);
        root
    }
    fn add_one_row_in_place(root: TreeRef, val: i32, depth: i32) {
        let Some(root) = root else {
            return;
        };
        match depth {
            ..=1 => panic!(),
            2 => {
                let mut root = root.borrow_mut();

                root.left = Self::new_node(val, root.left.take(), None);
                root.right = Self::new_node(val, None, root.right.take());
            }
            3.. => {
                let root_ref = root.borrow();
                Self::add_one_row_in_place(root_ref.left.clone(), val, depth - 1);
                Self::add_one_row_in_place(root_ref.right.clone(), val, depth - 1);
            }
        }
    }

    fn new_node(val: i32, left: TreeRef, right: TreeRef) -> TreeRef {
        let node = TreeNode { val, left, right };
        Option::Some(Rc::new(RefCell::new(node)))
    }
}
