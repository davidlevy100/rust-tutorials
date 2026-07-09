use single_number::single_number;

#[test]
fn examples() {
    assert_eq!(single_number(vec![2, 2, 1]), 1);
    assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
}

#[test]
fn edge_cases() {
    assert_eq!(single_number(vec![1]), 1);           // lone element
    assert_eq!(single_number(vec![0, 1, 0]), 1);     // zero appears twice
    assert_eq!(single_number(vec![-1, -1, -2]), -2); // negatives
}
