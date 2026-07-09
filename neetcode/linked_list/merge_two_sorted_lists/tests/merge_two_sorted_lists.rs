use merge_two_sorted_lists::{merge_two_lists, ListNode};

fn list_from(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

fn list_to_vec(mut head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(node) = head {
        out.push(node.val);
        head = &node.next;
    }
    out
}

#[test]
fn examples() {
    let out = merge_two_lists(list_from(&[1, 2, 4]), list_from(&[1, 3, 4]));
    assert_eq!(list_to_vec(&out), vec![1, 1, 2, 3, 4, 4]);
}

#[test]
fn edge_cases() {
    // both empty
    assert_eq!(list_to_vec(&merge_two_lists(list_from(&[]), list_from(&[]))), vec![]);
    // one empty
    assert_eq!(
        list_to_vec(&merge_two_lists(list_from(&[]), list_from(&[0]))),
        vec![0]
    );
    // interleaving of different lengths
    assert_eq!(
        list_to_vec(&merge_two_lists(list_from(&[1, 3]), list_from(&[2]))),
        vec![1, 2, 3]
    );
}
