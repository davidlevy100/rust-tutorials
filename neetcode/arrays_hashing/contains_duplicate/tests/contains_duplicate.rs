use contains_duplicate::contains_duplicate;

#[test]
fn examples() {
    assert!(contains_duplicate(vec![1, 2, 3, 1]));
    assert!(!contains_duplicate(vec![1, 2, 3, 4]));
    assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
}

#[test]
fn edge_cases() {
    assert!(!contains_duplicate(vec![]));       // nothing to duplicate
    assert!(!contains_duplicate(vec![7]));      // single element
    assert!(contains_duplicate(vec![-1, -1]));  // negatives count too
}
