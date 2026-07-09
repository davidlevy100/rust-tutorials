use last_stone_weight::last_stone_weight;

#[test]
fn examples() {
    assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
}

#[test]
fn edge_cases() {
    assert_eq!(last_stone_weight(vec![1]), 1);       // single stone
    assert_eq!(last_stone_weight(vec![2, 2]), 0);    // equal pair cancels out
    assert_eq!(last_stone_weight(vec![3, 7, 2]), 2);
    assert_eq!(last_stone_weight(vec![1, 1, 1]), 1);
}
