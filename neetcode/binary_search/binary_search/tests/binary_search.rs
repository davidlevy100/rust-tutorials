use binary_search::search;

#[test]
fn examples() {
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1); // not present
}

#[test]
fn edge_cases() {
    assert_eq!(search(vec![5], 5), 0);       // single element, hit
    assert_eq!(search(vec![5], -5), -1);     // single element, miss
    assert_eq!(search(vec![2, 5], 2), 0);    // first element
    assert_eq!(search(vec![2, 5], 5), 1);    // last element
}
