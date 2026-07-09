//! Linked List Cycle — LeetCode 141 (Easy)
//!
//! Return `true` if the list contains a cycle.
//!
//! NOTE: an `Option<Box<ListNode>>` can't actually represent a cycle (Box
//! ownership forms a tree, not a graph), so the test only covers the acyclic
//! case. Exercising a real cycle wants an `Rc<RefCell<..>>` or an index/arena
//! representation — a good stretch once the acyclic version passes.

#![allow(unused_variables)] // remove once you implement the body

/// LeetCode's singly-linked list node.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    todo!("implement has_cycle")
}
