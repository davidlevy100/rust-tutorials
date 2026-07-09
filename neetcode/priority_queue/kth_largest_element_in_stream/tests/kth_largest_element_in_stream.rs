use kth_largest_element_in_stream::KthLargest;

#[test]
fn examples() {
    let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
    assert_eq!(kth.add(3), 4);
    assert_eq!(kth.add(5), 5);
    assert_eq!(kth.add(10), 5);
    assert_eq!(kth.add(9), 8);
    assert_eq!(kth.add(4), 8);
}

#[test]
fn k_equals_one_tracks_the_max() {
    let mut kth = KthLargest::new(1, vec![]);
    assert_eq!(kth.add(-3), -3);
    assert_eq!(kth.add(-2), -2);
    assert_eq!(kth.add(-4), -2); // still the largest seen
    assert_eq!(kth.add(0), 0);
    assert_eq!(kth.add(4), 4);
}
