use linked_list_cycle::{has_cycle, ListNode};

fn list_from(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

// NOTE: an `Option<Box<ListNode>>` can't hold a cycle, so only acyclic inputs
// are testable here. The `true` case needs an `Rc<RefCell>`/arena list (see the
// note in src/lib.rs).
#[test]
fn acyclic_lists_have_no_cycle() {
    assert!(!has_cycle(list_from(&[1, 2, 3, 4])));
    assert!(!has_cycle(list_from(&[1])));
    assert!(!has_cycle(None));
}
