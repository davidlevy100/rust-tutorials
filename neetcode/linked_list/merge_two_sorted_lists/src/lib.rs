//! Merge Two Sorted Lists — LeetCode 21 (Easy)
//!
//! Splice two sorted lists into one sorted list.

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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    todo!("implement merge_two_lists")
}
