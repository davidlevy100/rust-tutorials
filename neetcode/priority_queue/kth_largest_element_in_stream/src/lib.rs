//! Kth Largest Element in a Stream — LeetCode 703 (Easy)
//!
//! A design problem: keep answering "what's the kth largest so far?" as values
//! arrive. Choose your own fields.
//!
//! Tip: a min-heap of size k works well — `BinaryHeap` is a max-heap, so wrap
//! values in `std::cmp::Reverse` to invert it.

#![allow(unused_variables)] // remove once you implement the bodies

pub struct KthLargest {
    // your fields here
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        todo!("implement KthLargest::new")
    }

    /// Add `val` to the stream and return the current kth largest element.
    pub fn add(&mut self, val: i32) -> i32 {
        todo!("implement KthLargest::add")
    }
}
