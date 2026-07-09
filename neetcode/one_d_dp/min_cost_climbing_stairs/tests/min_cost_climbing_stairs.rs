use min_cost_climbing_stairs::min_cost_climbing_stairs;

#[test]
fn examples() {
    assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    assert_eq!(
        min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}

#[test]
fn edge_cases() {
    assert_eq!(min_cost_climbing_stairs(vec![0, 0]), 0);   // free
    assert_eq!(min_cost_climbing_stairs(vec![10, 15]), 10); // start at 0, step to top
}
