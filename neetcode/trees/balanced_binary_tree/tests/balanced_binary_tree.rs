use balanced_binary_tree::{Tree, TreeNode};
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

use balanced_binary_tree::is_balanced;

#[test]
fn examples() {
    let balanced =
        tree_from_level_order(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert!(is_balanced(balanced));

    let skewed = tree_from_level_order(&[
        Some(1), Some(2), Some(2), Some(3), Some(3), None, None, Some(4), Some(4),
    ]);
    assert!(!is_balanced(skewed));
}

#[test]
fn edge_cases() {
    assert!(is_balanced(None));                          // empty is balanced
    assert!(is_balanced(tree_from_level_order(&[Some(1)]))); // single node
    // A right-leaning chain of 3 is unbalanced at the root.
    assert!(!is_balanced(tree_from_level_order(&[
        Some(1), None, Some(2), None, Some(3),
    ])));
}
