use missing_number::missing_number;

#[test]
fn examples() {
    assert_eq!(missing_number(vec![3, 0, 1]), 2);
    assert_eq!(missing_number(vec![0, 1]), 2); // missing the top of the range
    assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
}

#[test]
fn edge_cases() {
    assert_eq!(missing_number(vec![0]), 1);  // range 0..=1, only 0 present
    assert_eq!(missing_number(vec![1]), 0);  // 0 itself is missing
}
