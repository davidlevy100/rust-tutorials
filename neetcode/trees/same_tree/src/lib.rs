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

/// Same Tree — LeetCode 100 (Easy)
pub fn is_same_tree(p: Tree, q: Tree) -> bool {
    todo!("implement is_same_tree")
}
