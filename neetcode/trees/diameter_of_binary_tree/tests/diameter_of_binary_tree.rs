use diameter_of_binary_tree::{Tree, TreeNode};
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

use diameter_of_binary_tree::diameter_of_binary_tree;

#[test]
fn examples() {
    let root = tree_from_level_order(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
    assert_eq!(diameter_of_binary_tree(root), 3);

    let root = tree_from_level_order(&[Some(1), Some(2)]);
    assert_eq!(diameter_of_binary_tree(root), 1);
}

#[test]
fn edge_cases() {
    assert_eq!(diameter_of_binary_tree(None), 0);                          // empty
    assert_eq!(diameter_of_binary_tree(tree_from_level_order(&[Some(1)])), 0); // single node
}
