use two_sum::two_sum;

// LeetCode accepts the two indices in either order, so compare order-insensitively.
fn sorted(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
}

#[test]
fn examples() {
    assert_eq!(sorted(two_sum(vec![2, 7, 11, 15], 9)), vec![0, 1]);
    assert_eq!(sorted(two_sum(vec![3, 2, 4], 6)), vec![1, 2]);
}

#[test]
fn edge_cases() {
    assert_eq!(sorted(two_sum(vec![3, 3], 6)), vec![0, 1]); // duplicate values
    assert_eq!(sorted(two_sum(vec![-3, 4, 3, 90], 0)), vec![0, 2]); // negatives
}
