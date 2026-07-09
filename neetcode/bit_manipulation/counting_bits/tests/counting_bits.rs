use counting_bits::count_bits;

#[test]
fn examples() {
    assert_eq!(count_bits(2), vec![0, 1, 1]);
    assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}

#[test]
fn edge_cases() {
    assert_eq!(count_bits(0), vec![0]); // just i = 0
    assert_eq!(count_bits(1), vec![0, 1]);
}
