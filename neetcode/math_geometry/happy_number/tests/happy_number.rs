use happy_number::is_happy;

#[test]
fn examples() {
    assert!(is_happy(19));
    assert!(!is_happy(2)); // falls into a cycle
}

#[test]
fn edge_cases() {
    assert!(is_happy(1));   // already 1
    assert!(is_happy(7));
    assert!(!is_happy(4));  // the classic unhappy cycle
}
