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
        let Some(root) = root else {
            return None;
        };
        if depth == 1 {
            return Self::new_node(val, Some(root), None);
        }
        if depth == 2 {
            let new_root = {
                let root_ref = root.borrow();
                let new_root = TreeNode {
                    val: root_ref.val,
                    left: Self::new_node(val, root_ref.left.clone(), None),
                    right: Self::new_node(val, None, root_ref.right.clone()),
                };
                RefCell::new(new_root)
            };

            root.swap(&new_root);
            return Some(root);
        }
        {
            let root_ref = root.borrow();
            let _new_left = Self::add_one_row(root_ref.left.clone(), val, depth - 1);
            let _new_right = Self::add_one_row(root_ref.right.clone(), val, depth - 1);
        }
        // changes apply in-place, swapping out references underneath the ref cells, so we can throw out return value

        Some(root)
    }

    fn new_node(val: i32, left: TreeRef, right: TreeRef) -> TreeRef {
        let node = TreeNode { val, left, right };
        Option::Some(Rc::new(RefCell::new(node)))
    }
}
