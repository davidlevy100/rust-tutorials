use same_tree::{Tree, TreeNode};
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

use same_tree::is_same_tree;

#[test]
fn examples() {
    let a = tree_from_level_order(&[Some(1), Some(2), Some(3)]);
    let b = tree_from_level_order(&[Some(1), Some(2), Some(3)]);
    assert!(is_same_tree(a, b));

    let c = tree_from_level_order(&[Some(1), Some(2)]);
    let d = tree_from_level_order(&[Some(1), None, Some(2)]);
    assert!(!is_same_tree(c, d)); // same values, different shape
}

#[test]
fn edge_cases() {
    assert!(is_same_tree(None, None)); // both empty
    assert!(!is_same_tree(tree_from_level_order(&[Some(1)]), None)); // one empty
    let e = tree_from_level_order(&[Some(1), Some(2), Some(1)]);
    let f = tree_from_level_order(&[Some(1), Some(1), Some(2)]);
    assert!(!is_same_tree(e, f)); // same shape, different values
}
