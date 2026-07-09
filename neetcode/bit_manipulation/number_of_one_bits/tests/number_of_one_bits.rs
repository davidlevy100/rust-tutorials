use number_of_one_bits::hamming_weight;

#[test]
fn examples() {
    assert_eq!(hamming_weight(11), 3);   // 1011
    assert_eq!(hamming_weight(128), 1);  // 1000_0000
}

#[test]
fn edge_cases() {
    assert_eq!(hamming_weight(0), 0);            // no bits set
    assert_eq!(hamming_weight(u32::MAX), 32);    // all bits set
    assert_eq!(hamming_weight(1 << 31), 1);      // only the top bit
}
