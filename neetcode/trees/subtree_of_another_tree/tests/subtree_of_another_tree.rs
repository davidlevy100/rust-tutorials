use subtree_of_another_tree::{Tree, TreeNode};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Build a tree from a level-order slice, `None` marking a missing child.
fn tree_from_level_order(vals: &[Option<i32>]) -> Tree {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;
    while i < vals.len() {
        let node = match queue.pop_front() {
            Some(n) => n,
            None => break,
        };
        if i < vals.len() {
            if let Some(v) = vals[i] {
                let child = Rc::new(RefCell::new(TreeNode::new(v)));
                node.borrow_mut().left = Some(Rc::clone(&child));
                queue.push_back(child);
            }
            i += 1;
        }
        if i < vals.len() {
            if let Some(v) = vals[i] {
                let child = Rc::new(RefCell::new(TreeNode::new(v)));
                node.borrow_mut().right = Some(Rc::clone(&child));
                queue.push_back(child);
            }
            i += 1;
        }
    }
    Some(root)
}

use subtree_of_another_tree::is_subtree;

#[test]
fn examples() {
    let root = tree_from_level_order(&[Some(3), Some(4), Some(5), Some(1), Some(2)]);
    let sub = tree_from_level_order(&[Some(4), Some(1), Some(2)]);
    assert!(is_subtree(root, sub));

    let root2 = tree_from_level_order(&[
        Some(3), Some(4), Some(5), Some(1), Some(2), None, None, None, None, Some(0),
    ]);
    let sub2 = tree_from_level_order(&[Some(4), Some(1), Some(2)]);
    assert!(!is_subtree(root2, sub2)); // must match a whole subtree, not part of one
}

#[test]
fn edge_cases() {
    // A tree is a subtree of itself.
    let a = tree_from_level_order(&[Some(1), Some(2), Some(3)]);
    let b = tree_from_level_order(&[Some(1), Some(2), Some(3)]);
    assert!(is_subtree(a, b));

    // Single-node match deep in the tree.
    let root = tree_from_level_order(&[Some(1), Some(1)]);
    let leaf = tree_from_level_order(&[Some(1)]);
    assert!(is_subtree(root, leaf));
}
