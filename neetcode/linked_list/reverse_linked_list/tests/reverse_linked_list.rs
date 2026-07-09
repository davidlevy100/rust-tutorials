use reverse_linked_list::{reverse_list, ListNode};

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
    let out = reverse_list(list_from(&[1, 2, 3, 4, 5]));
    assert_eq!(list_to_vec(&out), vec![5, 4, 3, 2, 1]);

    let out = reverse_list(list_from(&[1, 2]));
    assert_eq!(list_to_vec(&out), vec![2, 1]);
}

#[test]
fn edge_cases() {
    assert_eq!(list_to_vec(&reverse_list(list_from(&[]))), vec![]); // empty
    assert_eq!(list_to_vec(&reverse_list(list_from(&[1]))), vec![1]); // single
}
