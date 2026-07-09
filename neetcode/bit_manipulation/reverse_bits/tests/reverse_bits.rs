use reverse_bits::reverse_bits;

#[test]
fn examples() {
    assert_eq!(reverse_bits(43261596), 964176192);
    assert_eq!(reverse_bits(4294967293), 3221225471); // LeetCode example 2
}

#[test]
fn edge_cases() {
    assert_eq!(reverse_bits(0), 0);
    assert_eq!(reverse_bits(1), 1 << 31); // LSB reversed becomes the MSB
    assert_eq!(reverse_bits(u32::MAX), u32::MAX);
}
