#![allow(unused_variables)] // remove once you implement the body

use std::cell::RefCell;
use std::rc::Rc;

/// LeetCode's binary tree node.
#[derive(PartialEq, Eq, Clone, Debug)]
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

/// The type LeetCode's tree functions take/return.
pub type Tree = Option<Rc<RefCell<TreeNode>>>;

/// Maximum Depth of Binary Tree — LeetCode 104 (Easy)
pub fn max_depth(root: Tree) -> i32 {
    todo!("implement max_depth")
}
